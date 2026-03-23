# fv

An extremely fast Python package and project manager, written in Rust.

**fv** is an independent Python package manager built on [uv](https://github.com/astral-sh/uv)'s
foundation, with telemetry removed, new features added, and long-standing bugs fixed. See
[MANIFESTO.md](MANIFESTO.md) for the full story.

## Highlights

- A single tool to replace `pip`, `pip-tools`, `pipx`, `poetry`, `pyenv`, `twine`, `virtualenv`, and
  more.
- 10-100x faster than `pip`.
- Provides [comprehensive project management](#projects), with a universal lockfile.
- Built-in [task runner](#task-runner) — define and run project tasks in `pyproject.toml`.
- [Activates virtual environments](#shell-activation) with `fv shell`.
- [Upgrades dependencies](#upgrade-dependencies) in one command with `fv upgrade`.
- [Runs scripts](#scripts), with support for inline dependency metadata.
- [Installs and manages](#python-versions) Python versions.
- [Runs and installs](#tools) tools published as Python packages.
- Includes a [pip-compatible interface](#the-pip-interface) for a performance boost with a familiar
  CLI.
- Supports Cargo-style workspaces for scalable projects.
- Disk-space efficient, with a global cache for dependency deduplication.
- No telemetry — your installs are your business.
- Supports macOS, Linux, and Windows.

## Installation

From [PyPI](https://pypi.org/project/fv/):

```bash
# With pip.
pip install fv
```

```bash
# Or pipx.
pipx install fv
```

Or build from source:

```bash
cargo install --path crates/fv
```

See the command line reference documentation with `fv help`.

## Features

### Projects

fv manages project dependencies and environments, with support for lockfiles, workspaces, and more,
similar to `rye` or `poetry`:

```console
$ fv init example
Initialized project `example` at `/home/user/example`

$ cd example

$ fv add ruff
Creating virtual environment at: .venv
Resolved 2 packages in 170ms
Installed 2 packages in 1ms
 + ruff==0.5.0

$ fv run ruff check
All checks passed!

$ fv lock
Resolved 2 packages in 0.33ms

$ fv sync
Resolved 2 packages in 0.70ms
Checked 1 package in 0.02ms
```

### Task runner

Define tasks in your `pyproject.toml` and run them with `fv run`:

```toml
[tool.fv.tasks]
test = "pytest -xvs"
lint = "ruff check ."
format = { cmd = "ruff format .", description = "Format code" }
check = { chain = ["lint", "test"], description = "Lint then test" }
```

```console
$ fv run test
# runs pytest -xvs

$ fv run test -- -k mytest
# extra args are passed through

$ fv run --list-tasks
Available tasks:
  check    Lint then test
  format   Format code
  lint     ruff check .
  test     pytest -xvs
```

### Shell activation

Activate the project's virtual environment in a new shell:

```console
$ fv shell
success: Activated virtual environment at .venv
Type exit to deactivate.
```

Works with bash, zsh, fish, nushell, powershell, and cmd.

### Upgrade dependencies

Upgrade all or specific dependencies in one command:

```console
$ fv upgrade
info: Upgrading all dependencies...
success: Dependencies upgraded successfully.

$ fv upgrade requests flask
info: Upgrading: requests, flask
success: Dependencies upgraded successfully.
```

Supports `--dry-run` and `--no-sync`.

### Scripts

fv manages dependencies and environments for single-file scripts.

Create a new script and add inline metadata declaring its dependencies:

```console
$ echo 'import requests; print(requests.get("https://example.com"))' > example.py

$ fv add --script example.py requests
Updated `example.py`
```

Then, run the script in an isolated virtual environment:

```console
$ fv run example.py
Reading inline script metadata from: example.py
Installed 5 packages in 12ms
<Response [200]>
```

### Tools

fv executes and installs command-line tools provided by Python packages, similar to `pipx`.

Run a tool in an ephemeral environment using `fvx` (an alias for `fv tool run`):

```console
$ fvx pycowsay 'hello world!'
Resolved 1 package in 167ms
Installed 1 package in 9ms
  """

  ------------
< hello world! >
  ------------
   \   ^__^
    \  (oo)\_______
       (__)\       )\/\
           ||----w |
           ||     ||
```

Install a tool with `fv tool install`:

```console
$ fv tool install ruff
Resolved 1 package in 6ms
Installed 1 package in 2ms
 + ruff==0.5.0
Installed 1 executable: ruff

$ ruff --version
ruff 0.5.0
```

### Python versions

fv installs Python and allows quickly switching between versions.

Install multiple Python versions:

```console
$ fv python install 3.12 3.13 3.14
Installed 3 versions in 972ms
 + cpython-3.12.12-macos-aarch64-none
 + cpython-3.13.9-macos-aarch64-none
 + cpython-3.14.0-macos-aarch64-none
```

Use a specific Python version in the current directory:

```console
$ fv python pin 3.11
Pinned `.python-version` to `3.11`
```

### The pip interface

fv provides a drop-in replacement for common `pip`, `pip-tools`, and `virtualenv` commands.

Migrate to fv without changing your existing workflows — and experience a 10-100x speedup — with the
`fv pip` interface.

Compile requirements into a platform-independent requirements file:

```console
$ fv pip compile requirements.in \
   --universal \
   --output-file requirements.txt
Resolved 43 packages in 12ms
```

Create a virtual environment:

```console
$ fv venv
Using Python 3.12.3
Creating virtual environment at: .venv
Activate with: source .venv/bin/activate
```

Install the locked requirements:

```console
$ fv pip sync requirements.txt
Resolved 43 packages in 11ms
Installed 43 packages in 208ms
 + babel==2.15.0
 + certifi==2024.7.4
 ...
```

### Cache size limit

Keep your cache from growing unbounded:

```bash
export UV_CACHE_MAX_SIZE=2G
```

Oldest entries are automatically pruned after every command when the cache exceeds the limit.
Supports `K`, `M`, `G`, and `T` suffixes.

### Custom lockfile name

Use different lockfiles for different environments:

```bash
UV_LOCKFILE=linux.lock fv lock
UV_LOCKFILE=macos.lock fv lock
```

### Private index support

Environment variables work in index URLs — useful for private indexes with credentials:

```toml
[[tool.fv.index]]
name = "private"
url = "https://${PYPI_TOKEN}@pypi.example.com/simple/"
```

Explicit indexes are also respected for transitive dependencies — you don't have to list every
internal package as a direct dependency anymore.

## Migrating from fv

fv is a drop-in replacement for fv. Same config files, same `pyproject.toml` settings, same
`fv.lock` format, same `UV_*` environment variables. The only change is the binary name.

```bash
# before
fv sync
fv run pytest
fvx ruff check .

# after
fv sync
fv run pytest
fvx ruff check .
```

## Contributing

We are passionate about supporting contributors of all levels of experience and would love to see
you get involved in the project. See the [contributing guide](CONTRIBUTING.md) to get started.

## FAQ

#### What platforms does fv support?

The same ones as uv: macOS, Linux, and Windows, across x86_64 and aarch64.

#### Is fv compatible with fv?

Yes. Same config format, same lockfile, same environment variables. You can switch between them
freely on the same project.

#### What's different from fv?

See [MANIFESTO.md](MANIFESTO.md) for the full comparison, or the table below for a quick summary:

| Feature                            | fv                        | fv                  |
| ---------------------------------- | ------------------------- | ------------------- |
| Telemetry (linehaul)               | Sends OS, Python, CI info | None                |
| Task runner                        | Not available             | `[tool.fv.tasks]`   |
| `shell` command                    | Not available             | `fv shell`          |
| `upgrade` command                  | Must chain two commands   | `fv upgrade`        |
| Cache size limit                   | No limit                  | `UV_CACHE_MAX_SIZE` |
| Custom lockfile name               | Not available             | `UV_LOCKFILE`       |
| Explicit index for transitive deps | Broken                    | Fixed               |
| Env vars in index URLs             | Only in requirements.txt  | Everywhere          |

## Acknowledgements

fv's dependency resolver uses [PubGrub](https://github.com/pubgrub-rs/pubgrub) under the hood. We're
grateful to the PubGrub maintainers, especially [Jacob Finkelman](https://github.com/Eh2406), for
their support.

fv's core is derived from [uv](https://github.com/astral-sh/uv) by Astral.

fv's Git implementation is based on [Cargo](https://github.com/rust-lang/cargo).

Some of fv's optimizations are inspired by the great work we've seen in [pnpm](https://pnpm.io/),
[Orogene](https://github.com/orogene/orogene), and [Bun](https://github.com/oven-sh/bun). We've also
learned a lot from Nathaniel J. Smith's [Posy](https://github.com/njsmith/posy) and adapted its
trampoline for Windows support.

## License

fv is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in fv
by you, as defined in the Apache-2.0 license, shall be dually licensed as above, without any
additional terms or conditions.
