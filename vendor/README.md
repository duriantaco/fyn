# Vendored crates

This directory contains source code vendored from published crates.io crates that
the workspace depends on via `path` dependencies (see `Cargo.toml`).

Each subdirectory is a bundle of the published crate, pinned to the exact version
recorded in `Cargo.lock`, with fyn-specific modifications applied on top (for
example, a trimmed `Cargo.toml` so dependency versions follow the workspace, and
any required source patches). The vendored sources are tracked in git so builds do
not depend on the network.

## Layout

- `astral_async_http_range_reader/` — vendored from
  [`astral_async_http_range_reader`](https://crates.io/crates/astral_async_http_range_reader)
  (repository: <https://github.com/astral-sh/async_http_range_reader>), pinned to the
  version in `Cargo.lock`.

## Verifying and updating

Run `scripts/fetch-vendor.sh --check` to verify that every vendored crate is
complete and that its version matches `Cargo.lock`. This check runs in CI as part
of `check-generated-files.yml`.

To inspect how the working tree differs from the published crate at the pinned
version, use `scripts/fetch-vendor.sh --fetch`: it downloads the crates.io source
archive into `target/vendor-src/` and prints a diff summary. To update a vendored
crate, bump the version in `Cargo.lock` (via `cargo update`) and in the vendored
`Cargo.toml`, re-verify with `--check`, and review the `--fetch` diff before
committing.