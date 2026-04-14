use std::ffi::OsString;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, anyhow, bail};
use fyn_cache::Cache;
use fyn_client::BaseClientBuilder;
use fyn_configuration::DependencyGroupsWithDefaults;
use fyn_fs::{Simplified, write_atomic_sync};
use fyn_preview::Preview;
use fyn_python::managed::python_executable_dir;
use fyn_python::{EnvironmentPreference, PythonDownloads, PythonInstallation, PythonPreference};
use fyn_settings::PythonInstallMirrors;
use fyn_shell::Shell;
use fyn_static::EnvVars;
use fyn_warnings::warn_user;
use fyn_workspace::{DiscoveryOptions, VirtualProject, WorkspaceCache, WorkspaceError};
use owo_colors::OwoColorize;

use crate::commands::ExitStatus;
use crate::commands::project::{WorkspacePython, validate_project_requires_python};
use crate::commands::reporters::PythonDownloadReporter;
use crate::printer::Printer;

/// Install a `python` shim into the executable directory.
pub(crate) fn install_shim(force: bool, printer: Printer) -> Result<ExitStatus> {
    let current_exe =
        std::env::current_exe().context("Failed to determine the path to the fyn executable")?;
    let shim_path = shim_path()?;
    let shim_contents = shim_contents(&current_exe, parent_directory(&shim_path)?);

    if let Some(parent) = shim_path.parent() {
        fs_err::create_dir_all(parent)?;
    }

    match fs_err::read(&shim_path) {
        Ok(existing) if existing == shim_contents.as_bytes() => {
            writeln!(
                printer.stderr(),
                "Python shim is already installed at {}",
                shim_path.simplified_display().cyan()
            )?;
            warn_if_not_on_path(parent_directory(&shim_path)?);
            return Ok(ExitStatus::Success);
        }
        Ok(_) => {
            if !force {
                bail!(
                    "Executable already exists at `{}` and is not the current fyn Python shim; use `--force` to replace it",
                    shim_path.simplified_display()
                );
            }
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(err.into()),
    }

    write_atomic_sync(&shim_path, shim_contents.as_bytes())?;
    #[cfg(unix)]
    ensure_executable(&shim_path)?;
    #[cfg(not(unix))]
    ensure_executable(&shim_path);

    writeln!(
        printer.stderr(),
        "Installed Python shim at {}",
        shim_path.simplified_display().cyan()
    )?;
    warn_if_not_on_path(parent_directory(&shim_path)?);

    Ok(ExitStatus::Success)
}

/// Execute the hidden shim entrypoint.
#[expect(clippy::too_many_arguments)]
pub(crate) async fn shim(
    project_dir: &Path,
    args: Vec<OsString>,
    no_config: bool,
    install_mirrors: &PythonInstallMirrors,
    client_builder: &BaseClientBuilder<'_>,
    python_preference: PythonPreference,
    python_downloads: PythonDownloads,
    cache: &Cache,
    workspace_cache: &WorkspaceCache,
    printer: Printer,
    preview: Preview,
) -> Result<ExitStatus> {
    let project =
        match VirtualProject::discover(project_dir, &DiscoveryOptions::default(), workspace_cache)
            .await
        {
            Ok(project) => Some(project),
            Err(
                WorkspaceError::MissingProject(_)
                | WorkspaceError::MissingPyprojectToml
                | WorkspaceError::NonWorkspace(_),
            ) => None,
            Err(err) => {
                warn_user!("{err}");
                None
            }
        };

    let groups = DependencyGroupsWithDefaults::none();
    let WorkspacePython {
        source,
        python_request,
        requires_python,
    } = WorkspacePython::from_request(
        None,
        project.as_ref().map(VirtualProject::workspace),
        &groups,
        project_dir,
        no_config,
    )
    .await?;

    let reporter = PythonDownloadReporter::single(printer);

    let interpreter = PythonInstallation::find_or_download(
        python_request.as_ref(),
        EnvironmentPreference::Any,
        python_preference,
        python_downloads,
        client_builder,
        cache,
        Some(&reporter),
        install_mirrors.python_install_mirror.as_deref(),
        install_mirrors.pypy_install_mirror.as_deref(),
        install_mirrors.python_downloads_json_url.as_deref(),
        preview,
    )
    .await?
    .into_interpreter();

    if let Some(requires_python) = requires_python {
        if let Err(err) = validate_project_requires_python(
            &interpreter,
            project.as_ref().map(VirtualProject::workspace),
            &groups,
            &requires_python,
            &source,
        ) {
            warn_user!("{err}");
        }
    }

    exec_python(interpreter.sys_executable(), &args)
}

fn shim_path() -> Result<PathBuf> {
    let executable_dir = python_executable_dir()?;
    Ok(executable_dir.join(shim_filename()))
}

#[cfg(unix)]
fn shim_filename() -> &'static str {
    "python"
}

#[cfg(windows)]
fn shim_filename() -> &'static str {
    "python.cmd"
}

#[cfg(not(any(unix, windows)))]
fn shim_filename() -> &'static str {
    "python"
}

fn shim_contents(current_exe: &Path, shim_dir: &Path) -> String {
    #[cfg(unix)]
    {
        format!(
            "#!/bin/sh\nexport {}={}\nexec {} python shim -- \"$@\"\n",
            EnvVars::UV_INTERNAL__PYTHON_SHIM_DIR,
            shell_quote(shim_dir),
            shell_quote(current_exe),
        )
    }

    #[cfg(windows)]
    {
        format!(
            "@echo off\r\nset \"{}={}\"\r\n\"{}\" python shim -- %*\r\n",
            EnvVars::UV_INTERNAL__PYTHON_SHIM_DIR,
            batch_quote(shim_dir),
            batch_quote(current_exe),
        )
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = current_exe;
        let _ = shim_dir;
        String::new()
    }
}

#[cfg(unix)]
fn ensure_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs_err::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs_err::set_permissions(path, permissions)?;
    Ok(())
}

#[cfg(not(unix))]
fn ensure_executable(_path: &Path) {}

#[cfg(unix)]
fn shell_quote(path: &Path) -> String {
    let path = path.as_os_str().to_string_lossy();
    format!("'{}'", path.replace('\'', r#"'"'"'"#))
}

#[cfg(windows)]
fn batch_quote(path: &Path) -> String {
    path.as_os_str()
        .to_string_lossy()
        .replace('%', "%%")
        .replace('"', "\"\"")
}

fn parent_directory(path: &Path) -> Result<&Path> {
    path.parent().ok_or_else(|| {
        anyhow!(
            "Failed to determine parent directory for `{}`",
            path.user_display()
        )
    })
}

fn warn_if_not_on_path(bin: &Path) {
    if !Shell::contains_path(bin) {
        if let Some(shell) = Shell::from_env() {
            if let Some(command) = shell.prepend_path(bin) {
                if shell.supports_update() {
                    warn_user!(
                        "`{}` is not on your PATH. To use the Python shim, run `{}` or `{}`.",
                        bin.simplified_display().cyan(),
                        command.green(),
                        "fyn python update-shell".green()
                    );
                } else {
                    warn_user!(
                        "`{}` is not on your PATH. To use the Python shim, run `{}`.",
                        bin.simplified_display().cyan(),
                        command.green()
                    );
                }
            } else {
                warn_user!(
                    "`{}` is not on your PATH. To use the Python shim, add the directory to your PATH.",
                    bin.simplified_display().cyan(),
                );
            }
        } else {
            warn_user!(
                "`{}` is not on your PATH. To use the Python shim, add the directory to your PATH.",
                bin.simplified_display().cyan(),
            );
        }
    }
}

fn exec_python(python: &Path, args: &[OsString]) -> Result<ExitStatus> {
    let mut command = Command::new(python);
    command.args(args);

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;

        let err = command.exec();
        Err(err.into())
    }

    #[cfg(windows)]
    {
        match fyn_windows::spawn_child(&mut command, false)? {}
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = command;
        Err(anyhow!(
            "The Python shim is only supported on Unix and Windows"
        ))
    }
}
