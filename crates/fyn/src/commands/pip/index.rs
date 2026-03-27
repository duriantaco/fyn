use std::collections::BTreeSet;
use std::fmt::Write;

use anyhow::Result;
use owo_colors::OwoColorize;
use rkyv::deserialize;
use tokio::sync::Semaphore;
use tracing::debug;

use fyn_cache::Cache;
use fyn_client::{
    BaseClientBuilder, FlatIndexClient, MetadataFormat, RegistryClient, RegistryClientBuilder,
    VersionFiles,
};
use fyn_configuration::{Concurrency, IndexStrategy, KeyringProviderType, TargetTriple};
use fyn_distribution_filename::DistFilename;
use fyn_distribution_types::{File, Index, IndexCapabilities, IndexLocations, RequiresPython};
use fyn_normalize::PackageName;
use fyn_pep440::Version;
use fyn_platform_tags::Tags;
use fyn_preview::Preview;
use fyn_python::{
    EnvironmentPreference, PythonEnvironment, PythonPreference, PythonRequest, PythonVersion,
};
use fyn_resolver::{ExcludeNewer, PythonRequirement};
use fyn_warnings::warn_user_once;

use crate::commands::ExitStatus;
use crate::commands::pip::resolution_tags;
use crate::printer::Printer;

pub(crate) async fn pip_index_versions(
    package: PackageName,
    index_locations: IndexLocations,
    index_strategy: IndexStrategy,
    keyring_provider: KeyringProviderType,
    exclude_newer: ExcludeNewer,
    python: Option<&str>,
    system: bool,
    python_version: Option<PythonVersion>,
    python_platform: Option<TargetTriple>,
    client_builder: &BaseClientBuilder<'_>,
    concurrency: Concurrency,
    cache: &Cache,
    printer: Printer,
    preview: Preview,
) -> Result<ExitStatus> {
    let environment = PythonEnvironment::find(
        &python.map(PythonRequest::parse).unwrap_or_default(),
        EnvironmentPreference::from_system_flag(system, false),
        PythonPreference::default().with_system_flag(system),
        cache,
        preview,
    )?;

    let interpreter = environment.interpreter();
    let tags = resolution_tags(
        python_version.as_ref(),
        python_platform.as_ref(),
        interpreter,
    )?;
    let python_requirement = if let Some(python_version) = python_version.as_ref() {
        PythonRequirement::from_python_version(interpreter, python_version)
    } else {
        PythonRequirement::from_interpreter(interpreter)
    };

    let flat_only = index_locations.no_index() && index_locations.flat_indexes().next().is_some();

    let client = RegistryClientBuilder::new(
        client_builder.clone().keyring(keyring_provider),
        cache.clone(),
    )
    .index_locations(index_locations.clone())
    .index_strategy(index_strategy)
    .markers(interpreter.markers())
    .platform(interpreter.platform())
    .build();

    let lookup = if flat_only {
        fetch_flat_index_versions(
            &client,
            &package,
            index_locations.flat_indexes(),
            cache,
            &exclude_newer,
            python_requirement.target(),
            Some(tags.as_ref()),
        )
        .await?
    } else {
        fetch_available_versions(
            &client,
            &package,
            &IndexCapabilities::default(),
            &concurrency.downloads_semaphore,
            &exclude_newer,
            python_requirement.target(),
            Some(tags.as_ref()),
        )
        .await?
    };

    match lookup {
        VersionLookup::Found(versions) => {
            let latest = versions
                .first()
                .expect("found versions should contain at least one entry");
            writeln!(printer.stdout(), "{package} ({latest})")?;
            writeln!(
                printer.stdout(),
                "Available versions: {}",
                versions
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
            Ok(ExitStatus::Success)
        }
        VersionLookup::NotFound => {
            writeln!(
                printer.stderr(),
                "{}{} Package not found: `{package}`",
                "error".red().bold(),
                ":".bold(),
            )?;
            Ok(ExitStatus::Failure)
        }
        VersionLookup::NoIndex => {
            writeln!(
                printer.stderr(),
                "{}{} No package index configured. Provide `--index-url`, `--default-index`, or `--find-links`.",
                "error".red().bold(),
                ":".bold(),
            )?;
            Ok(ExitStatus::Failure)
        }
        VersionLookup::Offline => {
            writeln!(
                printer.stderr(),
                "{}{} Network unavailable and `{package}` was not found in the cache.",
                "error".red().bold(),
                ":".bold(),
            )?;
            Ok(ExitStatus::Failure)
        }
        VersionLookup::NoCompatibleVersions => {
            writeln!(
                printer.stderr(),
                "{}{} No compatible versions found for `{package}`.",
                "error".red().bold(),
                ":".bold(),
            )?;
            Ok(ExitStatus::Failure)
        }
    }
}

#[derive(Debug)]
enum VersionLookup {
    Found(Vec<Version>),
    NotFound,
    NoIndex,
    Offline,
    NoCompatibleVersions,
}

async fn fetch_available_versions(
    client: &RegistryClient,
    package: &PackageName,
    capabilities: &IndexCapabilities,
    download_concurrency: &Semaphore,
    exclude_newer: &ExcludeNewer,
    requires_python: &RequiresPython,
    tags: Option<&Tags>,
) -> Result<VersionLookup> {
    debug!("Fetching available versions for: `{package}`");

    let archives = match client
        .simple_detail(package, None, capabilities, download_concurrency)
        .await
    {
        Ok(archives) => archives,
        Err(err) => {
            return Ok(match err.kind() {
                fyn_client::ErrorKind::RemotePackageNotFound(_) => VersionLookup::NotFound,
                fyn_client::ErrorKind::NoIndex(_) => VersionLookup::NoIndex,
                fyn_client::ErrorKind::Offline(_) => VersionLookup::Offline,
                _ => return Err(err.into()),
            });
        }
    };

    let mut versions = BTreeSet::new();
    let mut saw_package = false;

    for (_, archive) in archives {
        match archive {
            MetadataFormat::Simple(archive) => {
                saw_package |= archive.iter().next().is_some();
                for datum in archive.iter().rev() {
                    let version = deserialize::<Version, rkyv::rancor::Error>(&datum.version)
                        .expect("archived version always deserializes");
                    let files = deserialize::<VersionFiles, rkyv::rancor::Error>(&datum.files)
                        .expect("archived version files always deserializes");

                    if files.all().any(|(filename, file)| {
                        is_compatible_file(
                            package,
                            &filename,
                            &file,
                            exclude_newer,
                            requires_python,
                            tags,
                        )
                    }) {
                        versions.insert(version);
                    }
                }
            }
            MetadataFormat::Flat(entries) => {
                saw_package |= !entries.is_empty();
                for entry in entries {
                    if is_compatible_file(
                        package,
                        &entry.filename,
                        &entry.file,
                        exclude_newer,
                        requires_python,
                        tags,
                    ) {
                        versions.insert(entry.filename.version().clone());
                    }
                }
            }
        }
    }

    if versions.is_empty() {
        if saw_package {
            Ok(VersionLookup::NoCompatibleVersions)
        } else {
            Ok(VersionLookup::NotFound)
        }
    } else {
        Ok(VersionLookup::Found(versions.into_iter().rev().collect()))
    }
}

async fn fetch_flat_index_versions<'a>(
    client: &RegistryClient,
    package: &PackageName,
    indexes: impl Iterator<Item = &'a Index>,
    cache: &Cache,
    exclude_newer: &ExcludeNewer,
    requires_python: &RequiresPython,
    tags: Option<&Tags>,
) -> Result<VersionLookup> {
    let client = FlatIndexClient::new(client.cached_client(), client.connectivity(), cache);
    let entries = client.fetch_all(indexes.map(Index::url)).await?;

    let mut versions = BTreeSet::new();
    let mut saw_package = false;

    for entry in entries.entries {
        if entry.filename.name() != package {
            continue;
        }
        saw_package = true;

        if is_compatible_file(
            package,
            &entry.filename,
            &entry.file,
            exclude_newer,
            requires_python,
            tags,
        ) {
            versions.insert(entry.filename.version().clone());
        }
    }

    if versions.is_empty() {
        if saw_package {
            Ok(VersionLookup::NoCompatibleVersions)
        } else {
            Ok(VersionLookup::NotFound)
        }
    } else {
        Ok(VersionLookup::Found(versions.into_iter().rev().collect()))
    }
}

fn is_compatible_file(
    package: &PackageName,
    filename: &DistFilename,
    file: &File,
    exclude_newer: &ExcludeNewer,
    requires_python: &RequiresPython,
    tags: Option<&Tags>,
) -> bool {
    if !filename.version().is_stable() {
        return false;
    }

    if let Some(exclude_newer) = exclude_newer.exclude_newer_package(package) {
        match file.upload_time_utc_ms.as_ref() {
            Some(&upload_time) if upload_time >= exclude_newer.timestamp_millis() => return false,
            None => {
                warn_user_once!(
                    "{} is missing an upload date, but user provided: {}",
                    file.filename,
                    exclude_newer
                );
            }
            _ => {}
        }
    }

    if file
        .yanked
        .as_ref()
        .is_some_and(|yanked| yanked.is_yanked())
    {
        return false;
    }

    if file
        .requires_python
        .as_ref()
        .is_some_and(|file_requires_python| !requires_python.is_contained_by(file_requires_python))
    {
        return false;
    }

    match filename {
        DistFilename::WheelFilename(filename) => {
            tags.is_none_or(|tags| filename.compatibility(tags).is_compatible())
        }
        DistFilename::SourceDistFilename(_) => true,
    }
}
