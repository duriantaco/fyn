use std::path::PathBuf;

use anyhow::Result;
use assert_fs::prelude::*;

use fyn_static::EnvVars;
use fyn_test::fyn_snapshot;

#[test]
fn no_arguments() {
    let context = fyn_test::test_context!("3.12");

    fyn_snapshot!(context.filters(), context.pip_wheel(), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the following required arguments were not provided:
      <PACKAGE|--requirements <REQUIREMENTS>|--group <GROUP>>

    Usage: fyn pip wheel --cache-dir [CACHE_DIR] --exclude-newer <EXCLUDE_NEWER> <PACKAGE|--requirements <REQUIREMENTS>|--group <GROUP>>

    For more information, try '--help'.
    "
    );
}

#[test]
fn wheel_from_find_links() -> Result<()> {
    let context = fyn_test::test_context!("3.12").with_filtered_counts();
    let find_links = PathBuf::from(&context.workspace_root).join("test/links");
    let dest = context.temp_dir.child("wheels");
    dest.create_dir_all()?;

    fyn_snapshot!(context.filters(), context.pip_wheel()
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg("ok")
        .arg("--no-index")
        .arg("--find-links")
        .arg(find_links)
        .arg("--wheel-dir")
        .arg(dest.path()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Saved wheels/ok-2.0.0-py3-none-any.whl
    Saved 1 wheel in [TIME]
    "
    );

    dest.child("ok-2.0.0-py3-none-any.whl")
        .assert(predicates::path::exists());
    Ok(())
}

#[test]
fn wheel_local_source_tree() -> Result<()> {
    let context = fyn_test::test_context!("3.12").with_filtered_counts();
    let source_tree = PathBuf::from(&context.workspace_root).join("test/packages/built-by-uv");
    let dest = context.temp_dir.child("wheels");
    dest.create_dir_all()?;

    fyn_snapshot!(context.filters(), context.pip_wheel()
        .arg(source_tree)
        .arg("--no-deps")
        .arg("--wheel-dir")
        .arg(dest.path()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Saved wheels/built_by_uv-0.1.0-py3-none-any.whl
    Saved 1 wheel in [TIME]
    "
    );

    dest.child("built_by_uv-0.1.0-py3-none-any.whl")
        .assert(predicates::path::exists());
    Ok(())
}

#[test]
fn wheel_local_wheel_path() -> Result<()> {
    let context = fyn_test::test_context!("3.12").with_filtered_counts();
    let wheel = PathBuf::from(&context.workspace_root).join("test/links/ok-2.0.0-py3-none-any.whl");
    let dest = context.temp_dir.child("wheels");
    dest.create_dir_all()?;

    fyn_snapshot!(context.filters(), context.pip_wheel()
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .arg(wheel)
        .arg("--wheel-dir")
        .arg(dest.path()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Saved wheels/ok-2.0.0-py3-none-any.whl
    Saved 1 wheel in [TIME]
    "
    );

    dest.child("ok-2.0.0-py3-none-any.whl")
        .assert(predicates::path::exists());
    Ok(())
}
