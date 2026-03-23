use std::fmt::Write;

use anyhow::Result;
use owo_colors::OwoColorize;

use fv_client::BaseClientBuilder;

use crate::commands::ExitStatus;
use crate::printer::Printer;

/// Self-update is not yet available for fv.
pub(crate) async fn self_update(
    _version: Option<String>,
    _token: Option<String>,
    _dry_run: bool,
    printer: Printer,
    _client_builder: BaseClientBuilder<'_>,
) -> Result<ExitStatus> {
    writeln!(
        printer.stderr(),
        "{}",
        format_args!(
            "{}{} Self-update is not yet available for fv. Please update manually.",
            "error".red().bold(),
            ":".bold()
        )
    )?;
    Ok(ExitStatus::Error)
}
