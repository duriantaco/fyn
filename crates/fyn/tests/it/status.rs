use anyhow::Result;
use assert_fs::prelude::*;

use fyn_test::fyn_snapshot;

#[test]
fn status_in_managed_project() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    context
        .temp_dir
        .child("pyproject.toml")
        .write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.1.0"
    "#})?;
    context.temp_dir.child("fyn.lock").touch()?;

    fyn_snapshot!(context.filters(), context.command().arg("status"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    current directory: [TEMP_DIR]/
    project directory: [TEMP_DIR]/
    managed project: yes
    workspace root: [TEMP_DIR]/
    pyproject.toml: yes
    fyn.lock: yes
    pip-in-project: warn
    environment: [VENV]/
    python: [VENV]/bin/python3 (3.12.[X])

    ----- stderr -----
    "
    );

    Ok(())
}

#[test]
fn status_in_unmanaged_directory() {
    let context = fyn_test::test_context!("3.12");

    fyn_snapshot!(context.filters(), context.command().arg("status"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    current directory: [TEMP_DIR]/
    project directory: [TEMP_DIR]/
    managed project: no
    workspace root: none
    pyproject.toml: no
    fyn.lock: no
    pip-in-project: warn
    environment: [VENV]/
    python: [VENV]/bin/python3 (3.12.[X])

    ----- stderr -----
    "
    );
}

#[test]
fn status_json_in_managed_project() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    context
        .temp_dir
        .child("pyproject.toml")
        .write_str(indoc::indoc! {r#"
        [project]
        name = "example"
        version = "0.1.0"
    "#})?;
    context.temp_dir.child("fyn.lock").touch()?;

    fyn_snapshot!(
        context.filters(),
        context
            .command()
            .arg("status")
            .arg("--json"),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "current_directory": "[TEMP_DIR]/",
      "project_directory": "[TEMP_DIR]/",
      "managed_project": true,
      "workspace_root": "[TEMP_DIR]/",
      "pyproject_toml": true,
      "fyn_lock": true,
      "pip_in_project": "warn",
      "environment": {
        "path": "[VENV]/",
        "python": "[VENV]/bin/python3",
        "version": "3.12.[X]"
      }
    }

    ----- stderr -----
    "#
    );

    Ok(())
}

#[test]
fn status_json_in_unmanaged_directory() {
    let context = fyn_test::test_context!("3.12");

    fyn_snapshot!(
        context.filters(),
        context
            .command()
            .arg("status")
            .arg("--json"),
        @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "current_directory": "[TEMP_DIR]/",
      "project_directory": "[TEMP_DIR]/",
      "managed_project": false,
      "workspace_root": null,
      "pyproject_toml": false,
      "fyn_lock": false,
      "pip_in_project": "warn",
      "environment": {
        "path": "[VENV]/",
        "python": "[VENV]/bin/python3",
        "version": "3.12.[X]"
      }
    }

    ----- stderr -----
    "#
    );
}
