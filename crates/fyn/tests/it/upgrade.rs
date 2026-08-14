use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn project_upgrade_updates_constraint_lock_and_environment() -> Result<()> {
    let context = fyn_test::test_context_with_versions!(&["3.12"]);
    let pyproject = context.temp_dir.child("pyproject.toml");
    pyproject.write_str(
        r#"
[project]
name = "example"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio<3"]
"#,
    )?;

    context.lock().assert().success();
    let lockfile = context.temp_dir.child("fyn.lock");
    let original_lock = fs_err::read_to_string(&lockfile)?;

    context
        .upgrade()
        .arg("anyio")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Updated requirement: `anyio<3` -> `anyio<5`",
        ))
        .stderr(predicate::str::contains(
            "Dependencies upgraded successfully.",
        ));

    assert!(fs_err::read_to_string(&pyproject)?.contains("anyio<5"));
    assert_ne!(fs_err::read_to_string(&lockfile)?, original_lock);
    assert!(context.temp_dir.child(".venv").exists());
    context
        .assert_command(
            "import importlib.metadata; print(importlib.metadata.version('anyio'), end='')",
        )
        .success()
        .stdout("4.3.0");
    Ok(())
}

#[test]
fn project_upgrade_updates_constraint_and_lock_without_syncing() -> Result<()> {
    let context = fyn_test::test_context_with_versions!(&["3.12"]);
    let pyproject = context.temp_dir.child("pyproject.toml");
    pyproject.write_str(
        r#"
[project]
name = "example"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["AnyIO>=2,<3,!=2.1 ; python_version >= '3.12'"]
"#,
    )?;

    context
        .upgrade()
        .arg("anyio")
        .arg("--no-sync")
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated requirement:"));

    let content = fs_err::read_to_string(&pyproject)?;
    assert!(content.contains("anyio>=2,!=2.1,<5 ; python_full_version >= '3.12'"));
    assert!(context.temp_dir.child("fyn.lock").exists());
    assert!(!context.temp_dir.child(".venv").exists());
    Ok(())
}

#[test]
fn project_upgrade_dry_run_does_not_mutate_project() -> Result<()> {
    let context = fyn_test::test_context_with_versions!(&["3.12"]);
    let pyproject = context.temp_dir.child("pyproject.toml");
    let original = r#"
[project]
name = "example"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio<3"]
"#;
    pyproject.write_str(original)?;

    context
        .upgrade()
        .arg("anyio")
        .arg("--dry-run")
        .assert()
        .success()
        .stderr(predicate::str::contains("Would update requirement:"))
        .stderr(predicate::str::contains("No changes were made"));

    assert_eq!(fs_err::read_to_string(&pyproject)?, original);
    assert!(!context.temp_dir.child("fyn.lock").exists());
    assert!(!context.temp_dir.child(".venv").exists());
    Ok(())
}

#[test]
fn project_upgrade_all_respects_exclusions() -> Result<()> {
    let context = fyn_test::test_context_with_versions!(&["3.12"]);
    let pyproject = context.temp_dir.child("pyproject.toml");
    pyproject.write_str(
        r#"
[project]
name = "example"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio<3", "sniffio<1"]
"#,
    )?;

    context
        .upgrade()
        .arg("--exclude")
        .arg("anyio")
        .arg("--no-sync")
        .assert()
        .success()
        .stderr(predicate::str::contains("Updated requirement: `sniffio<1`"))
        .stderr(predicate::str::contains("anyio").not());

    let content = fs_err::read_to_string(&pyproject)?;
    assert!(content.contains("anyio<3"));
    assert!(content.contains("sniffio<2"));
    assert!(context.temp_dir.child("fyn.lock").exists());
    Ok(())
}

#[test]
fn project_upgrade_resolution_failure_leaves_project_unchanged() -> Result<()> {
    let context = fyn_test::test_context_with_versions!(&["3.12"]);
    let pyproject = context.temp_dir.child("pyproject.toml");
    let original = r#"
[project]
name = "example"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio<3", "sniffio==9999"]
"#;
    pyproject.write_str(original)?;

    context
        .upgrade()
        .arg("anyio")
        .arg("--no-sync")
        .assert()
        .failure();

    assert_eq!(fs_err::read_to_string(&pyproject)?, original);
    assert!(!context.temp_dir.child("fyn.lock").exists());
    assert!(!context.temp_dir.child(".venv").exists());
    Ok(())
}
