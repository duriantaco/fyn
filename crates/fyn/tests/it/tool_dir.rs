use assert_fs::fixture::PathChild;

use fyn_static::EnvVars;

use fyn_test::fyn_snapshot;

#[test]
fn tool_dir() {
    let context = fyn_test::test_context!("3.12");
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    fyn_snapshot!(context.filters(), context.tool_dir()
    .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
    .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @"
    success: true
    exit_code: 0
    ----- stdout -----
    [TEMP_DIR]/tools

    ----- stderr -----
    ");
}

#[test]
fn tool_dir_bin() {
    let context = fyn_test::test_context!("3.12");
    let tool_dir = context.temp_dir.child("tools");
    let bin_dir = context.temp_dir.child("bin");

    fyn_snapshot!(context.filters(), context.tool_dir().arg("--bin")
    .env(EnvVars::UV_TOOL_DIR, tool_dir.as_os_str())
    .env(EnvVars::XDG_BIN_HOME, bin_dir.as_os_str()), @"
    success: true
    exit_code: 0
    ----- stdout -----
    [TEMP_DIR]/bin

    ----- stderr -----
    ");
}
