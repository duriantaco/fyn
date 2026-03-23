use fv_test::fv_snapshot;

#[test]
fn debug_warn() {
    let context = fv_test::test_context!("3.12");

    fv_snapshot!(context.pip_debug(), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: pip's `debug` is unsupported (consider using `fvx pip debug` instead)
    "
    );
}
