#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;

use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use indoc::indoc;
use predicates::prelude::predicate;
use serde_json::Value;

use fyn_static::EnvVars;
use fyn_test::{build_vendor_links_url, packse_index_url};

fn install_fake_executable(
    context: &fyn_test::TestContext,
    name: &str,
    contents: &str,
) -> Result<()> {
    let executable = context.bin_dir.child(name);
    executable.write_str(contents)?;

    let mut perms = fs_err::metadata(executable.path())?.permissions();
    perms.set_mode(0o755);
    fs_err::set_permissions(executable.path(), perms)?;

    Ok(())
}

fn read_json(path: &assert_fs::fixture::ChildPath) -> Value {
    serde_json::from_str(&fs_err::read_to_string(path.path()).expect("payload to exist"))
        .expect("payload to be valid json")
}

#[test]
fn dependency_guard_command_blocks_install_and_receives_manifest() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let payload_path = context.temp_dir.child("guard-payload.json");

    install_fake_executable(
        &context,
        "depguard",
        indoc! {r#"
            #!/bin/sh
            cat > "$FYN_TEST_GUARD_PAYLOAD"
            printf 'blocked by test guard\n' >&2
            exit "${FYN_TEST_GUARD_EXIT_CODE:-1}"
        "#},
    )?;

    context
        .pip_install()
        .arg("only-wheels-a")
        .arg("--index-url")
        .arg(packse_index_url())
        .arg("--find-links")
        .arg(build_vendor_links_url())
        .arg("--dependency-guard-provider")
        .arg("command")
        .arg("--dependency-guard-command")
        .arg("depguard")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .env("FYN_TEST_GUARD_PAYLOAD", payload_path.path())
        .env("FYN_TEST_GUARD_EXIT_CODE", "9")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Dependency guard command `depguard` blocked installation: blocked by test guard",
        ));

    let payload = read_json(&payload_path);
    let package = payload["remote"]
        .as_array()
        .and_then(|packages| {
            packages
                .iter()
                .find(|package| package["name"].as_str() == Some("only-wheels-a"))
        })
        .expect("scenario package to be present in remote plan");

    assert_eq!(payload["schema_version"], 1);
    assert_eq!(package["name"], "only-wheels-a");
    assert_eq!(package["version"], "1.0.0");
    assert_eq!(package["source_kind"], "registry");
    assert_eq!(package["requires_prepare"], true);
    assert_eq!(package["purl"], "pkg:pypi/only-wheels-a@1.0.0");

    context.pip_show().arg("only-wheels-a").assert().failure();

    Ok(())
}

#[test]
fn dependency_guard_short_aliases_work_for_install() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let payload_path = context.temp_dir.child("guard-payload.json");

    install_fake_executable(
        &context,
        "install-guard",
        indoc! {r#"
            #!/bin/sh
            cat > "$FYN_TEST_GUARD_PAYLOAD"
            printf 'blocked by alias test\n' >&2
            exit 9
        "#},
    )?;

    context
        .pip_install()
        .arg("only-wheels-a")
        .arg("--index-url")
        .arg(packse_index_url())
        .arg("--find-links")
        .arg(build_vendor_links_url())
        .arg("--guard-provider")
        .arg("command")
        .arg("--guard-command")
        .arg("install-guard")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .env("FYN_TEST_GUARD_PAYLOAD", payload_path.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Dependency guard command `install-guard` blocked installation: blocked by alias test",
        ));

    let payload = read_json(&payload_path);
    assert_eq!(payload["schema_version"], 1);
    assert!(
        payload["remote"]
            .as_array()
            .is_some_and(|packages| !packages.is_empty())
    );

    Ok(())
}

#[test]
fn dependency_guard_socket_blocks_low_score() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let args_path = context.temp_dir.child("socket-args.txt");

    install_fake_executable(
        &context,
        "socket",
        indoc! {r#"
            #!/bin/sh
            printf '%s\n' "$@" >> "$FYN_TEST_SOCKET_ARGS"
            printf '%s' "$FYN_TEST_SOCKET_OUTPUT"
            exit "${FYN_TEST_SOCKET_EXIT_CODE:-0}"
        "#},
    )?;

    context
        .pip_install()
        .arg("only-wheels-a")
        .arg("--index-url")
        .arg(packse_index_url())
        .arg("--find-links")
        .arg(build_vendor_links_url())
        .arg("--dependency-guard-provider")
        .arg("socket")
        .arg("--dependency-guard-socket-min-score")
        .arg("90")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .env("FYN_TEST_SOCKET_ARGS", args_path.path())
        .env("FYN_TEST_SOCKET_OUTPUT", r#"{"score":{"overall":75}}"#)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Socket dependency guard blocked `pkg:pypi/only-wheels-a@1.0.0` because the score 75 is below the minimum 90",
        ));

    let args = fs_err::read_to_string(args_path.path())?;
    assert!(args.contains("package"));
    assert!(args.contains("score"));
    assert!(args.contains("--json"));
    assert!(args.contains("pkg:pypi/only-wheels-a@1.0.0"));

    context.pip_show().arg("only-wheels-a").assert().failure();

    Ok(())
}

#[test]
fn dependency_guard_socket_allows_install() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let args_path = context.temp_dir.child("socket-args.txt");

    install_fake_executable(
        &context,
        "socket",
        indoc! {r#"
            #!/bin/sh
            printf '%s\n' "$@" >> "$FYN_TEST_SOCKET_ARGS"
            printf '%s' "$FYN_TEST_SOCKET_OUTPUT"
            exit "${FYN_TEST_SOCKET_EXIT_CODE:-0}"
        "#},
    )?;

    context
        .pip_install()
        .arg("only-wheels-a")
        .arg("--index-url")
        .arg(packse_index_url())
        .arg("--find-links")
        .arg(build_vendor_links_url())
        .arg("--dependency-guard-provider")
        .arg("socket")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .env("FYN_TEST_SOCKET_ARGS", args_path.path())
        .env("FYN_TEST_SOCKET_OUTPUT", r#"{"score":{"overall":97}}"#)
        .assert()
        .success();

    let args = fs_err::read_to_string(args_path.path())?;
    assert!(args.contains("pkg:pypi/only-wheels-a@1.0.0"));

    context
        .pip_show()
        .arg("only-wheels-a")
        .assert()
        .success()
        .stdout(predicate::str::contains("Version: 1.0.0"));

    Ok(())
}

#[test]
fn dependency_guard_can_be_configured_from_fyn_toml() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    context.temp_dir.child("fyn.toml").write_str(indoc! {r#"
        [pip]
        dependency-guard-provider = ["command"]
        dependency-guard-command = ["depguard"]
        dependency-guard-socket-min-score = 91
    "#})?;

    let output = context
        .pip_install()
        .arg("--show-settings")
        .arg("only-wheels-a")
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone())?;
    assert!(stdout.contains("dependency_guard: DependencyGuardSettings {"));
    assert!(stdout.contains("providers: ["));
    assert!(stdout.contains("Command"));
    assert!(stdout.contains("command: Some("));
    assert!(stdout.contains("socket_min_score: 91"));

    Ok(())
}

#[test]
fn dependency_guard_sync_blocks_project_install() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let payload_path = context.temp_dir.child("guard-payload.json");

    context
        .temp_dir
        .child("pyproject.toml")
        .write_str(indoc! {r#"
        [project]
        name = "example"
        version = "0.1.0"
        dependencies = ["only-wheels-a"]
    "#})?;

    install_fake_executable(
        &context,
        "depguard",
        indoc! {r#"
            #!/bin/sh
            cat > "$FYN_TEST_GUARD_PAYLOAD"
            printf 'blocked during sync\n' >&2
            exit 7
        "#},
    )?;

    context
        .sync()
        .arg("--no-install-project")
        .arg("--index-url")
        .arg(packse_index_url())
        .arg("--find-links")
        .arg(build_vendor_links_url())
        .arg("--dependency-guard-provider")
        .arg("command")
        .arg("--dependency-guard-command")
        .arg("depguard")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .env("FYN_TEST_GUARD_PAYLOAD", payload_path.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Dependency guard command `depguard` blocked installation: blocked during sync",
        ));

    let payload = read_json(&payload_path);
    let package = payload["remote"]
        .as_array()
        .and_then(|packages| {
            packages
                .iter()
                .find(|package| package["name"].as_str() == Some("only-wheels-a"))
        })
        .expect("scenario package to be present in remote plan");

    assert_eq!(payload["schema_version"], 1);
    assert_eq!(package["name"], "only-wheels-a");
    assert_eq!(package["version"], "1.0.0");

    context.pip_show().arg("only-wheels-a").assert().failure();

    Ok(())
}

#[test]
fn dependency_guard_top_level_settings_apply_to_sync() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    context
        .temp_dir
        .child("pyproject.toml")
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"

        [tool.fyn]
        dependency-guard-provider = ["command"]
        dependency-guard-command = ["depguard"]
        dependency-guard-socket-min-score = 91
    "#})?;

    let output = context.sync().arg("--show-settings").assert().success();

    let stdout = String::from_utf8(output.get_output().stdout.clone())?;
    assert!(stdout.contains("dependency_guard: DependencyGuardSettings {"));
    assert!(stdout.contains("providers: ["));
    assert!(stdout.contains("Command"));
    assert!(stdout.contains("command: Some("));
    assert!(stdout.contains("socket_min_score: 91"));

    Ok(())
}

#[test]
fn dependency_guard_skips_empty_compile_only_plan() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let payload_path = context.temp_dir.child("guard-payload.json");

    context
        .pip_install()
        .arg("only-wheels-a")
        .arg("--index-url")
        .arg(packse_index_url())
        .arg("--find-links")
        .arg(build_vendor_links_url())
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .assert()
        .success();

    install_fake_executable(
        &context,
        "depguard",
        indoc! {r#"
            #!/bin/sh
            cat > "$FYN_TEST_GUARD_PAYLOAD"
            printf 'guard should not run\n' >&2
            exit 5
        "#},
    )?;

    context
        .pip_install()
        .arg("only-wheels-a")
        .arg("--index-url")
        .arg(packse_index_url())
        .arg("--find-links")
        .arg(build_vendor_links_url())
        .arg("--compile-bytecode")
        .arg("--dependency-guard-provider")
        .arg("command")
        .arg("--dependency-guard-command")
        .arg("depguard")
        .env_remove(EnvVars::UV_EXCLUDE_NEWER)
        .env("FYN_TEST_GUARD_PAYLOAD", payload_path.path())
        .assert()
        .success();

    assert!(!payload_path.path().exists());

    Ok(())
}

#[test]
fn dependency_guard_flags_appear_in_pip_install_help() -> Result<()> {
    let context = fyn_test::test_context_with_versions!(&[]);

    context
        .command()
        .arg("pip")
        .arg("install")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--dependency-guard-provider"))
        .stdout(predicate::str::contains("--guard-provider"))
        .stdout(predicate::str::contains("--dependency-guard-command"))
        .stdout(predicate::str::contains("--guard-command"))
        .stdout(predicate::str::contains(
            "--dependency-guard-socket-min-score",
        ))
        .stdout(predicate::str::contains("--guard-socket-min-score"));

    Ok(())
}

#[test]
fn dependency_guard_flags_appear_in_sync_help() -> Result<()> {
    let context = fyn_test::test_context_with_versions!(&[]);

    context
        .command()
        .arg("sync")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--dependency-guard-provider"))
        .stdout(predicate::str::contains("--guard-provider"))
        .stdout(predicate::str::contains("--dependency-guard-command"))
        .stdout(predicate::str::contains("--guard-command"))
        .stdout(predicate::str::contains(
            "--dependency-guard-socket-min-score",
        ))
        .stdout(predicate::str::contains("--guard-socket-min-score"));

    Ok(())
}
