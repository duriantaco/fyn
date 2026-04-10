#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
use anyhow::Result;
#[cfg(unix)]
use assert_fs::prelude::*;
#[cfg(unix)]
use fyn_test::fyn_snapshot;
#[cfg(unix)]
use indoc::indoc;

#[cfg(unix)]
fn install_fake_bash(context: &fyn_test::TestContext) -> Result<()> {
    let bash = context.bin_dir.child("bash");
    bash.write_str(indoc! {r#"
        #!/bin/sh
        python - <<'PY'
        import os
        import sys

        print(os.environ.get("FYN_TEST_EMPIRE_VARIABLE"))
        print(os.environ.get("FYN_TEST_REBEL_VARIABLE"))
        print(os.environ.get("VIRTUAL_ENV") == sys.prefix)
        print(os.path.basename(sys.prefix))
        PY
        "#})?;

    let mut perms = fs_err::metadata(bash.path())?.permissions();
    perms.set_mode(0o755);
    fs_err::set_permissions(bash.path(), perms)?;

    Ok(())
}

#[cfg(unix)]
#[test]
fn shell_with_default_env_file() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    install_fake_bash(&context)?;

    context.temp_dir.child(".env").write_str(indoc! {"
        FYN_TEST_EMPIRE_VARIABLE=palpatine
        FYN_TEST_REBEL_VARIABLE=leia
    "})?;

    fyn_snapshot!(context.filters(), context.shell(), @"
    success: true
    exit_code: 0
    ----- stdout -----
    palpatine
    leia
    True
    .venv

    ----- stderr -----
    success: Activated virtual environment at [VENV]/
    Type exit to deactivate.
    ");

    Ok(())
}

#[cfg(unix)]
#[test]
fn shell_with_default_env_file_missing_is_a_noop() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    install_fake_bash(&context)?;

    fyn_snapshot!(context.filters(), context.shell(), @"
    success: true
    exit_code: 0
    ----- stdout -----
    None
    None
    True
    .venv

    ----- stderr -----
    success: Activated virtual environment at [VENV]/
    Type exit to deactivate.
    ");

    Ok(())
}

#[cfg(unix)]
#[test]
fn shell_with_explicit_env_file_skips_default_env_file() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    install_fake_bash(&context)?;

    context.temp_dir.child(".env").write_str(indoc! {"
        FYN_TEST_EMPIRE_VARIABLE=default_value
    "})?;
    context.temp_dir.child(".file").write_str(indoc! {"
        FYN_TEST_EMPIRE_VARIABLE=explicit_value
        FYN_TEST_REBEL_VARIABLE=han
    "})?;

    fyn_snapshot!(context.filters(), context.shell().arg("--env-file").arg(".file"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    explicit_value
    han
    True
    .venv

    ----- stderr -----
    success: Activated virtual environment at [VENV]/
    Type exit to deactivate.
    ");

    Ok(())
}

#[cfg(unix)]
#[test]
fn shell_with_multiple_env_files_from_environment() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    install_fake_bash(&context)?;

    context.temp_dir.child(".env1").write_str(indoc! {"
        FYN_TEST_EMPIRE_VARIABLE=palpatine
        FYN_TEST_REBEL_VARIABLE=leia
    "})?;
    context.temp_dir.child(".env2").write_str(indoc! {"
        FYN_TEST_REBEL_VARIABLE=obi_wan
    "})?;

    fyn_snapshot!(
        context.filters(),
        context
            .shell()
            .env("UV_ENV_FILE", ".env1 .env2"),
        @"
    success: true
    exit_code: 0
    ----- stdout -----
    palpatine
    obi_wan
    True
    .venv

    ----- stderr -----
    success: Activated virtual environment at [VENV]/
    Type exit to deactivate.
    "
    );

    Ok(())
}

#[cfg(unix)]
#[test]
fn shell_with_env_omitted() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    install_fake_bash(&context)?;

    context.temp_dir.child(".env").write_str(indoc! {"
        FYN_TEST_EMPIRE_VARIABLE=palpatine
    "})?;

    fyn_snapshot!(context.filters(), context.shell().arg("--no-env-file"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    None
    None
    True
    .venv

    ----- stderr -----
    success: Activated virtual environment at [VENV]/
    Type exit to deactivate.
    ");

    Ok(())
}

#[cfg(unix)]
#[test]
fn shell_with_malformed_default_env_file() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    install_fake_bash(&context)?;

    context.temp_dir.child(".env").write_str(indoc! {"
        FYN_^TEST_EMPIRE_VARIABLE=darth_vader
    "})?;

    fyn_snapshot!(context.filters(), context.shell(), @"
    success: true
    exit_code: 0
    ----- stdout -----
    None
    None
    True
    .venv

    ----- stderr -----
    warning: Failed to parse environment file `.env` at position 4: FYN_^TEST_EMPIRE_VARIABLE=darth_vader
    success: Activated virtual environment at [VENV]/
    Type exit to deactivate.
    ");

    Ok(())
}

#[cfg(unix)]
#[test]
fn shell_with_not_existing_explicit_env_file() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    install_fake_bash(&context)?;

    fyn_snapshot!(
        context.filters(),
        context.shell().arg("--env-file").arg(".env.development"),
        @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: No environment file found at: `.env.development`
    "
    );

    Ok(())
}

#[cfg(unix)]
#[test]
fn shell_preserves_existing_environment_over_env_file() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    install_fake_bash(&context)?;

    context.temp_dir.child(".env").write_str(indoc! {"
        FYN_TEST_EMPIRE_VARIABLE=palpatine
    "})?;

    fyn_snapshot!(
        context.filters(),
        context
            .shell()
            .env("FYN_TEST_EMPIRE_VARIABLE", "darth_vader"),
        @"
    success: true
    exit_code: 0
    ----- stdout -----
    darth_vader
    None
    True
    .venv

    ----- stderr -----
    success: Activated virtual environment at [VENV]/
    Type exit to deactivate.
    "
    );

    Ok(())
}
