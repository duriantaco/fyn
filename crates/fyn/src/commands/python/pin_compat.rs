use anyhow::Result;
use fyn_cache::Cache;
use fyn_configuration::DependencyGroupsWithDefaults;
use fyn_distribution_types::RequiresPython;
use fyn_preview::Preview;
use fyn_python::downloads::ManagedPythonDownloadList;
use fyn_python::{EnvironmentPreference, PythonInstallation, PythonPreference, PythonRequest};
use fyn_workspace::VirtualProject;
use tracing::debug;

use crate::commands::project::find_requires_python;

/// Utility struct for representing pins in error messages.
struct Pin<'a> {
    request: &'a PythonRequest,
    version: &'a fyn_pep440::Version,
    resolved: bool,
    existing: bool,
}

/// Check if an existing pin is compatible with the workspace/project's `requires-python`.
pub(crate) fn existing_pin_compatibility_issue(
    pin: &PythonRequest,
    virtual_project: &VirtualProject,
    python_preference: PythonPreference,
    downloads_list: &ManagedPythonDownloadList,
    cache: &Cache,
    preview: Preview,
) -> Option<String> {
    // Check if the pinned version is compatible with the project.
    if let Some(pin_version) = pin.as_pep440_version() {
        if let Err(err) =
            assert_pin_compatible_with_project(pin, &pin_version, false, true, virtual_project)
        {
            return Some(err.to_string());
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
            assert_pin_compatible_with_project(pin, python_version, true, true, virtual_project)
                .err()
                .map(|err| err.to_string())
        }
        Err(err) => Some(format!(
            "Failed to resolve pinned Python version `{}`: {err}",
            pin.to_canonical_string(),
        )),
    }
}

pub(crate) fn project_requires_python(
    virtual_project: &VirtualProject,
) -> Result<Option<RequiresPython>> {
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

/// Checks if the pinned Python version is compatible with the workspace/project's
/// `requires-python`.
pub(crate) fn assert_pin_compatible_with_project(
    request: &PythonRequest,
    version: &fyn_pep440::Version,
    resolved: bool,
    existing: bool,
    virtual_project: &VirtualProject,
) -> Result<()> {
    let pin = Pin {
        request,
        version,
        resolved,
        existing,
    };
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
