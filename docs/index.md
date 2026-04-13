# fyn

An extremely fast Python package and project manager, written in Rust.

## Start here

Pick the path that matches how you use fyn:

- New to fyn: [installation](./getting-started/installation.md),
  [first steps](./getting-started/first-steps.md), and [getting help](./getting-started/help.md)
- Working on a project: [projects guide](./guides/projects.md)
- Running scripts: [scripts guide](./guides/scripts.md)
- Using developer tools: [tools guide](./guides/tools.md)
- Looking for flags or config details: [CLI reference](./reference/cli.md),
  [settings](./reference/settings.md), and [environment variables](./reference/environment.md)

<p align="center">
  <img alt="Shows a bar chart with benchmark results." src="https://github.com/astral-sh/uv/assets/1309177/629e59c0-9c6e-4013-9ad4-adb2bcf5080d#only-light">
</p>

<p align="center">
  <img alt="Shows a bar chart with benchmark results." src="https://github.com/astral-sh/uv/assets/1309177/03aa9163-1c79-4a87-a31d-7a9311ed9310#only-dark">
</p>

<p align="center">
  <i>Installing <a href="https://trio.readthedocs.io/">Trio</a>'s dependencies with a warm cache.</i>
</p>

## Highlights

- A single tool to replace `pip`, `pip-tools`, `pipx`, `poetry`, `pyenv`, `twine`, `virtualenv`, and
  more.
- [10-100x faster](https://github.com/oha/fyn/blob/main/BENCHMARKS.md) than `pip`.
- Provides [comprehensive project management](#projects), with a
  [universal lockfile](./concepts/projects/layout.md#the-lockfile).
- [Runs scripts](#scripts), with support for
  [inline dependency metadata](./guides/scripts.md#declaring-script-dependencies).
- [Installs and manages](#python-versions) Python versions.
- [Runs and installs](#tools) tools published as Python packages.
- Includes a [pip-compatible interface](#the-pip-interface) for a performance boost with a familiar
  CLI.
- Supports Cargo-style [workspaces](./concepts/projects/workspaces.md) for scalable projects.
- Disk-space efficient, with a [global cache](./concepts/cache.md) for dependency deduplication.
- Installable without Rust or Python via prebuilt distributions or `pip`.
- Supports macOS, Linux, and Windows.

fyn started as an independent community fork of [uv](https://github.com/astral-sh/uv) and now has
its own commands and configuration surface.

## Installation

Install fyn with `pipx`:

```console
$ pipx install fyn
```

Then, check out the [first steps](./getting-started/first-steps.md) or read on for a brief overview.

!!! tip

    fyn may also be installed with standalone release assets, Homebrew, and more. See all of the methods on the
    [installation page](./getting-started/installation.md).

## Projects

fyn manages project dependencies and environments, with support for lockfiles, workspaces, and more,
similar to `rye` or `poetry`:

```console
$ fyn init example
Initialized project `example` at `/home/user/example`

$ cd example

$ fyn add ruff
Creating virtual environment at: .venv
Resolved 2 packages in 170ms
   Built example @ file:///home/user/example
Prepared 2 packages in 627ms
Installed 2 packages in 1ms
 + example==0.1.0 (from file:///home/user/example)
 + ruff==0.5.4

$ fyn run ruff check
All checks passed!

$ fyn lock
Resolved 2 packages in 0.33ms

$ fyn sync
Resolved 2 packages in 0.70ms
Checked 1 package in 0.02ms
```

### Inspecting and entering the project environment

Use `fyn status` to see whether fyn considers the current directory a managed project, whether
`fyn.lock` is present, and which environment and Python interpreter it is using:

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

Use `fyn upgrade` to refresh all dependencies or only the packages you name:

```console
$ fyn upgrade
$ fyn upgrade requests flask
```

Use `fyn shell` to open a new shell with the project environment activated:

```console
$ fyn shell
success: Activated virtual environment at .venv
Type exit to deactivate.
```

When you need to choose a PyTorch backend for the current machine without rewriting project
metadata, use `fyn torch doctor`:

```console
$ fyn torch doctor
PyTorch doctor
recommended backend: cu130
```

See the [project guide](./guides/projects.md) to get started.

fyn also supports building and publishing projects, even if they're not managed with fyn. See the
[packaging guide](./guides/package.md) to learn more.

## Scripts

fyn manages dependencies and environments for single-file scripts.

Create a new script and add inline metadata declaring its dependencies:

```console
$ echo 'import requests; print(requests.get("https://astral.sh"))' > example.py

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

See the [scripts guide](./guides/scripts.md) to get started.

## Tools

fyn executes and installs command-line tools provided by Python packages, similar to `pipx`.

Run a tool in an ephemeral environment using `fynx` (an alias for `fyn tool run`):

```console
$ fynx pycowsay 'hello world!'
Resolved 1 package in 167ms
Installed 1 package in 9ms
 + pycowsay==0.0.0.2
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
 + ruff==0.5.4
Installed 1 executable: ruff

$ ruff --version
ruff 0.5.4
```

See the [tools guide](./guides/tools.md) to get started.

## Python versions

fyn installs Python and allows quickly switching between versions.

Install multiple Python versions:

```console
$ fyn python install 3.10 3.11 3.12
Searching for Python versions matching: Python 3.10
Searching for Python versions matching: Python 3.11
Searching for Python versions matching: Python 3.12
Installed 3 versions in 3.42s
 + cpython-3.10.14-macos-aarch64-none
 + cpython-3.11.9-macos-aarch64-none
 + cpython-3.12.4-macos-aarch64-none
```

Download Python versions as needed:

```console
$ fyn venv --python 3.12.0
Using CPython 3.12.0
Creating virtual environment at: .venv
Activate with: source .venv/bin/activate

$ fyn run --python pypy@3.8 -- python
Python 3.8.16 (a9dbdca6fc3286b0addd2240f11d97d8e8de187a, Dec 29 2022, 11:45:30)
[PyPy 7.3.11 with GCC Apple LLVM 13.1.6 (clang-1316.0.21.2.5)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>>>
```

Use a specific Python version in the current directory:

```console
$ fyn python pin 3.11
Pinned `.python-version` to `3.11`
```

See the [installing Python guide](./guides/install-python.md) to get started.

## The pip interface

fyn provides a fast, pip-compatible interface for common `pip`, `pip-tools`, and `virtualenv`
workflows.

fyn extends their interfaces with advanced features, such as dependency version overrides,
platform-independent resolutions, reproducible resolutions, alternative resolution strategies, and
more.

For many common workflows, you can switch to the `fyn pip` interface with minimal changes and keep
the same overall workflow shape, while getting a 10-100x speedup.

Mutating `fyn pip` commands operate directly on the selected environment. They do not update
`pyproject.toml` or `fyn.lock`; for managed project workflows, use `fyn add`, `fyn remove`,
`fyn sync`, or `fyn upgrade` instead.

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
Using CPython 3.12.3
Creating virtual environment at: .venv
Activate with: source .venv/bin/activate
```

Install the locked requirements:

```console
$ fyn pip sync requirements.txt
Resolved 43 packages in 11ms
Installed 43 packages in 208ms
 + babel==2.15.0
 + black==24.4.2
 + certifi==2024.7.4
 ...
```

See the [pip interface documentation](./pip/index.md) to get started.

## Learn more

See the [first steps](./getting-started/first-steps.md) or jump straight to the
[guides](./guides/index.md) to start using fyn.
