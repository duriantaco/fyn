use owo_colors::OwoColorize;
use std::fmt::Write;

use fyn_cache::Cache;
use fyn_fs::Simplified;

use crate::commands::ExitStatus;
use crate::printer::Printer;

/// Show the cache directory.
pub(crate) fn cache_dir(cache: &Cache, printer: Printer) -> anyhow::Result<ExitStatus> {
    writeln!(
        printer.stdout(),
        "{}",
        cache.root().simplified_display().cyan()
    )?;
    Ok(ExitStatus::Success)
}
