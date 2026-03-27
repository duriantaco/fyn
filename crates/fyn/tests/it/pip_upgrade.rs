use anyhow::Result;
use assert_fs::prelude::*;

use fyn_test::fyn_snapshot;

#[test]
fn no_arguments() {
    let context = fyn_test::test_context!("3.12");

    fyn_snapshot!(context.filters(), context.pip_upgrade(), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the following required arguments were not provided:
      --all

    Usage: fyn pip upgrade --all --cache-dir [CACHE_DIR] --exclude-newer <EXCLUDE_NEWER>

    For more information, try '--help'.
    "
    );
}

#[test]
fn upgrade_all_empty_target() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let target = context.temp_dir.child("target");
    target.create_dir_all()?;

    fyn_snapshot!(context.filters(), context.pip_upgrade()
        .arg("--all")
        .arg("--target")
        .arg(target.path()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Using CPython 3.12.[X] interpreter at: .venv/bin/python3
    Nothing to upgrade
    "
    );

    Ok(())
}

#[test]
fn upgrade_all_registry_packages() {
    let context = fyn_test::test_context!("3.12")
        .with_filtered_counts()
        .with_filtered_exe_suffix();

    fyn_snapshot!(context.filters(), context.pip_install()
        .arg("babel")
        .arg("--index-url")
        .arg("https://test.pypi.org/simple/"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + babel==2.6.0
     + pytz==2018.5
    "
    );

    fyn_snapshot!(context.filters(), context.pip_upgrade()
        .arg("--all")
        .arg("--index-url")
        .arg("https://pypi.org/simple/"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Uninstalled [N] packages in [TIME]
    Installed [N] packages in [TIME]
     - babel==2.6.0
     + babel==2.14.0
     - pytz==2018.5
     + pytz==2024.1
    "
    );
}

#[test]
fn upgrade_all_preserves_editable_installs() {
    let context = fyn_test::test_context!("3.12").with_filtered_counts();

    fyn_snapshot!(context.filters(), context.pip_install()
        .arg("-e")
        .arg(context.workspace_root.join("test/packages/poetry_editable")), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Prepared [N] packages in [TIME]
    Installed [N] packages in [TIME]
     + anyio==4.3.0
     + idna==3.6
     + poetry-editable==0.1.0 (from file://[WORKSPACE]/test/packages/poetry_editable)
     + sniffio==1.3.1
    "
    );

    fyn_snapshot!(context.filters(), context.pip_upgrade()
        .arg("--all"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Resolved [N] packages in [TIME]
    Checked [N] packages in [TIME]
    "
    );

    fyn_snapshot!(context.filters(), context.pip_freeze(), @"
    success: true
    exit_code: 0
    ----- stdout -----
    anyio==4.3.0
    idna==3.6
    -e file://[WORKSPACE]/test/packages/poetry_editable
    sniffio==1.3.1

    ----- stderr -----
    "
    );
}
