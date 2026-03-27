use std::path::PathBuf;

use anyhow::Result;
use assert_fs::prelude::*;

use fyn_static::EnvVars;
use fyn_test::fyn_snapshot;

#[test]
fn no_arguments() {
    let context = fyn_test::test_context!("3.12");

    fyn_snapshot!(context.filters(), context.pip_download(), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the following required arguments were not provided:
      <PACKAGE|--requirements <REQUIREMENTS>|--group <GROUP>>

    Usage: fyn pip download --cache-dir [CACHE_DIR] --exclude-newer <EXCLUDE_NEWER> <PACKAGE|--requirements <REQUIREMENTS>|--group <GROUP>>

    For more information, try '--help'.
    "
    );
}

#[test]
fn download_from_find_links() -> Result<()> {
    let context = fyn_test::test_context!("3.12").with_filtered_counts();
    let find_links = PathBuf::from(&context.workspace_root).join("test/links");
    let dest = context.temp_dir.child("downloads");
    dest.create_dir_all()?;

    fyn_snapshot!(context.filters(), context.pip_download()
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("ok")
        .arg("--no-index")
        .arg("--find-links")
        .arg(find_links)
        .arg("--dest")
        .arg(dest.path()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Saved downloads/ok-2.0.0-py3-none-any.whl
    Downloaded 1 file in [TIME]
    "
    );

    dest.child("ok-2.0.0-py3-none-any.whl")
        .assert(predicates::path::exists());
    Ok(())
}

#[test]
fn download_sdist_with_no_binary() -> Result<()> {
    let context = fyn_test::test_context!("3.12").with_filtered_counts();
    let find_links = PathBuf::from(&context.workspace_root).join("test/links");
    let dest = context.temp_dir.child("downloads");
    dest.create_dir_all()?;

    fyn_snapshot!(context.filters(), context.pip_download()
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("tqdm")
        .arg("--no-index")
        .arg("--find-links")
        .arg(find_links)
        .arg("--no-binary")
        .arg(":all:")
        .arg("--no-deps")
        .arg("--dest")
        .arg(dest.path()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Saved downloads/tqdm-999.0.0.tar.gz
    Downloaded 1 file in [TIME]
    "
    );

    dest.child("tqdm-999.0.0.tar.gz")
        .assert(predicates::path::exists());
    Ok(())
}

#[test]
fn download_local_wheel_path() -> Result<()> {
    let context = fyn_test::test_context!("3.12").with_filtered_counts();
    let wheel = PathBuf::from(&context.workspace_root).join("test/links/ok-2.0.0-py3-none-any.whl");
    let dest = context.temp_dir.child("downloads");
    dest.create_dir_all()?;

    fyn_snapshot!(context.filters(), context.pip_download()
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg(wheel)
        .arg("--dest")
        .arg(dest.path()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Saved downloads/ok-2.0.0-py3-none-any.whl
    Downloaded 1 file in [TIME]
    "
    );

    dest.child("ok-2.0.0-py3-none-any.whl")
        .assert(predicates::path::exists());
    Ok(())
}
