# Benchmarks

fv uses the same Rust engine as uv and achieves the same performance — 10-100x faster than pip.

All benchmarks were computed on macOS using Python 3.12.4 and come with a few important caveats:

- Benchmark performance may vary dramatically across different operating systems and filesystems. In
  particular, fv uses different installation strategies based on the underlying filesystem's
  capabilities. (For example, fv uses reflinking on macOS, and hardlinking on Linux.)
- Benchmark performance may vary dramatically depending on the set of packages being installed. For
  example, a resolution that requires building a single intensive source distribution may appear
  very similar across tools, since the bottleneck is tool-agnostic.

This document benchmarks against Trio's `docs-requirements.in`, as a representative example of a
real-world project.

## Warm Installation

Benchmarking package installation (e.g., `fv sync`) with a warm cache. This is equivalent to
removing and recreating a virtual environment, and then populating it with dependencies that you've
installed previously on the same machine.

## Cold Installation

Benchmarking package installation (e.g., `fv sync`) with a cold cache. This is equivalent to running
`fv sync` on a new machine or in CI (assuming that the package manager cache is not shared across
runs).

## Warm Resolution

Benchmarking dependency resolution (e.g., `fv lock`) with a warm cache, but no existing lockfile.
This is equivalent to blowing away an existing `requirements.txt` file to regenerate it from a
`requirements.in` file.

## Cold Resolution

Benchmarking dependency resolution (e.g., `fv lock`) with a cold cache. This is equivalent to
running `fv lock` on a new machine or in CI (assuming that the package manager cache is not shared
across runs).

## Reproduction

All benchmarks can be generated using the `scripts/benchmark` package, which wraps
[`hyperfine`](https://github.com/sharkdp/hyperfine) to facilitate benchmarking fv against a variety
of other tools.

The benchmark script itself has several requirements:

- A local fv release build (`cargo build --release`).
- The [`hyperfine`](https://github.com/sharkdp/hyperfine) command-line tool installed on your
  system.

To benchmark resolution against pip-compile, Poetry, and PDM:

```shell
fv run resolver \
    --uv-project \
    --poetry \
    --pdm \
    --pip-compile \
    --benchmark resolve-warm --benchmark resolve-cold \
    --json \
    ../requirements/trio.in
```

To benchmark installation against pip-sync, Poetry, and PDM:

```shell
fv run resolver \
    --uv-project \
    --poetry \
    --pdm \
    --pip-sync \
    --benchmark install-warm --benchmark install-cold \
    --json \
    ../requirements/compiled/trio.txt
```

Both commands should be run from the `scripts/benchmark` directory.

After running the benchmark script, you can generate the corresponding graph via:

```shell
cargo run -p fv-dev --all-features render-benchmarks resolve-warm.json --title "Warm Resolution"
cargo run -p fv-dev --all-features render-benchmarks resolve-cold.json --title "Cold Resolution"
cargo run -p fv-dev --all-features render-benchmarks install-warm.json --title "Warm Installation"
cargo run -p fv-dev --all-features render-benchmarks install-cold.json --title "Cold Installation"
```

You need to install the [Roboto Font](https://fonts.google.com/specimen/Roboto) if the labels are
missing in the generated graph.

## Troubleshooting

### Flaky benchmarks

If you're seeing high variance when running the cold benchmarks, then it's likely that you're
running into throttling or DDoS prevention from your ISP. In that case, ISPs forcefully terminate
TCP connections with a TCP reset. A possible workaround is to connect to VPN to bypass your ISP's
filtering mechanism.
