use std::path::PathBuf;

use fyn_test::fyn_snapshot;

#[test]
fn index_versions_from_find_links() {
    let context = fyn_test::test_context!("3.12")
        .with_exclude_newer("2100-01-01T00:00:00Z")
        .with_filter((r"warning: ok-[^\n]+\n", ""));
    let find_links = PathBuf::from(&context.workspace_root).join("test/links");

    fyn_snapshot!(context.filters(), context.pip_index_versions()
        .arg("ok")
        .arg("--no-index")
        .arg("--find-links")
        .arg(find_links), @"
    success: true
    exit_code: 0
    ----- stdout -----
    ok (2.0.0)
    Available versions: 2.0.0, 1.0.0

    ----- stderr -----
    "
    );
}

#[test]
fn index_versions_missing_package() {
    let context = fyn_test::test_context!("3.12").with_exclude_newer("2100-01-01T00:00:00Z");
    let find_links = PathBuf::from(&context.workspace_root).join("test/links");

    fyn_snapshot!(context.filters(), context.pip_index_versions()
        .arg("missing-package")
        .arg("--no-index")
        .arg("--find-links")
        .arg(find_links), @"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: Package not found: `missing-package`
    "
    );
}
