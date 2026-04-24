use std::fmt::Write;
use std::path::Path;

use fyn_cache::Cache;
use fyn_client::BaseClientBuilder;
use fyn_fs::Simplified;
use fyn_preview::Preview;
use fyn_python::downloads::ManagedPythonDownloadList;
use fyn_python::{
    EnvironmentPreference, PythonEnvironment, PythonPreference, PythonRequest, PythonVersionFile,
    VersionFileDiscoveryOptions,
};
use fyn_settings::{FilesystemOptions, PipInProjectPolicy, PythonInstallMirrors};
use fyn_workspace::{DiscoveryOptions, VirtualProject, Workspace, WorkspaceCache};
use owo_colors::OwoColorize;
use serde::Serialize;
use tracing::debug;

use crate::commands::ExitStatus;
use crate::commands::python::pin_compat::{
    assert_pin_compatible_with_project, existing_pin_compatibility_issue,
};
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
struct StatusCheck {
    passed: bool,
    issues: Vec<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    check: Option<StatusCheck>,
}

#[derive(Clone, Copy)]
struct StatusIssueInputs {
    managed_project: bool,
    pyproject_toml: bool,
    fyn_lock: bool,
    environment_found: bool,
}

fn status_issues(inputs: StatusIssueInputs) -> Vec<String> {
    let mut issues = Vec::new();
    if !inputs.managed_project {
        issues.push("not inside a managed project".to_string());
        return issues;
    }
    if !inputs.pyproject_toml {
        issues.push("pyproject.toml not found in workspace root".to_string());
    }
    if !inputs.fyn_lock {
        issues.push("fyn.lock not found in workspace root".to_string());
    }
    if !inputs.environment_found {
        issues.push("environment not found".to_string());
    }
    issues
}

async fn python_pin_issues(
    project_dir: &Path,
    virtual_project: Option<&VirtualProject>,
    python_preference: PythonPreference,
    install_mirrors: &PythonInstallMirrors,
    client_builder: &BaseClientBuilder<'_>,
    cache: &Cache,
    preview: Preview,
) -> Vec<String> {
    let Some(virtual_project) = virtual_project else {
        return Vec::new();
    };

    let version_file =
        match PythonVersionFile::discover(project_dir, &VersionFileDiscoveryOptions::default())
            .await
        {
            Ok(version_file) => version_file,
            Err(err) => {
                debug!("Failed to discover Python version file: {err}");
                return Vec::new();
            }
        };
    let Some(version_file) = version_file else {
        return Vec::new();
    };

    let pins: Vec<_> = version_file.versions().collect();
    let download_list = if pins.is_empty() {
        None
    } else {
        let client = match client_builder.clone().retries(0).build() {
            Ok(client) => client,
            Err(err) => {
                debug!("Failed to create client for Python pin compatibility checks: {err}");
                return Vec::new();
            }
        };
        match ManagedPythonDownloadList::new(
            &client,
            install_mirrors.python_downloads_json_url.as_deref(),
        )
        .await
        {
            Ok(download_list) => Some(download_list),
            Err(err) => {
                debug!("Failed to load Python downloads metadata for status checks: {err}");
                None
            }
        }
    };

    pins.into_iter()
        .filter_map(|pin| {
            download_list
                .as_ref()
                .and_then(|download_list| {
                    existing_pin_compatibility_issue(
                        pin,
                        virtual_project,
                        python_preference,
                        download_list,
                        cache,
                        preview,
                    )
                })
                .or_else(|| {
                    pin.as_pep440_version().and_then(|pin_version| {
                        assert_pin_compatible_with_project(
                            pin,
                            &pin_version,
                            false,
                            true,
                            virtual_project,
                        )
                        .err()
                        .map(|err| err.to_string())
                    })
                })
        })
        .collect()
}

/// Show the current project and environment status.
pub(crate) async fn status(
    check: bool,
    json: bool,
    project_dir: &Path,
    filesystem: Option<&FilesystemOptions>,
    python_preference: PythonPreference,
    install_mirrors: &PythonInstallMirrors,
    client_builder: &BaseClientBuilder<'_>,
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
    let managed_project = workspace.is_some();
    let workspace_root = workspace
        .as_ref()
        .map(|workspace| workspace.install_path().simplified_display().to_string());
    let pyproject_toml = root.join("pyproject.toml").is_file();
    let fyn_lock = root.join("fyn.lock").is_file();
    let virtual_project = if check && managed_project {
        match VirtualProject::discover(project_dir, &DiscoveryOptions::default(), workspace_cache)
            .await
        {
            Ok(virtual_project) => Some(virtual_project),
            Err(err) => {
                debug!("Failed to discover virtual project for status checks: {err}");
                None
            }
        }
    } else {
        None
    };
    let issues = if check {
        let mut issues = status_issues(StatusIssueInputs {
            managed_project,
            pyproject_toml,
            fyn_lock,
            environment_found: environment.is_some(),
        });
        issues.extend(
            python_pin_issues(
                project_dir,
                virtual_project.as_ref(),
                python_preference,
                install_mirrors,
                client_builder,
                cache,
                preview,
            )
            .await,
        );
        issues
    } else {
        Vec::new()
    };
    let check_passed = issues.is_empty();
    let report = StatusReport {
        current_directory: current_dir.simplified_display().to_string(),
        project_directory: project_dir.simplified_display().to_string(),
        managed_project,
        workspace_root,
        pyproject_toml,
        fyn_lock,
        pip_in_project: policy_name(pip_in_project_policy(filesystem)),
        environment: environment.as_ref().map(|environment| StatusEnvironment {
            path: environment.root().simplified_display().to_string(),
            python: environment
                .python_executable()
                .simplified_display()
                .to_string(),
            version: environment.interpreter().python_full_version().to_string(),
        }),
        check: check.then_some(StatusCheck {
            passed: check_passed,
            issues: issues.clone(),
        }),
    };

    if json {
        writeln!(
            printer.stdout(),
            "{}",
            serde_json::to_string_pretty(&report)?
        )?;
        return Ok(if check && !check_passed {
            ExitStatus::Failure
        } else {
            ExitStatus::Success
        });
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

    if check {
        writeln!(
            printer.stdout(),
            "check: {}",
            if check_passed {
                "ok".green().to_string()
            } else {
                "failed".yellow().to_string()
            }
        )?;
        for issue in &issues {
            writeln!(printer.stdout(), "issue: {}", issue.yellow())?;
        }
    }

    Ok(if check && !check_passed {
        ExitStatus::Failure
    } else {
        ExitStatus::Success
    })
}
