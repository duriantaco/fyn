# [Caching](#caching)

## [Dependency caching](#dependency-caching)

fyn uses aggressive caching to avoid re-downloading (and re-building) dependencies that have already been accessed in prior runs.

The specifics of fyn's caching semantics vary based on the nature of the dependency:

- **For registry dependencies** (like those downloaded from PyPI), fyn respects HTTP caching headers.
- **For direct URL dependencies**, fyn respects HTTP caching headers, and also caches based on the URL itself.
- **For Git dependencies**, fyn caches based on the fully-resolved Git commit hash. As such, `fyn pip compile` will pin Git dependencies to a specific commit hash when writing the resolved dependency set.
- **For local dependencies**, fyn caches based on the last-modified time of the source archive (i.e., the local `.whl` or `.tar.gz` file). For directories, fyn caches based on the last-modified time of the `pyproject.toml`, `setup.py`, or `setup.cfg` file.

If you're running into caching issues, fyn includes a few escape hatches:

- To clear the cache entirely, run `fyn cache clean`. To clear the cache for a specific package, run `fyn cache clean <package-name>`. For example, `fyn cache clean ruff` will clear the cache for the `ruff` package.
- To force fyn to revalidate cached data for all dependencies, pass `--refresh` to any command (e.g., `fyn sync --refresh` or `fyn pip install --refresh ...`).
- To force fyn to revalidate cached data for a specific dependency pass `--refresh-package` to any command (e.g., `fyn sync --refresh-package ruff` or `fyn pip install --refresh-package ruff ...`).
- To force fyn to ignore existing installed versions, pass `--reinstall` to any installation command (e.g., `fyn sync --reinstall` or `fyn pip install --reinstall ...`). (Consider running `fyn cache clean <package-name>` first, to ensure that the cache is cleared prior to reinstallation.)

As a special case, fyn will always rebuild and reinstall any local directory dependencies passed explicitly on the command-line (e.g., `fyn pip install .`).

## [Dynamic metadata](#dynamic-metadata)

By default, fyn will *only* rebuild and reinstall local directory dependencies (e.g., editables) if the `pyproject.toml`, `setup.py`, or `setup.cfg` file in the directory root has changed, or if a `src` directory is added or removed. This is a heuristic and, in some cases, may lead to fewer re-installs than desired.

To incorporate additional information into the cache key for a given package, you can add cache key entries under [`tool.fyn.cache-keys`](https://docs.astral.sh/uv/reference/settings/#cache-keys), which covers both file paths and Git commit hashes. Setting [`tool.fyn.cache-keys`](https://docs.astral.sh/uv/reference/settings/#cache-keys) will replace defaults, so any necessary files (like `pyproject.toml`) should still be included in the user-defined cache keys.

For example, if a project specifies dependencies in `pyproject.toml` but uses [`setuptools-scm`](https://pypi.org/project/setuptools-scm/) to manage its version, and should thus be rebuilt whenever the commit hash or dependencies change, you can add the following to the project's `pyproject.toml`:

pyproject.toml

```
[tool.fyn]
cache-keys = [{ file = "pyproject.toml" }, { git = { commit = true } }]
```

If your dynamic metadata incorporates information from the set of Git tags, you can expand the cache key to include the tags:

pyproject.toml

```
[tool.fyn]
cache-keys = [{ file = "pyproject.toml" }, { git = { commit = true, tags = true } }]
```

Similarly, if a project reads from a `requirements.txt` to populate its dependencies, you can add the following to the project's `pyproject.toml`:

pyproject.toml

```
[tool.fyn]
cache-keys = [{ file = "pyproject.toml" }, { file = "requirements.txt" }]
```

Globs are supported for `file` keys, following the syntax of the [`glob`](https://docs.rs/glob/0.3.1/glob/struct.Pattern.html) crate. For example, to invalidate the cache whenever a `.toml` file in the project directory or any of its subdirectories is modified, use the following:

pyproject.toml

```
[tool.fyn]
cache-keys = [{ file = "**/*.toml" }]
```

Note

The use of globs can be expensive, as fyn may need to walk the filesystem to determine whether any files have changed. This may, in turn, requiring traversal of large or deeply nested directories.

Similarly, if a project relies on an environment variable, you can add the following to the project's `pyproject.toml` to invalidate the cache whenever the environment variable changes:

pyproject.toml

```
[tool.fyn]
cache-keys = [{ file = "pyproject.toml" }, { env = "MY_ENV_VAR" }]
```

Finally, to invalidate a project whenever a specific directory (like `src`) is created or removed, add the following to the project's `pyproject.toml`:

pyproject.toml

```
[tool.fyn]
cache-keys = [{ file = "pyproject.toml" }, { dir = "src" }]
```

Note that the `dir` key will only track changes to the directory itself, and not arbitrary changes within the directory.

As an escape hatch, if a project uses `dynamic` metadata that isn't covered by `tool.fyn.cache-keys`, you can instruct fyn to *always* rebuild and reinstall it by adding the project to the `tool.fyn.reinstall-package` list:

pyproject.toml

```
[tool.fyn]
reinstall-package = ["my-package"]
```

This will force fyn to rebuild and reinstall `my-package` on every run, regardless of whether the package's `pyproject.toml`, `setup.py`, or `setup.cfg` file has changed.

## [Cache safety](#cache-safety)

It's safe to run multiple fyn commands concurrently, even against the same virtual environment. fyn's cache is designed to be thread-safe and append-only, and thus robust to multiple concurrent readers and writers. fyn applies a file-based lock to the target virtual environment when installing, to avoid concurrent modifications across processes.

Note that it's *never* safe to modify the cache directly (e.g., by removing a file or directory).

## [Clearing the cache](#clearing-the-cache)

fyn provides a few different mechanisms for removing entries from the cache:

- `fyn cache clean` removes *all* cache entries from the cache directory, clearing it out entirely.
- `fyn cache clean ruff` removes all cache entries for the `ruff` package, useful for invalidating the cache for a single or finite set of packages.
- `fyn cache prune` removes all *unused* cache entries. For example, the cache directory may contain entries created in previous fyn versions that are no longer necessary and can be safely removed. `fyn cache prune` is safe to run periodically, to keep the cache directory clean.

fyn blocks cache-modifying operations while other fyn commands are running. By default, those `fyn cache` commands have a 5 min timeout waiting for other fyn processes to terminate to avoid deadlocks. This timeout can be changed with [`UV_LOCK_TIMEOUT`](../../reference/environment/#uv_lock_timeout). In cases where it is known that no other fyn processes are reading or writing from the cache, `--force` can be used to ignore the lock.

## [Caching in continuous integration](#caching-in-continuous-integration)

It's common to cache package installation artifacts in continuous integration environments (like GitHub Actions or GitLab CI) to speed up subsequent runs.

By default, fyn caches both the wheels that it builds from source and the pre-built wheels that it downloads directly, to enable high-performance package installation.

However, in continuous integration environments, persisting pre-built wheels may be undesirable. With fyn, it turns out that it's often faster to *omit* pre-built wheels from the cache (and instead re-download them from the registry on each run). On the other hand, caching wheels that are built from source tends to be worthwhile, since the wheel building process can be expensive, especially for extension modules.

To support this caching strategy, fyn provides a `fyn cache prune --ci` command, which removes all pre-built wheels and unzipped source distributions from the cache, but retains any wheels that were built from source. We recommend running `fyn cache prune --ci` at the end of your continuous integration job to ensure maximum cache efficiency. For an example, see the [GitHub integration guide](../../guides/integration/github/#caching).

## [Cache directory](#cache-directory)

fyn determines the cache directory according to, in order:

1. A temporary cache directory, if `--no-cache` was requested.
1. The specific cache directory specified via `--cache-dir`, `UV_CACHE_DIR`, or [`tool.fyn.cache-dir`](../../reference/settings/#cache-dir).
1. A system-appropriate cache directory, e.g., `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on Unix and `%LOCALAPPDATA%\fyn\cache` on Windows

Note

fyn *always* requires a cache directory. When `--no-cache` is requested, fyn will still use a temporary cache for sharing data within that single invocation.

In most cases, `--refresh` should be used instead of `--no-cache` — as it will update the cache for subsequent operations but not read from the cache.

It is important for performance for the cache directory to be located on the same file system as the Python environment fyn is operating on. Otherwise, fyn will not be able to link files from the cache into the environment and will instead need to fallback to slow copy operations.

## [Cache versioning](#cache-versioning)

The fyn cache is composed of a number of buckets (e.g., a bucket for wheels, a bucket for source distributions, a bucket for Git repositories, and so on). Each bucket is versioned, such that if a release contains a breaking change to the cache format, fyn will not attempt to read from or write to an incompatible cache bucket.

For example, fyn 0.4.13 included a breaking change to the core metadata bucket. As such, the bucket version was increased from v12 to v13. Within a cache version, changes are guaranteed to be both forwards- and backwards-compatible.

Since changes in the cache format are accompanied by changes in the cache version, multiple versions of fyn can safely read and write to the same cache directory. However, if the cache version changed between a given pair of fyn releases, then those releases may not be able to share the same underlying cache entries.

For example, it's safe to use a single shared cache for fyn 0.4.12 and fyn 0.4.13, though the cache itself may contain duplicate entries in the core metadata bucket due to the change in cache version.
