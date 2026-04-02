use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use anyhow::Result;
use serde::Serialize;

use fyn_fs::PortablePathBuf;
use fyn_normalize::PackageName;
use fyn_preview::{Preview, PreviewFeature};
use fyn_resolver::{Lock, VERSION};
use fyn_warnings::warn_user;
use fyn_workspace::{DiscoveryOptions, Workspace, WorkspaceCache};

use crate::commands::ExitStatus;
use crate::printer::Printer;

/// The schema version for the metadata report.
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
enum SchemaVersion {
    /// An unstable, experimental schema.
    #[default]
    Preview,
}

/// The schema metadata for the metadata report.
#[derive(Serialize, Debug, Default)]
struct SchemaReport {
    /// The version of the schema.
    version: SchemaVersion,
}

/// Report for a single workspace member.
#[derive(Serialize, Debug)]
struct WorkspaceMemberReport {
    /// The name of the workspace member.
    name: PackageName,
    /// The path to the workspace member's root directory.
    path: PortablePathBuf,
    /// Direct dependencies on other workspace members, if available from the lockfile.
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<Vec<PackageName>>,
}

/// The report for a metadata operation.
#[derive(Serialize, Debug)]
struct MetadataReport {
    /// The schema of this report.
    schema: SchemaReport,
    /// The workspace root directory.
    workspace_root: PortablePathBuf,
    /// The workspace members.
    members: Vec<WorkspaceMemberReport>,
}

/// Display metadata about the workspace.
pub(crate) async fn metadata(
    project_dir: &Path,
    preview: Preview,
    workspace_cache: &WorkspaceCache,
    printer: Printer,
) -> Result<ExitStatus> {
    if !preview.is_enabled(PreviewFeature::WorkspaceMetadata) {
        warn_user!(
            "The `fyn workspace metadata` command is experimental and may change without warning. Pass `--preview-features {}` to disable this warning.",
            PreviewFeature::WorkspaceMetadata
        );
    }

    let workspace =
        Workspace::discover(project_dir, &DiscoveryOptions::default(), workspace_cache).await?;
    let member_dependencies = read_workspace_member_dependencies(&workspace).await;

    let members = workspace
        .packages()
        .values()
        .map(|package| {
            let name = package.project().name.clone();
            WorkspaceMemberReport {
                path: PortablePathBuf::from(package.root().as_path()),
                dependencies: member_dependencies.as_ref().map(|member_dependencies| {
                    member_dependencies.get(&name).cloned().unwrap_or_default()
                }),
                name,
            }
        })
        .collect();

    let report = MetadataReport {
        schema: SchemaReport::default(),
        workspace_root: PortablePathBuf::from(workspace.install_path().as_path()),
        members,
    };

    writeln!(
        printer.stdout(),
        "{}",
        serde_json::to_string_pretty(&report)?
    )?;

    Ok(ExitStatus::Success)
}

async fn read_workspace_member_dependencies(
    workspace: &Workspace,
) -> Option<BTreeMap<PackageName, Vec<PackageName>>> {
    let lock_path = workspace.install_path().join("fyn.lock");
    let Ok(encoded) = fs_err::tokio::read_to_string(lock_path).await else {
        return None;
    };
    let Ok(lock) = toml::from_str::<Lock>(&encoded) else {
        return None;
    };
    if lock.version() != VERSION {
        return None;
    }
    Some(workspace_member_dependencies(workspace, &lock))
}

fn workspace_member_dependencies(
    workspace: &Workspace,
    lock: &Lock,
) -> BTreeMap<PackageName, Vec<PackageName>> {
    let workspace_members = workspace
        .packages()
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();

    workspace
        .packages()
        .keys()
        .cloned()
        .map(|name| {
            let dependencies = lock
                .find_by_name(&name)
                .ok()
                .flatten()
                .map(|package| {
                    package
                        .dependencies()
                        .iter()
                        .filter_map(|dependency| {
                            let dependency = dependency.package_name();
                            workspace_members
                                .contains(dependency)
                                .then_some(dependency.clone())
                        })
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .collect()
                })
                .unwrap_or_default();
            (name, dependencies)
        })
        .collect()
}
