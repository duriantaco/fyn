# fyn

An extremely fast Python package and project manager, written in Rust.

**fyn** is an independent community fork of [uv](https://github.com/astral-sh/uv). It started on
uv's foundation, but it now has its own commands, settings, defaults, and behavior, alongside
reduced package-index request metadata, added features, and long-standing bug fixes. See
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
- Reduced package-index request metadata — compared with upstream uv, fyn sends a minimal
  `fyn/<version>` `User-Agent` header to package indexes such as PyPI, instead of `uv/<version>`
  plus the extra LineHaul environment metadata uv included. This reduces what is exposed in that
  header, but package indexes still receive normal network and request information.
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

If you pass a path, `fyn shell` activates that environment directly. Otherwise it uses `VIRTUAL_ENV`
when set, then the discovered project environment, then a local `.venv`. Use `--no-project` to skip
project discovery and only check the current directory.

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

`fyn upgrade` is the convenience form of running `fyn lock --upgrade` and then `fyn sync`.

### Project status

Inspect the current project and environment state:

```console
$ fyn status
current directory: /home/user/example
project directory: /home/user/example
managed project: yes
workspace root: /home/user/example
pyproject.toml: yes
fyn.lock: yes
pip-in-project: warn
environment: /home/user/example/.venv
python: /home/user/example/.venv/bin/python3 (3.12.0)
```

Use `--check` to fail when obvious project checks do not pass, or `--json` for scripting and editor
integrations.

### PyTorch backend diagnosis

Inspect the current machine and environment before installing or reinstalling PyTorch:

```console
$ fyn torch doctor
PyTorch doctor
recommended backend: cu130

next command:
  fyn pip install torch torchvision torchaudio --torch-backend=cu130
```

Use `--json` for scripting. `fyn torch doctor` reports the recommendation and current package state,
but does not modify `pyproject.toml`.

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

fyn provides a fast, pip-compatible interface for common `pip`, `pip-tools`, and `virtualenv`
workflows.

For many common workflows, you can switch to the `fyn pip` interface with minimal changes and keep
the same overall workflow shape, while getting a 10-100x speedup.

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

Explicit indexes are also respected for transitive dependencies, so you don't have to list every
internal package as a direct dependency.

## Migrating from uv

fyn is close to uv, but not a zero-edit rename. Most command-line workflows and `UV_*` environment
variables carry over, but project metadata and lockfile names differ.

```bash
# 1. Rename your lockfile
mv uv.lock fyn.lock

# 2. In pyproject.toml, rename [tool.uv] to [tool.fyn]
sed -i 's/\[tool\.uv\]/[tool.fyn]/' pyproject.toml

# 3. Use fyn instead of uv
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

#### Is fyn compatible with uv?

At the workflow level, often yes, but not as a drop-in replacement. Many commands and `UV_*`
environment variables carry over, but fyn now has fork-specific commands, config, defaults, and
behavior. Projects still need `[tool.uv]` renamed to `[tool.fyn]` and `uv.lock` renamed to
`fyn.lock` unless you override the lockfile name.

#### What's different from uv?

See [MANIFESTO.md](MANIFESTO.md) for the fuller comparison, or the table below for some of the
larger user-visible differences:

| Area                          | uv                                    | fyn                      |
| ----------------------------- | ------------------------------------- | ------------------------ | ----- | ------ |
| Config namespace and lockfile | `[tool.uv]`, `uv.lock`                | `[tool.fyn]`, `fyn.lock` |
| Package index User-Agent      | `uv/<version>` plus LineHaul metadata | Minimal `fyn/<version>`  |
| Task runner                   | No `[tool.uv.tasks]`                  | `[tool.fyn.tasks]`       |
| `shell` command               | No `uv shell`                         | `fyn shell`              |
| `upgrade` command             | No `uv upgrade`                       | `fyn upgrade`            |
| `status` command              | No `uv status`                        | `fyn status`             |
| `torch doctor` command        | No `uv torch doctor`                  | `fyn torch doctor`       |
| Managed-project `pip` policy  | No `pip-in-project` setting           | `pip-in-project = warn   | error | allow` |
| Cache size limit              | No `UV_CACHE_MAX_SIZE`                | `UV_CACHE_MAX_SIZE`      |
| Custom lockfile name          | No `UV_LOCKFILE`                      | `UV_LOCKFILE`            |

## Acknowledgements

fyn's dependency resolver uses [PubGrub](https://github.com/pubgrub-rs/pubgrub) under the hood.
We're grateful to the PubGrub maintainers, especially [Jacob Finkelman](https://github.com/Eh2406),
for their support.

fyn started as a fork of [uv](https://github.com/astral-sh/uv) by Astral and still shares
substantial ancestry with it.

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
