use std::fmt::Write;
use std::path::Path;

use anyhow::{Result, bail};
use fyn_distribution_types::RequiresPython;
use fyn_python::downloads::{ManagedPythonDownloadList, PythonDownloadRequest};
use owo_colors::OwoColorize;
use tracing::debug;

use fyn_cache::Cache;
use fyn_client::BaseClientBuilder;
use fyn_configuration::DependencyGroupsWithDefaults;
use fyn_fs::Simplified;
use fyn_preview::Preview;
use fyn_python::{
    EnvironmentPreference, PYTHON_VERSION_FILENAME, PythonDownloads, PythonInstallation,
    PythonInstallationKey, PythonPreference, PythonRequest, PythonVersionFile,
    VersionFileDiscoveryOptions, VersionRequest,
};
use fyn_settings::PythonInstallMirrors;
use fyn_warnings::warn_user_once;
use fyn_workspace::{DiscoveryOptions, VirtualProject, WorkspaceCache};

use crate::commands::{
    ExitStatus, project::find_requires_python, reporters::PythonDownloadReporter,
};
use crate::printer::Printer;

/// Pin to a specific Python version.
#[expect(clippy::fn_params_excessive_bools)]
pub(crate) async fn pin(
    project_dir: &Path,
    request: Option<String>,
    resolved: bool,
    upgrade: bool,
    python_preference: PythonPreference,
    python_downloads: PythonDownloads,
    no_project: bool,
    global: bool,
    rm: bool,
    install_mirrors: PythonInstallMirrors,
    client_builder: BaseClientBuilder<'_>,
    cache: &Cache,
    workspace_cache: &WorkspaceCache,
    printer: Printer,
    preview: Preview,
) -> Result<ExitStatus> {
    let virtual_project = if no_project {
        None
    } else {
        match VirtualProject::discover(project_dir, &DiscoveryOptions::default(), workspace_cache)
            .await
        {
            Ok(virtual_project) => Some(virtual_project),
            Err(err) => {
                debug!("Failed to discover virtual project: {err}");
                None
            }
        }
    };

    // Search for an existing file, we won't necessarily write to this, we'll construct a target
    // path if there's a request later on.
    let version_file = PythonVersionFile::discover(
        project_dir,
        &VersionFileDiscoveryOptions::default().with_no_local(global),
    )
    .await?;

    if rm {
        let Some(file) = version_file else {
            if global {
                bail!("No global Python pin found");
            }
            bail!("No Python version file found");
        };

        if !global && file.is_global() {
            bail!("No Python version file found; use `--rm --global` to remove the global pin");
        }

        fs_err::tokio::remove_file(file.path()).await?;
        writeln!(
            printer.stdout(),
            "Removed {} at `{}`",
            if global {
                "global Python pin"
            } else {
                "Python version file"
            },
            file.path().user_display()
        )?;
        return Ok(ExitStatus::Success);
    }

    if request.is_none() && !upgrade {
        // Display the current pinned Python version
        if let Some(file) = version_file.as_ref() {
            for pin in file.versions() {
                writeln!(printer.stdout(), "{}", pin.to_canonical_string())?;
                if let Some(virtual_project) = &virtual_project {
                    let client = client_builder.clone().retries(0).build();
                    let download_list = ManagedPythonDownloadList::new(
                        &client,
                        install_mirrors.python_downloads_json_url.as_deref(),
                    )
                    .await?;
                    warn_if_existing_pin_incompatible_with_project(
                        pin,
                        virtual_project,
                        python_preference,
                        &download_list,
                        cache,
                        preview,
                    );
                }
            }
            return Ok(ExitStatus::Success);
        }
        bail!("No Python version file found; specify a version to create one");
    }

    let mut inherited_versions = None;
    let request = if let Some(request) = request {
        PythonRequest::parse(&request)
    } else {
        let Some(file) = version_file.as_ref() else {
            bail!("No Python version file found; specify a version to create one");
        };
        let versions = file.clone().into_versions();
        let Some(request) = versions.first().cloned() else {
            bail!("No Python version file found; specify a version to create one");
        };
        inherited_versions = Some(versions);
        request
    };

    if let PythonRequest::ExecutableName(name) = request {
        bail!("Requests for arbitrary names (e.g., `{name}`) are not supported in version files");
    }

    let request = if upgrade {
        let requires_python = virtual_project
            .as_ref()
            .map(project_requires_python)
            .transpose()?
            .flatten();
        resolve_upgraded_pin(
            &request,
            requires_python.as_ref(),
            &install_mirrors,
            client_builder.clone(),
        )
        .await?
    } else {
        request
    };

    let existing = version_file.clone();

    let request = if upgrade && !resolved {
        request
    } else {
        let reporter = PythonDownloadReporter::single(printer);

        let python = match PythonInstallation::find_or_download(
            Some(&request),
            EnvironmentPreference::OnlySystem,
            python_preference,
            python_downloads,
            &client_builder,
            cache,
            Some(&reporter),
            install_mirrors.python_install_mirror.as_deref(),
            install_mirrors.pypy_install_mirror.as_deref(),
            install_mirrors.python_downloads_json_url.as_deref(),
            preview,
        )
        .await
        {
            Ok(python) => Some(python),
            // If no matching Python version is found, don't fail unless `resolved` was requested
            Err(fyn_python::Error::MissingPython(err, ..)) if !resolved => {
                // N.B. We omit the hint and just show the inner error message
                warn_user_once!("{err}");
                None
            }
            // If there was some other error, log it
            Err(err) if !resolved => {
                debug!("{err}");
                None
            }
            // If `resolved` was requested, we must find an interpreter — fail otherwise
            Err(err) => return Err(err.into()),
        };

        if !upgrade && let Some(virtual_project) = &virtual_project {
            if let Some(request_version) = request.as_pep440_version() {
                assert_pin_compatible_with_project(
                    &Pin {
                        request: &request,
                        version: &request_version,
                        resolved: false,
                        existing: false,
                    },
                    virtual_project,
                )?;
            } else if let Some(python) = &python {
                // Warn if the resolved Python is incompatible with the Python requirement unless --resolved is used
                if let Err(err) = assert_pin_compatible_with_project(
                    &Pin {
                        request: &request,
                        version: python.python_version(),
                        resolved: true,
                        existing: false,
                    },
                    virtual_project,
                ) {
                    if resolved {
                        return Err(err);
                    }
                    warn_user_once!("{err}");
                }
            }
        }

        if resolved {
            // SAFETY: We exit early if Python is not found and resolved is `true`
            // TODO(zanieb): Maybe avoid reparsing here?
            PythonRequest::parse(
                &python
                    .unwrap()
                    .interpreter()
                    .sys_executable()
                    .user_display()
                    .to_string(),
            )
        } else {
            request
        }
    };

    let new = if upgrade {
        if let Some(mut versions) = inherited_versions {
            let existing = existing
                .clone()
                .expect("inherited versions implies existing version file");
            versions[0] = request.clone();
            existing.with_versions(versions)
        } else if global {
            let Some(new) = PythonVersionFile::global() else {
                // TODO(zanieb): We should find a nice way to surface that as an error
                bail!("Failed to determine directory for global Python pin");
            };
            new.with_versions(vec![request.clone()])
        } else {
            PythonVersionFile::new(project_dir.join(PYTHON_VERSION_FILENAME))
                .with_versions(vec![request.clone()])
        }
    } else if global {
        let Some(new) = PythonVersionFile::global() else {
            // TODO(zanieb): We should find a nice way to surface that as an error
            bail!("Failed to determine directory for global Python pin");
        };
        new.with_versions(vec![request.clone()])
    } else {
        PythonVersionFile::new(project_dir.join(PYTHON_VERSION_FILENAME))
            .with_versions(vec![request.clone()])
    };

    if upgrade
        && existing
            .as_ref()
            .filter(|existing| existing.path() == new.path())
            .is_some_and(|existing| existing.clone().into_versions() == new.clone().into_versions())
    {
        writeln!(
            printer.stdout(),
            "`{}` is already pinned to the latest compatible version",
            request.to_canonical_string().green()
        )?;
        return Ok(ExitStatus::Success);
    }

    new.write().await?;

    // If we updated an existing version file to a new version
    if let Some(existing) = existing
        .as_ref()
        .filter(|existing| existing.path() == new.path())
        .and_then(PythonVersionFile::version)
        .filter(|version| *version != new.version().unwrap())
    {
        writeln!(
            printer.stdout(),
            "Updated `{}` from `{}` -> `{}`",
            new.path().user_display().cyan(),
            existing.to_canonical_string().green(),
            new.version().unwrap().to_canonical_string().green()
        )?;
    } else {
        writeln!(
            printer.stdout(),
            "Pinned `{}` to `{}`",
            new.path().user_display().cyan(),
            new.version().unwrap().to_canonical_string().green()
        )?;
    }

    Ok(ExitStatus::Success)
}

async fn resolve_upgraded_pin(
    request: &PythonRequest,
    requires_python: Option<&RequiresPython>,
    install_mirrors: &PythonInstallMirrors,
    client_builder: BaseClientBuilder<'_>,
) -> Result<PythonRequest> {
    let upgrade_request = upgrade_request(request)?;
    let client = client_builder.retries(0).build();
    let download_list = ManagedPythonDownloadList::new(
        &client,
        install_mirrors.python_downloads_json_url.as_deref(),
    )
    .await?;

    let Some(download) = download_list
        .iter_matching(&upgrade_request)
        .find(|download| {
            requires_python.is_none_or(|requires_python| {
                requires_python.contains(download.key().version().version())
            })
        })
    else {
        if let Some(requires_python) = requires_python {
            bail!(
                "No Python download found for `{}` that satisfies the project `requires-python` value of `{}`",
                request.to_canonical_string(),
                requires_python.specifiers()
            );
        }
        bail!(
            "No Python download found to upgrade `{}`",
            request.to_canonical_string()
        );
    };

    exact_pin_from_download(request, download.key())
}

fn upgrade_request(request: &PythonRequest) -> Result<PythonDownloadRequest> {
    let download_request = match request {
        PythonRequest::Default | PythonRequest::Any => PythonDownloadRequest::default(),
        PythonRequest::Directory(path) | PythonRequest::File(path) => {
            bail!(
                "`--upgrade` is only supported for version, implementation, or download requests; got path `{}`",
                path.user_display()
            );
        }
        PythonRequest::ExecutableName(name) => {
            bail!(
                "`--upgrade` is only supported for version, implementation, or download requests; got executable name `{name}`"
            );
        }
        request => PythonDownloadRequest::from_request(request).ok_or_else(|| {
            anyhow::anyhow!(
                "`--upgrade` is only supported for version, implementation, or download requests"
            )
        })?,
    }
    .fill()?;

    if request.includes_patch() || request.includes_prerelease() {
        Ok(download_request.without_patch())
    } else {
        Ok(download_request)
    }
}

fn exact_pin_from_download(
    base_request: &PythonRequest,
    key: &PythonInstallationKey,
) -> Result<PythonRequest> {
    Ok(match base_request {
        PythonRequest::Default | PythonRequest::Any | PythonRequest::Version(_) => {
            PythonRequest::Version(exact_version_request(key))
        }
        PythonRequest::Implementation(implementation)
        | PythonRequest::ImplementationVersion(implementation, _) => {
            PythonRequest::ImplementationVersion(*implementation, exact_version_request(key))
        }
        PythonRequest::Key(_) => PythonRequest::Key(PythonDownloadRequest::try_from(key).map_err(
            |implementation| {
                anyhow::anyhow!(
                    "Cannot create an upgraded pin for unknown implementation `{implementation}`"
                )
            },
        )?),
        PythonRequest::Directory(path) | PythonRequest::File(path) => {
            bail!(
                "`--upgrade` is only supported for version, implementation, or download requests; got path `{}`",
                path.user_display()
            );
        }
        PythonRequest::ExecutableName(name) => {
            bail!(
                "`--upgrade` is only supported for version, implementation, or download requests; got executable name `{name}`"
            );
        }
    })
}

fn exact_version_request(key: &PythonInstallationKey) -> VersionRequest {
    let version = key.version();
    let variant = *key.variant();

    if let Some(prerelease) = version.pre() {
        VersionRequest::MajorMinorPrerelease(version.major(), version.minor(), prerelease, variant)
    } else {
        VersionRequest::MajorMinorPatch(
            version.major(),
            version.minor(),
            version.patch().unwrap_or_default(),
            variant,
        )
    }
}

/// Check if pinned request is compatible with the workspace/project's `Requires-Python`.
fn warn_if_existing_pin_incompatible_with_project(
    pin: &PythonRequest,
    virtual_project: &VirtualProject,
    python_preference: PythonPreference,
    downloads_list: &ManagedPythonDownloadList,
    cache: &Cache,
    preview: Preview,
) {
    // Check if the pinned version is compatible with the project.
    if let Some(pin_version) = pin.as_pep440_version() {
        if let Err(err) = assert_pin_compatible_with_project(
            &Pin {
                request: pin,
                version: &pin_version,
                resolved: false,
                existing: true,
            },
            virtual_project,
        ) {
            warn_user_once!("{err}");
            return;
        }
    }

    // If there is not a version in the pinned request, attempt to resolve the pin into an
    // interpreter to check for compatibility on the current system.
    match PythonInstallation::find(
        pin,
        EnvironmentPreference::OnlySystem,
        python_preference,
        downloads_list,
        cache,
        preview,
    ) {
        Ok(python) => {
            let python_version = python.python_version();
            debug!(
                "The pinned Python version `{}` resolves to `{}`",
                pin, python_version
            );
            // Warn on incompatibilities when viewing existing pins
            if let Err(err) = assert_pin_compatible_with_project(
                &Pin {
                    request: pin,
                    version: python_version,
                    resolved: true,
                    existing: true,
                },
                virtual_project,
            ) {
                warn_user_once!("{err}");
            }
        }
        Err(err) => {
            warn_user_once!(
                "Failed to resolve pinned Python version `{}`: {err}",
                pin.to_canonical_string(),
            );
        }
    }
}

/// Utility struct for representing pins in error messages.
struct Pin<'a> {
    request: &'a PythonRequest,
    version: &'a fyn_pep440::Version,
    resolved: bool,
    existing: bool,
}

fn project_requires_python(virtual_project: &VirtualProject) -> Result<Option<RequiresPython>> {
    // Don't factor in requires-python settings on dependency-groups
    let groups = DependencyGroupsWithDefaults::none();

    match virtual_project {
        VirtualProject::Project(project_workspace) => {
            debug!(
                "Discovered project `{}` at: {}",
                project_workspace.project_name(),
                project_workspace.workspace().install_path().display()
            );

            Ok(find_requires_python(
                project_workspace.workspace(),
                &groups,
            )?)
        }
        VirtualProject::NonProject(workspace) => {
            debug!(
                "Discovered virtual workspace at: {}",
                workspace.install_path().display()
            );
            Ok(find_requires_python(workspace, &groups)?)
        }
    }
}

/// Checks if the pinned Python version is compatible with the workspace/project's `Requires-Python`.
fn assert_pin_compatible_with_project(pin: &Pin, virtual_project: &VirtualProject) -> Result<()> {
    let (requires_python, project_type) = match virtual_project {
        VirtualProject::Project(..) => (project_requires_python(virtual_project)?, "project"),
        VirtualProject::NonProject(..) => (project_requires_python(virtual_project)?, "workspace"),
    };

    let Some(requires_python) = requires_python else {
        return Ok(());
    };

    if requires_python.contains(pin.version) {
        return Ok(());
    }

    let given = if pin.existing { "pinned" } else { "requested" };
    let resolved = if pin.resolved {
        format!(" resolves to `{}` which ", pin.version)
    } else {
        String::new()
    };

    Err(anyhow::anyhow!(
        "The {given} Python version `{}`{resolved} is incompatible with the {} `requires-python` value of `{}`.",
        pin.request.to_canonical_string(),
        project_type,
        requires_python
    ))
}
