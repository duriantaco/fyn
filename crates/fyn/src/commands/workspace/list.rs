use std::fmt::Write;
use std::path::Path;

use anyhow::Result;

use fyn_fs::Simplified;
use fyn_workspace::{DiscoveryOptions, Workspace, WorkspaceCache};
use owo_colors::OwoColorize;

use crate::commands::ExitStatus;
use crate::printer::Printer;

/// List workspace members
pub(crate) async fn list(
    project_dir: &Path,
    paths: bool,
    workspace_cache: &WorkspaceCache,
    printer: Printer,
) -> Result<ExitStatus> {
    let workspace =
        Workspace::discover(project_dir, &DiscoveryOptions::default(), workspace_cache).await?;

    for (name, member) in workspace.packages() {
        if paths {
            writeln!(
                printer.stdout(),
                "{}",
                member.root().simplified_display().cyan()
            )?;
        } else {
            writeln!(printer.stdout(), "{}", name.cyan())?;
        }
    }

    Ok(ExitStatus::Success)
}
