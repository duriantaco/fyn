# fyn

**Python projects, under control.**

**fyn** is an independent community fork of [uv](https://github.com/astral-sh/uv), built around
reviewable, transactional dependency changes and a repository-local project workflow. It keeps a
fast uv-derived packaging foundation, then focuses on the work around the resolver: previewing
changes, keeping the manifest, lockfile, and environment aligned, explaining project state, and
encoding team policy in `pyproject.toml`.

See [MANIFESTO.md](https://github.com/duriantaco/fyn/blob/main/MANIFESTO.md) for the product
direction and the project's origins.

## Why fyn?

- **Review before writing.** `fyn lock diff` resolves in dry-run mode and shows package and metadata
  changes without touching `fyn.lock`.
- **Change dependencies as one transaction.** `fyn upgrade --dry-run` previews manifest and lockfile
  edits. A real upgrade updates `pyproject.toml`, `fyn.lock`, and the environment together, and
  rolls back the files if locking or syncing fails.
- **Explain the project.** `fyn why` shows why a package is present, `fyn audit --explain` includes
  dependency paths for findings, and `fyn status --check` turns missing or misdirected project
  environments and incompatible Python pins into actionable local or CI failures.
- **Put the team's workflow in the repository.** Define tasks in `[tool.fyn.tasks]`, enforce the fyn
  version, guard managed environments from direct `fyn pip` mutations, and run the same commands
  locally and in CI.
- **Keep package-source policy explicit.** Pin dependencies to named indexes in project metadata,
  keep the dependency-confusion-resistant `first-index` default, and redirect a declared index name
  to a local or CI mirror without changing the project policy.
- **Use the broader Python toolchain from one binary.** Manage projects, scripts, tools, Python
  versions, workspaces, builds, and publishing on macOS, Linux, and Windows.

Current uv and fyn share a large foundation, including high-performance resolution, universal
locking, Python and tool management, dependency auditing, cache controls, and PyTorch backend
selection. fyn treats those as shared capabilities rather than product differentiators. Its focus is
the inspectable, policy-aware workflow around them.

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

See the command line reference with `fyn help`.

## Documentation

The live docs site is [duriantaco.github.io/fyn](https://duriantaco.github.io/fyn/). The source of
truth still lives in [`docs/`](https://github.com/duriantaco/fyn/tree/main/docs) in the repository.

Start here:

- [Documentation home](https://duriantaco.github.io/fyn/)
- [Getting started](https://duriantaco.github.io/fyn/getting-started/first-steps/)
- [Working on projects](https://duriantaco.github.io/fyn/guides/projects/)
- [Running scripts](https://duriantaco.github.io/fyn/guides/scripts/)
- [Command reference](https://duriantaco.github.io/fyn/reference/cli/)

For CLI-specific help, use `fyn help` or `fyn help <command>`.

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

$ fyn lock diff
No lockfile changes detected

$ fyn sync
Resolved 2 packages in 0.70ms
Checked 1 package in 0.02ms
```

Use `fyn lock diff` to preview how dependency or project metadata edits would change `fyn.lock`
without writing it. The diff reports added, removed, and changed packages, plus metadata-only
lockfile updates, which is useful before reviewing or committing a lockfile change.

### Task runner

Define tasks in your `pyproject.toml` and run them with `fyn run`:

```toml
[tool.fyn.tasks]
test = { cmd = "pytest -xvs", env = { PYTHONWARNINGS = "error" } }
lint = "ruff check ."
format = { cmd = "ruff format .", description = "Format code" }
check = ["ruff check .", "pytest -q"]
```

```console
$ fyn run test
# runs pytest -xvs

$ fyn run check
# runs each command in order, stopping on the first failure

$ fyn run test -- -k mytest
# extra args are passed through

$ fyn run --list-tasks
Available tasks:
  check    (command sequence)
  format   Format code
  lint     ruff check .
  test     pytest -xvs
```

Direct command arrays run in order and stop on the first failure. Task `env` values are applied to
the spawned command. For chained tasks, parent `env` values are inherited by child tasks, and child
task values take precedence. Extra arguments are supported for `cmd` tasks, but not for direct
command arrays or chained tasks.

Run the same named task across child workspace members with `--workspace`:

```console
$ fyn run --workspace test
$ fyn run --workspace --filter api,worker test
$ fyn run --workspace --sequential test
$ fyn run --workspace --list-tasks
```

Unless `--no-sync` is supplied, fyn syncs the shared workspace environment once with all packages
installed, then runs the task only in child members that define it. Active, declared workspace
dependencies run before their dependents, while independent members run in parallel unless
`--sequential` is supplied. `--filter` accepts exact package names and can be repeated or given a
comma-separated list. The workspace root is excluded, so it can define an aggregate task that
invokes `fyn run --workspace` without recursively running itself.

If `fyn run` cannot spawn a command, it tries to point you at the next step instead of stopping at a
raw OS error: `fyn run --list-tasks` for likely task typos, `fyn tool run <command>` for missing
Python-provided executables, or a path-specific hint for missing `./script`-style commands.

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
success: Dependencies upgraded successfully.

$ fyn upgrade requests flask
success: Dependencies upgraded successfully.

$ fyn upgrade --exclude django
success: Dependencies upgraded successfully.
```

`fyn upgrade` resolves the newest versions of the selected direct dependencies, widens only the
version constraints that block those versions, updates `pyproject.toml` and `fyn.lock`, and syncs
the environment. Lower bounds, exclusions, extras, markers, and the existing constraint style are
preserved where possible.

Use `--dry-run` to preview both lock and requirement changes, `--no-sync` to update the manifest and
lockfile without touching the environment, and `--exclude PACKAGE` to omit dependencies from an
all-package upgrade. Manifest and lockfile changes are rolled back if locking or syncing fails. This
first version upgrades `[project].dependencies` in single-project workspaces.

### Explain dependencies and audit findings

Show the project dependency paths that include a package:

```console
$ fyn why numpy
numpy is included because:
project v0.1.0 -> pandas v2.2.1 -> numpy v1.26.4
project v0.1.0 -> scikit-learn v1.4.1.post1 -> scipy v1.12.0 -> numpy v1.26.4
```

Use `--universal` to ignore the current platform and Python version, or the dependency group flags
such as `--group`, `--only-group`, and `--no-dev` to explain the selected project view.

Audit the selected project dependencies for known vulnerabilities and adverse package statuses. Add
`--explain` to include the dependency path for each finding:

```console
$ fyn audit --explain
```

Dependency auditing is also available in current uv. fyn's project workflow puts its results next to
the same graph explanation, lockfile preview, and project-health commands used before and after a
dependency change.

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
project environment: /home/user/example/.venv
environment: /home/user/example/.venv
python: /home/user/example/.venv/bin/python3 (3.12.0)
```

Use `--check` to fail when obvious project checks do not pass, or `--json` for scripting and editor
integrations. In managed projects, `--check` also reports missing or mismatched project environments
and `.python-version` pins that do not satisfy `requires-python`, and prints `hint:` lines with the
suggested fix.

```console
$ fyn status --check
...
check: failed
issue: environment not found
hint: Run `fyn sync` or `fyn venv` to create the project environment.
```

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

Use `--python-downloads-json-url <source>` when you need `fyn python pin` to resolve against a
custom Python downloads manifest instead of the default bundled metadata.

### The pip interface

fyn provides a fast, pip-compatible interface for common `pip`, `pip-tools`, and `virtualenv`
workflows.

For many common workflows, you can switch to the `fyn pip` interface with minimal changes and keep
the same overall workflow shape.

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

### Package index policy

Keep source selection reviewable by declaring a named, explicit index and pinning packages to it in
project metadata:

```toml
[[tool.fyn.index]]
name = "internal"
url = "https://packages.example.com/simple"
explicit = true

[tool.fyn.sources]
acme = { index = "internal" }
```

fyn uses the dependency-confusion-resistant `first-index` strategy by default. A user- or
system-level `fyn.toml` can replace the URL for the already-declared `internal` name, so developer
machines and CI can use different mirrors without changing which logical source the project allows:

```toml
[[index]]
name = "internal"
url = "https://mirror.example.com/simple"
explicit = true
```

Local configuration cannot introduce a new source pin: the index name and package association must
remain declared in the project or workspace metadata.

## Migrating from uv

fyn is close to uv, but not a zero-edit rename. Most command-line workflows and `UV_*` environment
variables carry over. Within `pyproject.toml`, fyn reads `[tool.fyn]` and falls back to `[tool.uv]`
when `[tool.fyn]` is absent, which makes it possible to evaluate existing project configuration
before rewriting it. That fallback does not apply to standalone configuration files.

fyn uses `fyn.lock` by default. For a dedicated migration, work on a branch, rename `uv.lock` to
`fyn.lock`, rename a project-level `uv.toml` to `fyn.toml` if present, and rename every
`[tool.uv...]` table header in `pyproject.toml` to its `[tool.fyn...]` equivalent. Copy any user- or
system-level settings you still need into fyn's corresponding `fyn/fyn.toml`
[configuration directory](https://duriantaco.github.io/fyn/concepts/configuration-files/). Then
preview the result before syncing the environment:

```console
$ fyn lock diff
$ fyn sync
$ fyn status --check
```

Review the resulting project metadata and lockfile as normal repository changes rather than treating
migration as a blind search-and-replace.

## Contributing

We are passionate about supporting contributors of all levels of experience and would love to see
you get involved in the project. See the
[contributing guide](https://github.com/duriantaco/fyn/blob/main/CONTRIBUTING.md) to get started.

## FAQ

#### What platforms does fyn support?

The same ones as uv: macOS, Linux, and Windows, across x86_64 and aarch64.

#### Is fyn compatible with uv?

At the workflow level, often yes, but not as a drop-in replacement. Many commands and `UV_*`
environment variables carry over, but fyn now has fork-specific commands, config, defaults, and
behavior. In `pyproject.toml`, fyn falls back to `[tool.uv]` configuration when `[tool.fyn]` is
absent, but its canonical project namespace is `[tool.fyn]`, standalone configuration file is
`fyn.toml`, and default lockfile is `fyn.lock`.

#### What's different from uv?

See [MANIFESTO.md](https://github.com/duriantaco/fyn/blob/main/MANIFESTO.md) for the fuller
comparison. Current uv and fyn share many individual capabilities, including auditing, cache
controls, and PyTorch backend selection. fyn's distinction is the direction of the complete project
workflow:

- dependency changes can be previewed and applied across the manifest, lockfile, and environment,
  with manifest and lockfile edits rolled back if locking or syncing fails
- `lock diff`, `why`, `audit --explain`, and `status --check` make change impact and project state
  visible to people and CI
- project policy covers repository commands, the required fyn version, managed-environment
  guardrails, and named package sources
- fyn uses its own `[tool.fyn]` namespace, `fyn.lock`, command surface, defaults, and release policy

The goal is not to claim that uv lacks every underlying primitive. It is to make reviewability,
transactional change, and repository-owned policy the center of fyn's product decisions.

## Acknowledgements

fyn's dependency resolver uses [PubGrub](https://github.com/pubgrub-rs/pubgrub) under the hood.
We're grateful to the PubGrub maintainers, especially [Jacob Finkelman](https://github.com/Eh2406),
for their support.

fyn started as a fork of [uv](https://github.com/astral-sh/uv) by Astral and still shares
substantial ancestry with it.

Some of fyn's workflow UX, especially around task-running and future workflow ergonomics, has also
been informed by [Hatch](https://github.com/pypa/hatch).

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
