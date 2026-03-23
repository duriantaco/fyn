#![cfg_attr(windows, windows_subsystem = "windows")]

use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, ExitStatus};

/// Spawns a command exec style.
fn exec_spawn(cmd: &mut Command) -> std::io::Result<Infallible> {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let err = cmd.exec();
        Err(err)
    }
    #[cfg(windows)]
    {
        fv_windows::spawn_child(cmd, true)
    }
}

/// Assuming the binary is called something like `fvw@1.2.3(.exe)`, compute the `@1.2.3(.exe)` part
/// so that we can preferentially find `fv@1.2.3(.exe)`, for folks who like managing multiple
/// installs in this way.
fn get_fvw_suffix(current_exe: &Path) -> Option<&str> {
    let os_file_name = current_exe.file_name()?;
    let file_name_str = os_file_name.to_str()?;
    file_name_str.strip_prefix("fvw")
}

/// Gets the path to `fv`, given info about `fvw`
fn get_fv_path(current_exe_parent: &Path, fvw_suffix: Option<&str>) -> std::io::Result<PathBuf> {
    // First try to find a matching suffixed `fv`, e.g. `fv@1.2.3(.exe)`
    let fv_with_suffix = fvw_suffix.map(|suffix| current_exe_parent.join(format!("fv{suffix}")));
    if let Some(fv_with_suffix) = &fv_with_suffix {
        #[expect(clippy::print_stderr, reason = "printing a very rare warning")]
        match fv_with_suffix.try_exists() {
            Ok(true) => return Ok(fv_with_suffix.to_owned()),
            Ok(false) => { /* definitely not there, proceed to fallback */ }
            Err(err) => {
                // We don't know if `fv@1.2.3` exists, something errored when checking.
                // We *could* blindly use `fv@1.2.3` in this case, as the code below does, however
                // in this extremely narrow corner case it's *probably* better to default to `fv`,
                // since we don't want to mess up existing users who weren't using suffixes?
                eprintln!(
                    "warning: failed to determine if `{}` exists, trying `fv` instead: {err}",
                    fv_with_suffix.display()
                );
            }
        }
    }

    // Then just look for good ol' `fv`
    let fv = current_exe_parent.join(format!("fv{}", std::env::consts::EXE_SUFFIX));
    // If we are sure the `fv` binary does not exist, display a clearer error message.
    // If we're not certain if fv exists (try_exists == Err), keep going and hope it works.
    if matches!(fv.try_exists(), Ok(false)) {
        let message = if let Some(fv_with_suffix) = fv_with_suffix {
            format!(
                "Could not find the `fv` binary at either of:\n  {}\n  {}",
                fv_with_suffix.display(),
                fv.display(),
            )
        } else {
            format!("Could not find the `fv` binary at: {}", fv.display())
        };
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, message))
    } else {
        Ok(fv)
    }
}

fn run() -> std::io::Result<ExitStatus> {
    let current_exe = std::env::current_exe()?;
    let Some(bin) = current_exe.parent() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not determine the location of the `fvw` binary",
        ));
    };
    let fvw_suffix = get_fvw_suffix(&current_exe);
    let fv = get_fv_path(bin, fvw_suffix)?;
    let args = std::env::args_os()
        // Skip the `fvw` name
        .skip(1)
        .collect::<Vec<_>>();

    let mut cmd = Command::new(fv);
    cmd.args(&args);
    match exec_spawn(&mut cmd)? {}
}

#[expect(clippy::print_stderr)]
fn main() -> ExitCode {
    let result = run();
    match result {
        // Fail with 2 if the status cannot be cast to an exit code
        Ok(status) => u8::try_from(status.code().unwrap_or(2)).unwrap_or(2).into(),
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(2)
        }
    }
}
