# fyn

An extremely fast Python package and project manager, written in Rust.

**fyn** is an independent Python package manager built on [uv](https://github.com/astral-sh/uv)'s
foundation, with telemetry removed, new features added, and long-standing bugs fixed. See
[MANIFESTO.md](MANIFESTO.md) for the full story.

## Highlights

- A single tool to replace `pip`, `pip-tools`, `pipx`, `poetry`, `pyenv`, `twine`, `virtualenv`, and
  more.
- 10-100x faster than `pip`.
- Provides [comprehensive project management](#projects), with a universal lockfile.
- Built-in [task runner](#task-runner) — define and run project tasks in `pyproject.toml`.
- [Activates virtual environments](#shell-activation) with `fyn shell`.
- [Upgrades dependencies](#upgrade-dependencies) in one command with `fyn upgrade`.
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

From [PyPI](https://pypi.org/project/fyn/):

```bash
# With pip.
pip install fyn
```

```bash
# Or pipx.
pipx install fyn
```

Or build from source:

```bash
cargo install --path crates/fyn
```

See the command line reference documentation with `fyn help`.

## Features

### Projects

fyn manages project dependencies and environments, with support for lockfiles, workspaces, and more,
similar to `rye` or `poetry`:

```console
$ fyn init example
Initialized project `example` at `/home/user/example`

$ cd example

$ fyn add ruff
Creating virtual environment at: .venv
Resolved 2 packages in 170ms
Installed 2 packages in 1ms
 + ruff==0.5.0

$ fyn run ruff check
All checks passed!

$ fyn lock
Resolved 2 packages in 0.33ms

$ fyn sync
Resolved 2 packages in 0.70ms
Checked 1 package in 0.02ms
```

### Task runner

Define tasks in your `pyproject.toml` and run them with `fyn run`:

```toml
[tool.fyn.tasks]
test = "pytest -xvs"
lint = "ruff check ."
format = { cmd = "ruff format .", description = "Format code" }
check = { chain = ["lint", "test"], description = "Lint then test" }
```

```console
$ fyn run test
# runs pytest -xvs

$ fyn run test -- -k mytest
# extra args are passed through

$ fyn run --list-tasks
Available tasks:
  check    Lint then test
  format   Format code
  lint     ruff check .
  test     pytest -xvs
```

### Shell activation

Activate the project's virtual environment in a new shell:

```console
$ fyn shell
success: Activated virtual environment at .venv
Type exit to deactivate.
```

Works with bash, zsh, fish, nushell, powershell, and cmd.

### Upgrade dependencies

Upgrade all or specific dependencies in one command:

```console
$ fyn upgrade
info: Upgrading all dependencies...
success: Dependencies upgraded successfully.

$ fyn upgrade requests flask
info: Upgrading: requests, flask
success: Dependencies upgraded successfully.
```

Supports `--dry-run` and `--no-sync`.

### Scripts

fyn manages dependencies and environments for single-file scripts.

Create a new script and add inline metadata declaring its dependencies:

```console
$ echo 'import requests; print(requests.get("https://example.com"))' > example.py

$ fyn add --script example.py requests
Updated `example.py`
```

Then, run the script in an isolated virtual environment:

```console
$ fyn run example.py
Reading inline script metadata from: example.py
Installed 5 packages in 12ms
<Response [200]>
```

### Tools

fyn executes and installs command-line tools provided by Python packages, similar to `pipx`.

Run a tool in an ephemeral environment using `fynx` (an alias for `fyn tool run`):

```console
$ fynx pycowsay 'hello world!'
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

Install a tool with `fyn tool install`:

```console
$ fyn tool install ruff
Resolved 1 package in 6ms
Installed 1 package in 2ms
 + ruff==0.5.0
Installed 1 executable: ruff

$ ruff --version
ruff 0.5.0
```

### Python versions

fyn installs Python and allows quickly switching between versions.

Install multiple Python versions:

```console
$ fyn python install 3.12 3.13 3.14
Installed 3 versions in 972ms
 + cpython-3.12.12-macos-aarch64-none
 + cpython-3.13.9-macos-aarch64-none
 + cpython-3.14.0-macos-aarch64-none
```

Use a specific Python version in the current directory:

```console
$ fyn python pin 3.11
Pinned `.python-version` to `3.11`
```

### The pip interface

fyn provides a drop-in replacement for common `pip`, `pip-tools`, and `virtualenv` commands.

Migrate to fyn without changing your existing workflows — and experience a 10-100x speedup — with
the `fyn pip` interface.

Compile requirements into a platform-independent requirements file:

```console
$ fyn pip compile requirements.in \
   --universal \
   --output-file requirements.txt
Resolved 43 packages in 12ms
```

Create a virtual environment:

```console
$ fyn venv
Using Python 3.12.3
Creating virtual environment at: .venv
Activate with: source .venv/bin/activate
```

Install the locked requirements:

```console
$ fyn pip sync requirements.txt
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
UV_LOCKFILE=linux.lock fyn lock
UV_LOCKFILE=macos.lock fyn lock
```

### Private index support

Environment variables work in index URLs — useful for private indexes with credentials:

```toml
[[tool.fyn.index]]
name = "private"
url = "https://${PYPI_TOKEN}@pypi.example.com/simple/"
```

Explicit indexes are also respected for transitive dependencies — you don't have to list every
internal package as a direct dependency anymore.

## Migrating from fyn

fyn is a drop-in replacement for fyn. Same config files, same `pyproject.toml` settings, same
`fyn.lock` format, same `UV_*` environment variables. The only change is the binary name.

```bash
# before
fyn sync
fyn run pytest
fynx ruff check .

# after
fyn sync
fyn run pytest
fynx ruff check .
```

## Contributing

We are passionate about supporting contributors of all levels of experience and would love to see
you get involved in the project. See the [contributing guide](CONTRIBUTING.md) to get started.

## FAQ

#### What platforms does fyn support?

The same ones as uv: macOS, Linux, and Windows, across x86_64 and aarch64.

#### Is fyn compatible with fyn?

Yes. Same config format, same lockfile, same environment variables. You can switch between them
freely on the same project.

#### What's different from fyn?

See [MANIFESTO.md](MANIFESTO.md) for the full comparison, or the table below for a quick summary:

| Feature                            | fyn                       | fyn                 |
| ---------------------------------- | ------------------------- | ------------------- |
| Telemetry (linehaul)               | Sends OS, Python, CI info | None                |
| Task runner                        | Not available             | `[tool.fyn.tasks]`  |
| `shell` command                    | Not available             | `fyn shell`         |
| `upgrade` command                  | Must chain two commands   | `fyn upgrade`       |
| Cache size limit                   | No limit                  | `UV_CACHE_MAX_SIZE` |
| Custom lockfile name               | Not available             | `UV_LOCKFILE`       |
| Explicit index for transitive deps | Broken                    | Fixed               |
| Env vars in index URLs             | Only in requirements.txt  | Everywhere          |

## Acknowledgements

fyn's dependency resolver uses [PubGrub](https://github.com/pubgrub-rs/pubgrub) under the hood.
We're grateful to the PubGrub maintainers, especially [Jacob Finkelman](https://github.com/Eh2406),
for their support.

fyn's core is derived from [uv](https://github.com/astral-sh/uv) by Astral.

fyn's Git implementation is based on [Cargo](https://github.com/rust-lang/cargo).

Some of fyn's optimizations are inspired by the great work we've seen in [pnpm](https://pnpm.io/),
[Orogene](https://github.com/orogene/orogene), and [Bun](https://github.com/oven-sh/bun). We've also
learned a lot from Nathaniel J. Smith's [Posy](https://github.com/njsmith/posy) and adapted its
trampoline for Windows support.

## License

fyn is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in fyn
by you, as defined in the Apache-2.0 license, shall be dually licensed as above, without any
additional terms or conditions.
