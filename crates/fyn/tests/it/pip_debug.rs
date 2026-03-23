use fyn_test::fyn_snapshot;

#[test]
fn debug_warn() {
    let context = fyn_test::test_context!("3.12");

    fyn_snapshot!(context.pip_debug(), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: pip's `debug` is unsupported (consider using `fynx pip debug` instead)
    "
    );
}
