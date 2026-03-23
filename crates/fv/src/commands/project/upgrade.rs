use std::fmt::Write;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::commands::ExitStatus;
use crate::printer::Printer;

/// Upgrade project dependencies by re-locking with --upgrade semantics and syncing.
///
/// This is a convenience command that combines `fv lock --upgrade` and `fv sync`.
pub(crate) async fn upgrade(
    packages: Vec<String>,
    dry_run: bool,
    no_sync: bool,
    printer: Printer,
) -> Result<ExitStatus> {
    if packages.is_empty() {
        writeln!(
            printer.stderr(),
            "{}{} Upgrading all dependencies...",
            "info".cyan().bold(),
            ":".bold()
        )?;
    } else {
        writeln!(
            printer.stderr(),
            "{}{} Upgrading: {}",
            "info".cyan().bold(),
            ":".bold(),
            packages.join(", ").green()
        )?;
    }

    if dry_run {
        writeln!(
            printer.stderr(),
            "{}{} Dry run — no changes will be made.",
            "info".cyan().bold(),
            ":".bold()
        )?;
    }

    // Re-exec ourselves with `lock --upgrade`. This inherits all env vars
    // (UV_CACHE_DIR, UV_OFFLINE, etc.) and the working directory, so the
    // child process picks up the same settings the user set.
    let exe = std::env::current_exe()?;

    let mut lock_args = vec!["lock".to_string()];
    if packages.is_empty() {
        lock_args.push("--upgrade".to_string());
    } else {
        for pkg in &packages {
            lock_args.push("--upgrade-package".to_string());
            lock_args.push(pkg.clone());
        }
    }
    if dry_run {
        lock_args.push("--dry-run".to_string());
    }

    // Forward any global flags that were set via env vars — the child process
    // reads them automatically since we don't clear the environment.
    let lock_status = std::process::Command::new(&exe).args(&lock_args).status()?;

    if !lock_status.success() {
        writeln!(
            printer.stderr(),
            "{}{} Lock failed",
            "error".red().bold(),
            ":".bold()
        )?;
        return Ok(ExitStatus::Failure);
    }

    if no_sync || dry_run {
        writeln!(
            printer.stderr(),
            "{}{} Lockfile updated (sync skipped).",
            "success".green().bold(),
            ":".bold()
        )?;
        return Ok(ExitStatus::Success);
    }

    let sync_status = std::process::Command::new(&exe).arg("sync").status()?;

    if !sync_status.success() {
        writeln!(
            printer.stderr(),
            "{}{} Sync failed",
            "error".red().bold(),
            ":".bold()
        )?;
        return Ok(ExitStatus::Failure);
    }

    writeln!(
        printer.stderr(),
        "{}{} Dependencies upgraded successfully.",
        "success".green().bold(),
        ":".bold()
    )?;

    Ok(ExitStatus::Success)
}
