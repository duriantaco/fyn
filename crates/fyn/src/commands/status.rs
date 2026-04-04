use std::fmt::Write;
use std::path::Path;

use fyn_cache::Cache;
use fyn_fs::Simplified;
use fyn_preview::Preview;
use fyn_python::{EnvironmentPreference, PythonEnvironment, PythonPreference, PythonRequest};
use fyn_settings::{FilesystemOptions, PipInProjectPolicy};
use fyn_workspace::{DiscoveryOptions, Workspace, WorkspaceCache};
use owo_colors::OwoColorize;
use serde::Serialize;

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

#[derive(Serialize)]
struct StatusEnvironment {
    path: String,
    python: String,
    version: String,
}

#[derive(Serialize)]
struct StatusReport {
    current_directory: String,
    project_directory: String,
    managed_project: bool,
    workspace_root: Option<String>,
    pyproject_toml: bool,
    fyn_lock: bool,
    pip_in_project: &'static str,
    environment: Option<StatusEnvironment>,
}

/// Show the current project and environment status.
pub(crate) async fn status(
    json: bool,
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
    let environment = PythonEnvironment::find(
        &PythonRequest::default(),
        EnvironmentPreference::from_system_flag(false, false),
        python_preference,
        cache,
        preview,
    )
    .ok();
    let report = StatusReport {
        current_directory: current_dir.simplified_display().to_string(),
        project_directory: project_dir.simplified_display().to_string(),
        managed_project: workspace.is_some(),
        workspace_root: workspace
            .as_ref()
            .map(|workspace| workspace.install_path().simplified_display().to_string()),
        pyproject_toml: root.join("pyproject.toml").is_file(),
        fyn_lock: root.join("fyn.lock").is_file(),
        pip_in_project: policy_name(pip_in_project_policy(filesystem)),
        environment: environment.as_ref().map(|environment| StatusEnvironment {
            path: environment.root().simplified_display().to_string(),
            python: environment
                .python_executable()
                .simplified_display()
                .to_string(),
            version: environment.interpreter().python_full_version().to_string(),
        }),
    };

    if json {
        writeln!(
            printer.stdout(),
            "{}",
            serde_json::to_string_pretty(&report)?
        )?;
        return Ok(ExitStatus::Success);
    }

    writeln!(
        printer.stdout(),
        "current directory: {}",
        report.current_directory.cyan()
    )?;
    writeln!(
        printer.stdout(),
        "project directory: {}",
        report.project_directory.cyan()
    )?;
    writeln!(
        printer.stdout(),
        "managed project: {}",
        yes_no(report.managed_project).cyan()
    )?;
    writeln!(
        printer.stdout(),
        "workspace root: {}",
        if let Some(workspace_root) = report.workspace_root.as_ref() {
            workspace_root.cyan().to_string()
        } else {
            "none".dimmed().to_string()
        }
    )?;
    writeln!(
        printer.stdout(),
        "pyproject.toml: {}",
        yes_no(report.pyproject_toml).cyan()
    )?;
    writeln!(
        printer.stdout(),
        "fyn.lock: {}",
        yes_no(report.fyn_lock).cyan()
    )?;
    writeln!(
        printer.stdout(),
        "pip-in-project: {}",
        report.pip_in_project.cyan()
    )?;

    if let Some(environment) = report.environment {
        writeln!(printer.stdout(), "environment: {}", environment.path.cyan())?;
        writeln!(
            printer.stdout(),
            "python: {} ({})",
            environment.python.cyan(),
            environment.version.cyan()
        )?;
    } else {
        writeln!(printer.stdout(), "environment: {}", "not found".dimmed())?;
        writeln!(printer.stdout(), "python: {}", "not found".dimmed())?;
    }

    Ok(ExitStatus::Success)
}
