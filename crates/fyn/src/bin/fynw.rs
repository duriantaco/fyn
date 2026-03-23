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
        fyn_windows::spawn_child(cmd, true)
    }
}

/// Assuming the binary is called something like `fynw@1.2.3(.exe)`, compute the `@1.2.3(.exe)` part
/// so that we can preferentially find `fyn@1.2.3(.exe)`, for folks who like managing multiple
/// installs in this way.
fn get_fynw_suffix(current_exe: &Path) -> Option<&str> {
    let os_file_name = current_exe.file_name()?;
    let file_name_str = os_file_name.to_str()?;
    file_name_str.strip_prefix("fynw")
}

/// Gets the path to `fyn`, given info about `fynw`
fn get_fyn_path(current_exe_parent: &Path, fynw_suffix: Option<&str>) -> std::io::Result<PathBuf> {
    // First try to find a matching suffixed `fyn`, e.g. `fyn@1.2.3(.exe)`
    let fyn_with_suffix = fynw_suffix.map(|suffix| current_exe_parent.join(format!("fyn{suffix}")));
    if let Some(fyn_with_suffix) = &fyn_with_suffix {
        #[expect(clippy::print_stderr, reason = "printing a very rare warning")]
        match fyn_with_suffix.try_exists() {
            Ok(true) => return Ok(fyn_with_suffix.to_owned()),
            Ok(false) => { /* definitely not there, proceed to fallback */ }
            Err(err) => {
                // We don't know if `fyn@1.2.3` exists, something errored when checking.
                // We *could* blindly use `fyn@1.2.3` in this case, as the code below does, however
                // in this extremely narrow corner case it's *probably* better to default to `fyn`,
                // since we don't want to mess up existing users who weren't using suffixes?
                eprintln!(
                    "warning: failed to determine if `{}` exists, trying `fyn` instead: {err}",
                    fyn_with_suffix.display()
                );
            }
        }
    }

    // Then just look for good ol' `fyn`
    let fyn = current_exe_parent.join(format!("fyn{}", std::env::consts::EXE_SUFFIX));
    // If we are sure the `fyn` binary does not exist, display a clearer error message.
    // If we're not certain if fyn exists (try_exists == Err), keep going and hope it works.
    if matches!(fyn.try_exists(), Ok(false)) {
        let message = if let Some(fyn_with_suffix) = fyn_with_suffix {
            format!(
                "Could not find the `fyn` binary at either of:\n  {}\n  {}",
                fyn_with_suffix.display(),
                fyn.display(),
            )
        } else {
            format!("Could not find the `fyn` binary at: {}", fyn.display())
        };
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, message))
    } else {
        Ok(fyn)
    }
}

fn run() -> std::io::Result<ExitStatus> {
    let current_exe = std::env::current_exe()?;
    let Some(bin) = current_exe.parent() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not determine the location of the `fynw` binary",
        ));
    };
    let fynw_suffix = get_fynw_suffix(&current_exe);
    let fyn = get_fyn_path(bin, fynw_suffix)?;
    let args = std::env::args_os()
        // Skip the `fynw` name
        .skip(1)
        .collect::<Vec<_>>();

    let mut cmd = Command::new(fyn);
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
