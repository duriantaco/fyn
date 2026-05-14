use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;
use assert_fs::prelude::*;
use indoc::indoc;

use fyn_test::fyn_snapshot;

#[test]
fn transitive_dependency_paths() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = [
            "pandas",
            "scikit-learn==1.4.1.post1",
        ]
    "#,
    )?;

    fyn_snapshot!(context.filters(), context.why().arg("numpy").arg("--universal"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    numpy is included because:
    project v0.1.0 -> pandas v2.2.1 -> numpy v1.26.4
    project v0.1.0 -> scikit-learn v1.4.1.post1 -> numpy v1.26.4
    project v0.1.0 -> scikit-learn v1.4.1.post1 -> scipy v1.12.0 -> numpy v1.26.4

    ----- stderr -----
    Resolved 11 packages in [TIME]
    ");

    fyn_snapshot!(context.filters(), context.why().arg("pandas").arg("--frozen").arg("--universal"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    pandas is included because:
    project v0.1.0 -> pandas v2.2.1

    ----- stderr -----
    ");

    Ok(())
}

#[test]
fn dependency_group_paths() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]

        [dependency-groups]
        foo = ["anyio"]
        bar = ["iniconfig"]
        dev = ["sniffio"]
        "#,
    )?;

    context.lock().assert().success();

    fyn_snapshot!(context.filters(), context.why().arg("sniffio").arg("--frozen"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    sniffio is included because:
    project v0.1.0 -> sniffio v1.3.1 (group: dev)

    ----- stderr -----
    ");

    fyn_snapshot!(context.filters(), context.why().arg("sniffio").arg("--frozen").arg("--no-dev"), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No dependency path found for `sniffio`
    ");

    fyn_snapshot!(context.filters(), context.why().arg("sniffio").arg("--frozen").arg("--group").arg("foo"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    sniffio is included because:
    project v0.1.0 -> anyio v4.3.0 (group: foo) -> sniffio v1.3.1
    project v0.1.0 -> sniffio v1.3.1 (group: dev)

    ----- stderr -----
    ");

    fyn_snapshot!(context.filters(), context.why().arg("sniffio").arg("--frozen").arg("--only-group").arg("foo"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    sniffio is included because:
    project v0.1.0 -> anyio v4.3.0 (group: foo) -> sniffio v1.3.1

    ----- stderr -----
    ");

    fyn_snapshot!(context.filters(), context.why().arg("sniffio").arg("--frozen").arg("--group").arg("missing"), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Group `missing` is not defined in any project's `dependency-groups` table
    ");

    Ok(())
}

#[test]
fn optional_and_extra_dependency_paths() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["flask[dotenv]"]

        [project.optional-dependencies]
        async = ["anyio"]
    "#,
    )?;

    fyn_snapshot!(context.filters(), context.why().arg("python-dotenv").arg("--universal"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    python-dotenv is included because:
    project v0.1.0 -> flask[dotenv] v3.0.2 -> python-dotenv v1.0.1 (extra: dotenv)

    ----- stderr -----
    Resolved 13 packages in [TIME]
    ");

    fyn_snapshot!(context.filters(), context.why().arg("sniffio").arg("--frozen").arg("--universal"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    sniffio is included because:
    project v0.1.0 -> anyio v4.3.0 (extra: async) -> sniffio v1.3.1

    ----- stderr -----
    ");

    Ok(())
}

#[test]
fn platform_filtered_paths() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["jupyter-client"]
    "#,
    )?;

    fyn_snapshot!(context.filters(), context.why().arg("pywin32").arg("--python-platform").arg("windows"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    pywin32 is included because:
    project v0.1.0 -> jupyter-client v8.6.1 -> jupyter-core v5.7.2 -> pywin32 v306

    ----- stderr -----
    Resolved 12 packages in [TIME]
    ");

    fyn_snapshot!(context.filters(), context.why().arg("pywin32").arg("--frozen").arg("--python-platform").arg("linux"), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No dependency path found for `pywin32`
    ");

    Ok(())
}

#[test]
fn script_paths_are_dry_run() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let script = context.temp_dir.child("script.py");
    script.write_str(indoc! {r#"
        # /// script
        # requires-python = ">=3.12"
        # dependencies = [
        #   "requests<3",
        # ]
        # ///

        import requests
        "#})?;

    fyn_snapshot!(context.filters(), context.why().arg("urllib3").arg("--script").arg(script.path()), @"
    success: true
    exit_code: 0
    ----- stdout -----
    urllib3 is included because:
    requests v2.31.0 -> urllib3 v2.2.1

    ----- stderr -----
    Resolved 5 packages in [TIME]
    ");

    fyn_snapshot!(context.filters(), context.why().arg("requests").arg("--script").arg(script.path()), @"
    success: true
    exit_code: 0
    ----- stdout -----
    requests is included because:
    requests v2.31.0

    ----- stderr -----
    Resolved 5 packages in [TIME]
    ");

    fyn_snapshot!(context.filters(), context.why().arg("requests").arg("--script").arg(script.path()).arg("--group").arg("dev"), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: PEP 723 scripts do not support dependency groups, but group `dev` was specified
    ");

    assert!(!context.temp_dir.child("script.py.lock").exists());

    Ok(())
}

#[test]
fn depth_limits_paths() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = [
            "pandas",
            "scikit-learn==1.4.1.post1",
        ]
    "#,
    )?;

    fyn_snapshot!(context.filters(), context.why().arg("numpy").arg("--universal").arg("--depth").arg("2"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    numpy is included because:
    project v0.1.0 -> pandas v2.2.1 -> numpy v1.26.4
    project v0.1.0 -> scikit-learn v1.4.1.post1 -> numpy v1.26.4

    ----- stderr -----
    Resolved 11 packages in [TIME]
    ");

    Ok(())
}

#[test]
fn workspace_member_paths_do_not_cycle() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [tool.fyn.workspace]
        members = ["packages/*"]
    "#,
    )?;

    let package_a_dir = context.temp_dir.child("packages").child("package-a");
    package_a_dir.create_dir_all()?;
    package_a_dir.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "package-a"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["package-b"]

        [tool.fyn.sources]
        package-b = { workspace = true }
    "#,
    )?;

    let package_b_dir = context.temp_dir.child("packages").child("package-b");
    package_b_dir.create_dir_all()?;
    package_b_dir.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "package-b"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["package-a"]

        [tool.fyn.sources]
        package-a = { workspace = true }
    "#,
    )?;

    fyn_snapshot!(context.filters(), context.why().arg("package-a"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    package-a is included because:
    package-a v0.1.0
    package-b v0.1.0 -> package-a v0.1.0

    ----- stderr -----
    Resolved 2 packages in [TIME]
    ");

    Ok(())
}

#[test]
fn missing_selected_dependency_path() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions"]
    "#,
    )?;

    context.lock().assert().success();

    fyn_snapshot!(context.filters(), context.why().arg("sniffio").arg("--frozen"), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No dependency path found for `sniffio`
    ");

    Ok(())
}
