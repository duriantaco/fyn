use std::env;
use std::fmt::Write;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use fv_cache::Cache;
use fv_fs::Simplified;
use fv_python::PythonEnvironment;
use fv_shell::Shell;
use fv_static::EnvVars;
use fv_workspace::{DiscoveryOptions, VirtualProject, WorkspaceCache};

use crate::commands::ExitStatus;
use crate::printer::Printer;

/// Spawn a new shell with the virtual environment activated.
pub(crate) async fn shell(
    path: Option<PathBuf>,
    no_project: bool,
    cache: &Cache,
    printer: Printer,
) -> Result<ExitStatus> {
    // Find the virtual environment.
    let venv_path = if let Some(path) = path {
        // Explicit path provided.
        path
    } else if let Some(value) = env::var_os(EnvVars::VIRTUAL_ENV) {
        // VIRTUAL_ENV is set.
        PathBuf::from(value)
    } else if no_project {
        // No project discovery — look for .venv in the current directory.
        let venv = PathBuf::from(".venv");
        if !venv.exists() {
            writeln!(
                printer.stderr(),
                "{}{} No virtual environment found at {}",
                "error".red().bold(),
                ":".bold(),
                venv.simplified_display().cyan()
            )?;
            return Ok(ExitStatus::Failure);
        }
        venv
    } else {
        // Try to find the project's virtual environment.
        if let Ok(project) = VirtualProject::discover(
            &env::current_dir()?,
            &DiscoveryOptions::default(),
            &WorkspaceCache::default(),
        )
        .await
        {
            project.workspace().venv(None)
        } else {
            // Fall back to .venv in the current directory.
            let venv = PathBuf::from(".venv");
            if !venv.exists() {
                writeln!(
                    printer.stderr(),
                    "{}{} No virtual environment found. Create one with {}",
                    "error".red().bold(),
                    ":".bold(),
                    "fv venv".green()
                )?;
                return Ok(ExitStatus::Failure);
            }
            venv
        }
    };

    // Validate the virtual environment.
    let Ok(venv) = PythonEnvironment::from_root(&venv_path, cache) else {
        writeln!(
            printer.stderr(),
            "{}{} Invalid virtual environment at {}",
            "error".red().bold(),
            ":".bold(),
            venv_path.simplified_display().cyan()
        )?;
        return Ok(ExitStatus::Failure);
    };

    // Detect the current shell.
    let detected_shell = Shell::from_env().context(
        "Could not detect the current shell. Set the SHELL environment variable and try again.",
    )?;

    // Get the scripts directory (bin on Unix, Scripts on Windows).
    let scripts = venv.scripts();

    // Build the new PATH with the venv's scripts directory prepended.
    let path_var = env::var_os("PATH").unwrap_or_default();
    let new_path =
        env::join_paths(std::iter::once(scripts.to_path_buf()).chain(env::split_paths(&path_var)))?;

    writeln!(
        printer.stderr(),
        "{}{} Activated virtual environment at {}",
        "success".green().bold(),
        ":".bold(),
        venv.root().simplified_display().cyan()
    )?;
    writeln!(printer.stderr(), "Type {} to deactivate.", "exit".green())?;

    // Determine the shell executable.
    let shell_exe = match detected_shell {
        Shell::Bash => "bash",
        Shell::Zsh => "zsh",
        Shell::Fish => "fish",
        Shell::Ksh => "ksh",
        Shell::Csh => "csh",
        Shell::Nushell => "nu",
        Shell::Powershell => "pwsh",
        Shell::Cmd => "cmd",
    };

    // Spawn a new shell process with the virtual environment activated.
    let status = Command::new(shell_exe)
        .env("PATH", &new_path)
        .env("VIRTUAL_ENV", venv.root())
        .env_remove("PYTHONHOME")
        .status()
        .with_context(|| format!("Failed to spawn shell: {shell_exe}"))?;

    if status.success() {
        Ok(ExitStatus::Success)
    } else {
        let code = u8::try_from(status.code().unwrap_or(1)).unwrap_or(1);
        Ok(ExitStatus::External(code))
    }
}
