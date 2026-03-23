use assert_fs::fixture::PathChild;

use fyn_static::EnvVars;

use fyn_test::fyn_snapshot;

#[test]
fn python_dir() {
    let context = fyn_test::test_context!("3.12");

    let python_dir = context.temp_dir.child("python");
    fyn_snapshot!(context.filters(), context.python_dir()
    .env(EnvVars::UV_PYTHON_INSTALL_DIR, python_dir.as_os_str()), @"
    success: true
    exit_code: 0
    ----- stdout -----
    [TEMP_DIR]/python

    ----- stderr -----
    ");
}
