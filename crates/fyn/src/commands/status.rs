use std::fmt::Write;
use std::path::Path;

use fyn_cache::Cache;
use fyn_fs::Simplified;
use fyn_preview::Preview;
use fyn_python::{EnvironmentPreference, PythonEnvironment, PythonPreference, PythonRequest};
use fyn_settings::{FilesystemOptions, PipInProjectPolicy};
use fyn_workspace::{DiscoveryOptions, Workspace, WorkspaceCache};
use owo_colors::OwoColorize;

use crate::commands::ExitStatus;
use crate::printer::Printer;

fn pip_in_project_policy(filesystem: Option<&FilesystemOptions>) -> PipInProjectPolicy {
    filesystem
        .and_then(|filesystem| filesystem.pip.as_ref().and_then(|pip| pip.pip_in_project))
        .unwrap_or_default()
}

fn policy_name(policy: PipInProjectPolicy) -> &'static str {
    match policy {
        PipInProjectPolicy::Allow => "allow",
        PipInProjectPolicy::Warn => "warn",
        PipInProjectPolicy::Error => "error",
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

/// Show the current project and environment status.
pub(crate) async fn status(
    project_dir: &Path,
    filesystem: Option<&FilesystemOptions>,
    python_preference: PythonPreference,
    cache: &Cache,
    workspace_cache: &WorkspaceCache,
    printer: Printer,
    preview: Preview,
) -> anyhow::Result<ExitStatus> {
    let current_dir = std::env::current_dir()?;
    let workspace = Workspace::discover(project_dir, &DiscoveryOptions::default(), workspace_cache)
        .await
        .ok();
    let root = workspace
        .as_ref()
        .map(|workspace| workspace.install_path().as_path())
        .unwrap_or(project_dir);

    writeln!(
        printer.stdout(),
        "current directory: {}",
        current_dir.simplified_display().cyan()
    )?;
    writeln!(
        printer.stdout(),
        "project directory: {}",
        project_dir.simplified_display().cyan()
    )?;
    writeln!(
        printer.stdout(),
        "managed project: {}",
        yes_no(workspace.is_some()).cyan()
    )?;
    writeln!(
        printer.stdout(),
        "workspace root: {}",
        if workspace.is_some() {
            root.simplified_display().to_string().cyan().to_string()
        } else {
            "none".dimmed().to_string()
        }
    )?;
    writeln!(
        printer.stdout(),
        "pyproject.toml: {}",
        yes_no(root.join("pyproject.toml").is_file()).cyan()
    )?;
    writeln!(
        printer.stdout(),
        "fyn.lock: {}",
        yes_no(root.join("fyn.lock").is_file()).cyan()
    )?;
    writeln!(
        printer.stdout(),
        "pip-in-project: {}",
        policy_name(pip_in_project_policy(filesystem)).cyan()
    )?;

    if let Ok(environment) = PythonEnvironment::find(
        &PythonRequest::default(),
        EnvironmentPreference::from_system_flag(false, false),
        python_preference,
        cache,
        preview,
    ) {
        writeln!(
            printer.stdout(),
            "environment: {}",
            environment.root().simplified_display().cyan()
        )?;
        writeln!(
            printer.stdout(),
            "python: {} ({})",
            environment.python_executable().simplified_display().cyan(),
            environment.interpreter().python_full_version().cyan()
        )?;
    } else {
        writeln!(printer.stdout(), "environment: {}", "not found".dimmed())?;
        writeln!(printer.stdout(), "python: {}", "not found".dimmed())?;
    }

    Ok(ExitStatus::Success)
}
