use assert_cmd::assert::OutputAssertExt;

use fyn_test::fyn_snapshot;

/// Test that `cache size` returns 0 for an empty cache directory (raw output).
#[test]
fn cache_size_empty_raw() {
    let context = fyn_test::test_context!("3.12");

    // Clean cache first to ensure truly empty state
    context.clean().assert().success();

    fyn_snapshot!(context.cache_size().arg("--preview"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    0

    ----- stderr -----
    ");
}

/// Test that `cache size` returns raw bytes after installing packages.
#[test]
fn cache_size_with_packages_raw() {
    let context = fyn_test::test_context!("3.12");

    // Install a requirement to populate the cache.
    context.pip_install().arg("iniconfig").assert().success();

    // Check cache size is now positive (raw bytes).
    fyn_snapshot!(context.with_filtered_cache_size().filters(), context.cache_size().arg("--preview"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    [SIZE]

    ----- stderr -----
    ");
}

/// Test that `cache size --human` returns human-readable format after installing packages.
#[test]
fn cache_size_with_packages_human() {
    let context = fyn_test::test_context!("3.12");

    // Install a requirement to populate the cache.
    context.pip_install().arg("iniconfig").assert().success();

    // Check cache size with --human flag
    fyn_snapshot!(context.with_filtered_cache_size().filters(), context.cache_size().arg("--preview").arg("--human"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    [SIZE]

    ----- stderr -----
    ");
}
