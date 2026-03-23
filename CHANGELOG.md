# Changelog

<!-- prettier-ignore-start -->


## 0.10.13

Released on 2026-03-22. First release as **fyn**, a community fork of uv.
See [MANIFESTO.md](MANIFESTO.md) for why this exists.

### Breaking changes

- Renamed binary from `uv` to `fyn`, `uvx` to `fynx`, `uvw` to `fynw`. All 70 workspace crates
  renamed from `uv-*` to `fyn-*`. Configuration files and environment variables (`UV_*`) are
  unchanged for backwards compatibility.

### Enhancements

- Add `fyn shell` command to spawn a new shell with the virtual environment activated. Detects
  bash, zsh, fish, nushell, powershell, and cmd automatically
  ([astral-sh/uv#1910](https://github.com/astral-sh/uv/issues/1910))
- Add `fyn upgrade [packages...]` command to upgrade and sync dependencies in one step. Supports
  `--dry-run` and `--no-sync`
  ([astral-sh/uv#6794](https://github.com/astral-sh/uv/issues/6794))
- Add task runner via `[tool.uv.tasks]` in `pyproject.toml`. Supports string and table forms,
  `--list-tasks`, and extra arg passthrough
  ([astral-sh/uv#5903](https://github.com/astral-sh/uv/issues/5903))
- Add `UV_CACHE_MAX_SIZE` environment variable for automatic cache size limiting with LRU
  eviction. Supports `K`, `M`, `G`, and `T` suffixes
  ([astral-sh/uv#5731](https://github.com/astral-sh/uv/issues/5731))
- Add `UV_LOCKFILE` environment variable to override the default lockfile name
  ([astral-sh/uv#6830](https://github.com/astral-sh/uv/issues/6830))
- Expand environment variables (`${VAR}`) in index URLs from `pyproject.toml` and `uv.toml`
  ([astral-sh/uv#5734](https://github.com/astral-sh/uv/issues/5734))

### Bug fixes

- Fix explicit index not respected for transitive dependencies
  ([astral-sh/uv#8253](https://github.com/astral-sh/uv/issues/8253))
- Fix `fyn remove --group` re-sync not including the target group, causing other group packages
  to be uninstalled ([astral-sh/uv#9012](https://github.com/astral-sh/uv/issues/9012))

### Privacy

- Remove linehaul telemetry module. User-Agent header is now `fyn/<version>` with no system
  profiling
- Remove `releases.astral.sh` as download mirror. Downloads go to GitHub directly
- Disable self-update (previously pointed to `astral-sh/uv` releases)

### Internal

- Update all crate READMEs and documentation links
- Replace `panic!()` with `.expect()` in lockfile path handling
- Add path traversal protection on `UV_LOCKFILE` environment variable
- Fix `CARGO_BIN_EXE` compile-time reference in test framework for renamed binary