use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result, anyhow};
use futures::StreamExt;
use itertools::Itertools;
use owo_colors::OwoColorize;
use tokio::io::AsyncWriteExt;
use tracing::debug;

use fyn_cache::Cache;
use fyn_client::{BaseClientBuilder, FlatIndexClient, RegistryClient, RegistryClientBuilder};
use fyn_configuration::KeyringProviderType;
use fyn_configuration::{
    BuildIsolation, BuildOptions, Concurrency, ExtrasSpecification, IndexStrategy, NoSources,
    Reinstall, TargetTriple, Upgrade,
};
use fyn_dispatch::{BuildDispatch, SharedState};
use fyn_distribution::LoweredExtraBuildDependencies;
use fyn_distribution_types::{
    BuiltDist, ConfigSettings, DependencyMetadata, Dist, ExtraBuildVariables, Index,
    IndexLocations, NameRequirementSpecification, Origin, PackageConfigSettings, RemoteSource,
    Requirement, Resolution, ResolvedDist, SourceDist, UnresolvedRequirementSpecification,
};
use fyn_fs::Simplified;
use fyn_install_wheel::LinkMode;
use fyn_normalize::PackageName;
use fyn_preview::Preview;
use fyn_pypi_types::Conflicts;
use fyn_python::{
    EnvironmentPreference, PythonDownloads, PythonEnvironment, PythonInstallation,
    PythonPreference, PythonRequest, PythonVersion, VersionRequest,
};
use fyn_requirements::{GroupsSpecification, RequirementsSource, RequirementsSpecification};
use fyn_resolver::{
    DependencyMode, ExcludeNewer, FlatIndex, ForkStrategy, InMemoryIndex, OptionsBuilder,
    PrereleaseMode, PythonRequirement, ResolutionMode, ResolverEnvironment,
};
use fyn_settings::PythonInstallMirrors;
use fyn_torch::{TorchMode, TorchSource, TorchStrategy};
use fyn_types::{EmptyInstalledPackages, HashStrategy};
use fyn_warnings::warn_user;
use fyn_workspace::WorkspaceCache;
use fyn_workspace::pyproject::ExtraBuildDependencies;

use crate::commands::pip::loggers::DefaultResolveLogger;
use crate::commands::pip::{operations, resolution_markers, resolution_tags};
use crate::commands::reporters::PythonDownloadReporter;
use crate::commands::{ExitStatus, diagnostics, elapsed};
use crate::printer::Printer;

/// Download distribution archives into a directory.
pub(crate) async fn pip_download(
    requirements: &[RequirementsSource],
    constraints: &[RequirementsSource],
    overrides: &[RequirementsSource],
    excludes: &[RequirementsSource],
    build_constraints: &[RequirementsSource],
    constraints_from_workspace: Vec<Requirement>,
    overrides_from_workspace: Vec<Requirement>,
    excludes_from_workspace: Vec<PackageName>,
    build_constraints_from_workspace: Vec<Requirement>,
    extras: &ExtrasSpecification,
    groups: &GroupsSpecification,
    resolution_mode: ResolutionMode,
    prerelease_mode: PrereleaseMode,
    fork_strategy: ForkStrategy,
    dependency_mode: DependencyMode,
    index_locations: IndexLocations,
    index_strategy: IndexStrategy,
    torch_backend: Option<TorchMode>,
    dependency_metadata: DependencyMetadata,
    keyring_provider: KeyringProviderType,
    client_builder: &BaseClientBuilder<'_>,
    config_settings: ConfigSettings,
    config_settings_package: PackageConfigSettings,
    build_isolation: BuildIsolation,
    extra_build_dependencies: &ExtraBuildDependencies,
    extra_build_variables: &ExtraBuildVariables,
    build_options: BuildOptions,
    install_mirrors: PythonInstallMirrors,
    mut python_version: Option<PythonVersion>,
    python_platform: Option<TargetTriple>,
    python_downloads: PythonDownloads,
    exclude_newer: ExcludeNewer,
    sources: NoSources,
    link_mode: LinkMode,
    mut python: Option<String>,
    system: bool,
    python_preference: PythonPreference,
    concurrency: Concurrency,
    cache: Cache,
    workspace_cache: WorkspaceCache,
    dest: Option<&Path>,
    printer: Printer,
    preview: Preview,
) -> Result<ExitStatus> {
    let start = std::time::Instant::now();
    let client_builder = client_builder.clone().keyring(keyring_provider);

    if python.is_none() && python_version.is_none() {
        if let Ok(request) = std::env::var(fyn_static::EnvVars::UV_PYTHON) {
            if !request.is_empty() {
                python = Some(request);
            }
        }
    }

    if python_version.is_none() {
        if let Some(request) = python.as_ref() {
            if let Ok(version) = PythonVersion::from_str(request) {
                python_version = Some(version);
                python = None;
            }
        }
    }

    let RequirementsSpecification {
        project,
        requirements,
        constraints,
        overrides,
        excludes,
        pylock,
        source_trees,
        groups,
        extras: used_extras,
        index_url,
        extra_index_urls,
        no_index,
        find_links,
        no_binary,
        no_build,
    } = operations::read_requirements(
        requirements,
        constraints,
        overrides,
        excludes,
        extras,
        Some(groups),
        &client_builder,
    )
    .await?;

    if pylock.is_some() {
        return Err(anyhow!(
            "`pylock.toml` is not a supported input format for `fyn pip download`"
        ));
    }

    let constraints = constraints
        .iter()
        .cloned()
        .chain(
            constraints_from_workspace
                .into_iter()
                .map(NameRequirementSpecification::from),
        )
        .collect();

    let overrides: Vec<UnresolvedRequirementSpecification> = overrides
        .iter()
        .cloned()
        .chain(
            overrides_from_workspace
                .into_iter()
                .map(UnresolvedRequirementSpecification::from),
        )
        .collect();

    let excludes: Vec<PackageName> = excludes
        .into_iter()
        .chain(excludes_from_workspace)
        .collect();

    let build_constraints: Vec<NameRequirementSpecification> =
        operations::read_constraints(build_constraints, &client_builder)
            .await?
            .into_iter()
            .chain(
                build_constraints_from_workspace
                    .into_iter()
                    .map(NameRequirementSpecification::from),
            )
            .collect();

    if source_trees.is_empty() {
        let mut unused_extras = extras
            .explicit_names()
            .filter(|extra| !used_extras.contains(extra))
            .collect::<Vec<_>>();
        if !unused_extras.is_empty() {
            unused_extras.sort_unstable();
            unused_extras.dedup();
            let s = if unused_extras.len() == 1 { "" } else { "s" };
            return Err(anyhow!(
                "Requested extra{s} not found: {}",
                unused_extras.iter().join(", ")
            ));
        }
    }

    let environment_preference = EnvironmentPreference::from_system_flag(system, false);
    let python_preference = python_preference.with_system_flag(system);
    let reporter = PythonDownloadReporter::single(printer);
    let interpreter = if let Some(python) = python.as_ref() {
        let request = PythonRequest::parse(python);
        PythonInstallation::find_or_download(
            Some(&request),
            environment_preference,
            python_preference,
            python_downloads,
            &client_builder,
            &cache,
            Some(&reporter),
            install_mirrors.python_install_mirror.as_deref(),
            install_mirrors.pypy_install_mirror.as_deref(),
            install_mirrors.python_downloads_json_url.as_deref(),
            preview,
        )
        .await
    } else {
        let request = if let Some(version) = python_version.as_ref() {
            PythonRequest::Version(VersionRequest::from(version))
        } else {
            PythonRequest::default()
        };
        PythonInstallation::find_best(
            &request,
            environment_preference,
            python_preference,
            python_downloads,
            &client_builder,
            &cache,
            Some(&reporter),
            install_mirrors.python_install_mirror.as_deref(),
            install_mirrors.pypy_install_mirror.as_deref(),
            install_mirrors.python_downloads_json_url.as_deref(),
            preview,
        )
        .await
    }?
    .into_interpreter();

    debug!(
        "Using Python {} interpreter at {} for builds",
        interpreter.python_version(),
        interpreter.sys_executable().user_display().cyan()
    );

    if let Some(python_version) = python_version.as_ref() {
        let matches_without_patch = {
            python_version.major() == interpreter.python_major()
                && python_version.minor() == interpreter.python_minor()
        };
        if no_build.is_none()
            && python.is_none()
            && python_version.version() != interpreter.python_version()
            && (python_version.patch().is_some() || !matches_without_patch)
        {
            warn_user!(
                "The requested Python version {} is not available; {} will be used to build dependencies instead.",
                python_version.version(),
                interpreter.python_version(),
            );
        }
    }

    let state = SharedState::default();
    let top_level_index = if python_version.is_some() {
        InMemoryIndex::default()
    } else {
        state.index().clone()
    };

    let python_requirement = if let Some(python_version) = python_version.as_ref() {
        PythonRequirement::from_python_version(&interpreter, python_version)
    } else {
        PythonRequirement::from_interpreter(&interpreter)
    };

    let tags = resolution_tags(
        python_version.as_ref(),
        python_platform.as_ref(),
        &interpreter,
    )?;
    let marker_env = resolution_markers(
        python_version.as_ref(),
        python_platform.as_ref(),
        &interpreter,
    );

    let hasher = HashStrategy::None;

    let index_locations = index_locations.combine(
        extra_index_urls
            .into_iter()
            .map(Index::from_extra_index_url)
            .chain(index_url.map(Index::from_index_url))
            .map(|index| index.with_origin(Origin::RequirementsTxt))
            .collect(),
        find_links
            .into_iter()
            .map(Index::from_find_links)
            .map(|index| index.with_origin(Origin::RequirementsTxt))
            .collect(),
        no_index,
    );

    let torch_backend = torch_backend
        .map(|mode| {
            let source = if fyn_auth::PyxTokenStore::from_settings()
                .is_ok_and(|store| store.has_credentials())
            {
                TorchSource::Pyx
            } else {
                TorchSource::default()
            };
            TorchStrategy::from_mode(
                mode,
                source,
                python_platform
                    .map(TargetTriple::platform)
                    .as_ref()
                    .unwrap_or(interpreter.platform())
                    .os(),
            )
        })
        .transpose()?;

    let client = RegistryClientBuilder::new(client_builder.clone(), cache.clone())
        .index_locations(index_locations.clone())
        .index_strategy(index_strategy)
        .torch_backend(torch_backend.clone())
        .markers(interpreter.markers())
        .platform(interpreter.platform())
        .build();

    let build_options = build_options.combine(no_binary, no_build);

    let flat_index = {
        let client = FlatIndexClient::new(client.cached_client(), client.connectivity(), &cache);
        let entries = client
            .fetch_all(index_locations.flat_indexes().map(Index::url))
            .await?;
        FlatIndex::from_entries(entries, Some(&tags), &hasher, &build_options)
    };

    let environment;
    let types_build_isolation = match build_isolation {
        BuildIsolation::Isolate => fyn_types::BuildIsolation::Isolated,
        BuildIsolation::Shared => {
            environment = PythonEnvironment::from_interpreter(interpreter.clone());
            fyn_types::BuildIsolation::Shared(&environment)
        }
        BuildIsolation::SharedPackage(ref packages) => {
            environment = PythonEnvironment::from_interpreter(interpreter.clone());
            fyn_types::BuildIsolation::SharedPackage(&environment, packages)
        }
    };

    let build_hashes = HashStrategy::None;
    let build_constraints = fyn_configuration::Constraints::from_requirements(
        build_constraints
            .iter()
            .map(|constraint| constraint.requirement.clone()),
    );

    let extra_build_requires =
        LoweredExtraBuildDependencies::from_non_lowered(extra_build_dependencies.clone())
            .into_inner();

    let build_dispatch = BuildDispatch::new(
        &client,
        &cache,
        &build_constraints,
        &interpreter,
        &index_locations,
        &flat_index,
        &dependency_metadata,
        state,
        index_strategy,
        &config_settings,
        &config_settings_package,
        types_build_isolation,
        &extra_build_requires,
        extra_build_variables,
        link_mode,
        &build_options,
        &build_hashes,
        exclude_newer.clone(),
        sources,
        workspace_cache,
        concurrency.clone(),
        preview,
    );

    let options = OptionsBuilder::new()
        .resolution_mode(resolution_mode)
        .prerelease_mode(prerelease_mode)
        .fork_strategy(fork_strategy)
        .dependency_mode(dependency_mode)
        .exclude_newer(exclude_newer.clone())
        .index_strategy(index_strategy)
        .torch_backend(torch_backend)
        .build_options(build_options.clone())
        .build();

    let resolution = match operations::resolve(
        requirements,
        constraints,
        overrides,
        excludes,
        source_trees,
        project,
        BTreeSet::default(),
        extras,
        &groups,
        Vec::default(),
        EmptyInstalledPackages,
        &hasher,
        &Reinstall::None,
        &Upgrade::none(),
        Some(&tags),
        ResolverEnvironment::specific(marker_env.clone()),
        python_requirement,
        interpreter.markers(),
        Conflicts::empty(),
        &client,
        &flat_index,
        &top_level_index,
        &build_dispatch,
        &concurrency,
        options,
        Box::new(DefaultResolveLogger),
        printer,
    )
    .await
    {
        Ok((resolution, _)) => Resolution::from(resolution),
        Err(err) => {
            return diagnostics::OperationDiagnostic::native_tls(client_builder.is_native_tls())
                .report(err)
                .map_or(Ok(ExitStatus::Failure), |err| Err(err.into()));
        }
    };

    operations::diagnose_resolution(resolution.diagnostics(), printer)?;

    let dest = if let Some(dest) = dest {
        std::path::absolute(dest)?
    } else {
        std::env::current_dir()?
    };
    if dest.exists() && !dest.is_dir() {
        return Err(anyhow!(
            "Destination is not a directory: {}",
            dest.user_display()
        ));
    }
    fs_err::tokio::create_dir_all(&dest).await?;

    let resolved = resolution
        .distributions()
        .filter_map(|dist| match dist {
            ResolvedDist::Installable { dist, .. } => Some(dist.as_ref()),
            ResolvedDist::Installed { .. } => None,
        })
        .sorted_by_key(std::string::ToString::to_string)
        .collect::<Vec<_>>();

    let mut downloaded = 0usize;
    for dist in resolved {
        let path = download_distribution(dist, &dest, &client).await?;
        writeln!(printer.stderr(), "Saved {}", path.user_display().cyan())?;
        downloaded += 1;
    }

    let s = if downloaded == 1 { "" } else { "s" };
    writeln!(
        printer.stderr(),
        "{}",
        format!(
            "Downloaded {} file{s} {}",
            downloaded.to_string().bold(),
            format!("in {}", elapsed(start.elapsed())).dimmed()
        )
        .dimmed()
    )?;

    Ok(ExitStatus::Success)
}

async fn download_distribution(
    dist: &Dist,
    dest_dir: &Path,
    client: &RegistryClient,
) -> Result<PathBuf> {
    let filename = dist.filename()?.into_owned();
    let target = dest_dir.join(&filename);

    match dist {
        Dist::Built(BuiltDist::Registry(dist)) => {
            let url = dist
                .best_wheel()
                .file
                .url
                .to_url()
                .context("Failed to resolve wheel download URL")?;
            download_from_url(&url, &target, client).await?;
        }
        Dist::Source(SourceDist::Registry(dist)) => {
            let url = dist
                .file
                .url
                .to_url()
                .context("Failed to resolve source distribution download URL")?;
            download_from_url(&url, &target, client).await?;
        }
        Dist::Built(BuiltDist::DirectUrl(dist)) => {
            download_from_url(dist.location.as_ref(), &target, client).await?;
        }
        Dist::Source(SourceDist::DirectUrl(dist)) => {
            download_from_url(dist.location.as_ref(), &target, client).await?;
        }
        Dist::Built(BuiltDist::Path(dist)) => {
            copy_local_archive(dist.install_path.as_ref(), &target).await?;
        }
        Dist::Source(SourceDist::Path(dist)) => {
            copy_local_archive(dist.install_path.as_ref(), &target).await?;
        }
        Dist::Source(SourceDist::Git(dist)) => {
            return Err(anyhow!(
                "`fyn pip download` does not support Git requirements yet: {}",
                dist.url
            ));
        }
        Dist::Source(SourceDist::Directory(dist)) => {
            return Err(anyhow!(
                "`fyn pip download` does not support local source trees yet: {}",
                dist.install_path.user_display()
            ));
        }
    }

    Ok(target)
}

pub(super) async fn copy_local_archive(source: &Path, target: &Path) -> Result<()> {
    if source == target {
        return Ok(());
    }

    fs_err::tokio::copy(source, target).await.with_context(|| {
        format!(
            "Failed to copy `{}` to `{}`",
            source.user_display(),
            target.user_display()
        )
    })?;
    Ok(())
}

pub(super) async fn download_from_url(
    url: &fyn_redacted::DisplaySafeUrl,
    target: &Path,
    client: &RegistryClient,
) -> Result<()> {
    if url.scheme() == "file" {
        let path = url
            .to_file_path()
            .map_err(|()| anyhow!("Non-file URL: {url}"))?;
        copy_local_archive(&path, target).await?;
        return Ok(());
    }

    let filename = target
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("Invalid target filename: {}", target.user_display()))?;
    let temp_path = target.with_file_name(format!(".{filename}.part"));

    let result = async {
        let response = client
            .uncached_client(url)
            .get(url::Url::from(url.clone()))
            .header(
                "accept-encoding",
                reqwest::header::HeaderValue::from_static("identity"),
            )
            .send()
            .await?;
        response.error_for_status_ref()?;

        let mut file = fs_err::tokio::File::create(&temp_path).await?;
        let mut reader = response.bytes_stream();
        while let Some(chunk) = reader.next().await {
            file.write_all(&chunk?).await?;
        }
        file.flush().await?;
        drop(file);

        fs_err::tokio::rename(&temp_path, target).await?;
        Ok::<(), anyhow::Error>(())
    }
    .await;

    if result.is_err() {
        let _ = fs_err::tokio::remove_file(&temp_path).await;
    }

    result.with_context(|| format!("Failed to download `{url}`"))
}
