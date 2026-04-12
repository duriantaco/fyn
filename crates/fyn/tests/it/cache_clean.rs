use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;

use fyn_cache::Cache;
use fyn_static::EnvVars;

use fyn_test::fyn_snapshot;

/// `cache clean` should remove all packages.
#[test]
fn clean_all() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let requirements_txt = context.temp_dir.child("requirements.txt");
    requirements_txt.write_str("typing-extensions\niniconfig")?;

    // Install a requirement, to populate the cache.
    context
        .pip_sync()
        .arg("requirements.txt")
        .assert()
        .success();

    fyn_snapshot!(context.with_filtered_counts().filters(), context.clean().arg("--verbose"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG fyn [VERSION] ([COMMIT] DATE)
    Clearing cache at: [CACHE_DIR]/
    Removed [N] files ([SIZE])
    ");

    Ok(())
}

/// `cache clear` should behave as an alias of `cache clean`.
#[test]
fn clear_all_alias() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let requirements_txt = context.temp_dir.child("requirements.txt");
    requirements_txt.write_str("typing-extensions\niniconfig")?;

    // Install a requirement, to populate the cache.
    context
        .pip_sync()
        .arg("requirements.txt")
        .assert()
        .success();

    let mut command = context.command();
    command.arg("cache").arg("clear").arg("--verbose");

    fyn_snapshot!(context.with_filtered_counts().filters(), command, @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG fyn [VERSION] ([COMMIT] DATE)
    Clearing cache at: [CACHE_DIR]/
    Removed [N] files ([SIZE])
    ");

    Ok(())
}

#[tokio::test]
async fn clean_force() -> Result<()> {
    let context = fyn_test::test_context!("3.12").with_filtered_counts();

    let requirements_txt = context.temp_dir.child("requirements.txt");
    requirements_txt.write_str("typing-extensions\niniconfig")?;

    // Install a requirement, to populate the cache.
    context
        .pip_sync()
        .arg("requirements.txt")
        .assert()
        .success();

    // When unlocked, `--force` should still take a lock
    fyn_snapshot!(context.filters(), context.clean().arg("--verbose").arg("--force"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG fyn [VERSION] ([COMMIT] DATE)
    Clearing cache at: [CACHE_DIR]/
    Removed [N] files ([SIZE])
    ");

    // Install a requirement, to re-populate the cache.
    context
        .pip_sync()
        .arg("requirements.txt")
        .assert()
        .success();

    // When locked, `--force` should proceed without blocking
    let _cache = fyn_cache::Cache::from_path(context.cache_dir.path())
        .with_exclusive_lock()
        .await;
    fyn_snapshot!(context.filters(), context.clean().arg("--verbose").arg("--force"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG fyn [VERSION] ([COMMIT] DATE)
    DEBUG Lock is busy for `[CACHE_DIR]/`
    DEBUG Cache is currently in use, proceeding due to `--force`
    Clearing cache at: [CACHE_DIR]/
    Removed [N] files ([SIZE])
    ");

    Ok(())
}

/// `cache clean iniconfig` should remove a single package (`iniconfig`).
#[test]
fn clean_package_pypi() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let requirements_txt = context.temp_dir.child("requirements.txt");
    requirements_txt.write_str("anyio\niniconfig")?;

    // Install a requirement, to populate the cache.
    context
        .pip_sync()
        .arg("requirements.txt")
        .assert()
        .success();

    // Assert that the `.rkyv` file is created for `iniconfig`.
    let rkyv = context
        .cache_dir
        .child("simple-v20")
        .child("pypi")
        .child("iniconfig.rkyv");
    assert!(
        rkyv.exists(),
        "Expected the `.rkyv` file to exist for `iniconfig`"
    );

    let filters: Vec<_> = context
        .filters()
        .into_iter()
        .chain([
            // The cache entry does not have a stable key, so we filter it out.
            (
                r"\[CACHE_DIR\](\\|\/)(.+)(\\|\/).*",
                "[CACHE_DIR]/$2/[ENTRY]",
            ),
            // The file count varies by operating system, so we filter it out.
            ("Removed \\d+ files?", "Removed [N] files"),
        ])
        .collect();

    fyn_snapshot!(&filters, context.clean().arg("--verbose").arg("iniconfig"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG fyn [VERSION] ([COMMIT] DATE)
    DEBUG Removing dangling cache entry: [CACHE_DIR]/archive-v0/[ENTRY]
    Removed [N] files ([SIZE])
    ");

    // Assert that the `.rkyv` file is removed for `iniconfig`.
    assert!(
        !rkyv.exists(),
        "Expected the `.rkyv` file to be removed for `iniconfig`"
    );

    // Running `fyn cache prune` should have no effect.
    fyn_snapshot!(&filters, context.prune().arg("--verbose"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG fyn [VERSION] ([COMMIT] DATE)
    Pruning cache at: [CACHE_DIR]/
    No unused entries found
    ");

    Ok(())
}

/// `cache clean iniconfig` should remove a single package (`iniconfig`).
#[test]
fn clean_package_index() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    let requirements_txt = context.temp_dir.child("requirements.txt");
    requirements_txt.write_str("anyio\niniconfig")?;

    // Install a requirement, to populate the cache.
    context
        .pip_sync()
        .arg("requirements.txt")
        .arg("--index-url")
        .arg("https://test.pypi.org/simple")
        .assert()
        .success();

    // Assert that the `.rkyv` file is created for `iniconfig`.
    let rkyv = context
        .cache_dir
        .child("simple-v20")
        .child("index")
        .child("e8208120cae3ba69")
        .child("iniconfig.rkyv");
    assert!(
        rkyv.exists(),
        "Expected the `.rkyv` file to exist for `iniconfig`"
    );

    let filters: Vec<_> = context
        .filters()
        .into_iter()
        .chain([
            // The cache entry does not have a stable key, so we filter it out.
            (
                r"\[CACHE_DIR\](\\|\/)(.+)(\\|\/).*",
                "[CACHE_DIR]/$2/[ENTRY]",
            ),
            // The file count varies by operating system, so we filter it out.
            ("Removed \\d+ files?", "Removed [N] files"),
        ])
        .collect();

    fyn_snapshot!(&filters, context.clean().arg("--verbose").arg("iniconfig"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG fyn [VERSION] ([COMMIT] DATE)
    DEBUG Removing dangling cache entry: [CACHE_DIR]/archive-v0/[ENTRY]
    Removed [N] files ([SIZE])
    ");

    // Assert that the `.rkyv` file is removed for `iniconfig`.
    assert!(
        !rkyv.exists(),
        "Expected the `.rkyv` file to be removed for `iniconfig`"
    );

    Ok(())
}

#[tokio::test]
async fn cache_timeout() {
    let context = fyn_test::test_context!("3.12");

    // Simulate another fyn process running and locking the cache, e.g., with a source build.
    let _cache = Cache::from_path(context.cache_dir.path())
        .with_exclusive_lock()
        .await;

    fyn_snapshot!(context.filters(), context.clean().env(EnvVars::UV_LOCK_TIMEOUT, "1"), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Cache is currently in-use, waiting for other fyn processes to finish (use `--force` to override)
    error: Timeout ([TIME]) when waiting for lock on `[CACHE_DIR]/` at `[CACHE_DIR]/.lock`, is another fyn process running? You can set `UV_LOCK_TIMEOUT` to increase the timeout.
    ");
}

/// `cache clean` should handle file paths normally restricted by Win32 path normalization.
#[cfg(windows)]
#[test]
fn clean_handles_verbatim_paths() -> Result<()> {
    let context = fyn_test::test_context!("3.12");

    fs_err::remove_dir_all(&context.cache_dir)?;

    let uwsgi_shard = context
        .cache_dir
        .child("sdists-v9")
        .child("pypi")
        .child("uwsgi")
        .child("2.0.31")
        .child("QxDIp0qpjbsWjWURKmegK")
        .child("src")
        .child("core");

    uwsgi_shard.create_dir_all()?;
    let invalid_path = uwsgi_shard.child("logging.").to_path_buf();
    let invalid_file = fyn_fs::verbatim_path(invalid_path.as_path());
    fs_err::write(&invalid_file, b"")?;

    let remove_err = fs_err::remove_file(&invalid_path).expect_err("expected to fail");
    assert_eq!(remove_err.kind(), std::io::ErrorKind::NotFound);

    fyn_snapshot!(context.filters(), context.clean().arg("--verbose"), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    DEBUG fyn [VERSION] ([COMMIT] DATE)
    Clearing cache at: [CACHE_DIR]/
    Removed 2 files
    ");

    Ok(())
}
