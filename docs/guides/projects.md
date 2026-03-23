---
title: Working on projects
description:
  A guide to using fv to create and manage Python projects, including adding dependencies, running
  commands, and building publishable distributions.
---

# Working on projects

fv supports managing Python projects, which define their dependencies in a `pyproject.toml` file.

## Creating a new project

You can create a new Python project using the `fv init` command:

```console
$ fv init hello-world
$ cd hello-world
```

Alternatively, you can initialize a project in the working directory:

```console
$ mkdir hello-world
$ cd hello-world
$ fv init
```

fv will create the following files:

```text
├── .gitignore
├── .python-version
├── README.md
├── main.py
└── pyproject.toml
```

The `main.py` file contains a simple "Hello world" program. Try it out with `fv run`:

```console
$ fv run main.py
Hello from hello-world!
```

## Project structure

A project consists of a few important parts that work together and allow fv to manage your project.
In addition to the files created by `fv init`, fv will create a virtual environment and `fv.lock`
file in the root of your project the first time you run a project command, i.e., `fv run`,
`fv sync`, or `fv lock`.

A complete listing would look like:

```text
.
├── .venv
│   ├── bin
│   ├── lib
│   └── pyvenv.cfg
├── .python-version
├── README.md
├── main.py
├── pyproject.toml
└── fv.lock
```

### `pyproject.toml`

The `pyproject.toml` contains metadata about your project:

```toml title="pyproject.toml"
[project]
name = "hello-world"
version = "0.1.0"
description = "Add your description here"
readme = "README.md"
dependencies = []
```

You'll use this file to specify dependencies, as well as details about the project such as its
description or license. You can edit this file manually, or use commands like `fv add` and
`fv remove` to manage your project from the terminal.

!!! tip

    See the official [`pyproject.toml` guide](https://packaging.python.org/en/latest/guides/writing-pyproject-toml/)
    for more details on getting started with the `pyproject.toml` format.

You'll also use this file to specify fv [configuration options](../concepts/configuration-files.md)
in a [`[tool.fv]`](../reference/settings.md) section.

### `.python-version`

The `.python-version` file contains the project's default Python version. This file tells fv which
Python version to use when creating the project's virtual environment.

### `.venv`

The `.venv` folder contains your project's virtual environment, a Python environment that is
isolated from the rest of your system. This is where fv will install your project's dependencies.

See the [project environment](../concepts/projects/layout.md#the-project-environment) documentation
for more details.

### `fv.lock`

`fv.lock` is a cross-platform lockfile that contains exact information about your project's
dependencies. Unlike the `pyproject.toml` which is used to specify the broad requirements of your
project, the lockfile contains the exact resolved versions that are installed in the project
environment. This file should be checked into version control, allowing for consistent and
reproducible installations across machines.

`fv.lock` is a human-readable TOML file but is managed by fv and should not be edited manually.

See the [lockfile](../concepts/projects/layout.md#the-lockfile) documentation for more details.

## Managing dependencies

You can add dependencies to your `pyproject.toml` with the `fv add` command. This will also update
the lockfile and project environment:

```console
$ fv add requests
```

You can also specify version constraints or alternative sources:

```console
$ # Specify a version constraint
$ fv add 'requests==2.31.0'

$ # Add a git dependency
$ fv add git+https://github.com/psf/requests
```

If you're migrating from a `requirements.txt` file, you can use `fv add` with the `-r` flag to add
all dependencies from the file:

```console
$ # Add all dependencies from `requirements.txt`.
$ fv add -r requirements.txt -c constraints.txt
```

To remove a package, you can use `fv remove`:

```console
$ fv remove requests
```

To upgrade a package, run `fv lock` with the `--upgrade-package` flag:

```console
$ fv lock --upgrade-package requests
```

The `--upgrade-package` flag will attempt to update the specified package to the latest compatible
version, while keeping the rest of the lockfile intact.

See the documentation on [managing dependencies](../concepts/projects/dependencies.md) for more
details.

## Viewing your version

The `fv version` command can be used to read your package's version.

To get the version of your package, run `fv version`:

```console
$ fv version
hello-world 0.7.0
```

To get the version without the package name, use the `--short` option:

```console
$ fv version --short
0.7.0
```

To get version information in a JSON format, use the `--output-format json` option:

```console
$ fv version --output-format json
{
    "package_name": "hello-world",
    "version": "0.7.0",
    "commit_info": null
}
```

See the [publishing guide](./package.md#updating-your-version) for details on updating your package
version.

## Running commands

`fv run` can be used to run arbitrary scripts or commands in your project environment.

Prior to every `fv run` invocation, fv will verify that the lockfile is up-to-date with the
`pyproject.toml`, and that the environment is up-to-date with the lockfile, keeping your project
in-sync without the need for manual intervention. `fv run` guarantees that your command is run in an
environment with all required dependencies at their locked versions.

!!! note

    `fv run` does not remove extraneous packages (those not in the lockfile) from the environment
    by default. See [handling of extraneous packages](../concepts/projects/sync.md#handling-of-extraneous-packages)
    for details.

For example, to use `flask`:

```console
$ fv add flask
$ fv run -- flask run -p 3000
```

Or, to run a script:

```python title="example.py"
# Require a project dependency
import flask

print("hello world")
```

```console
$ fv run example.py
```

Alternatively, you can use `fv sync` to manually update the environment then activate it before
executing a command:

=== "macOS and Linux"

    ```console
    $ fv sync
    $ source .venv/bin/activate
    $ flask run -p 3000
    $ python example.py
    ```

=== "Windows"

    ```pwsh-session
    PS> fv sync
    PS> .venv\Scripts\activate
    PS> flask run -p 3000
    PS> python example.py
    ```

!!! note

    The virtual environment must be active to run scripts and commands in the project without `fv run`. Virtual environment activation differs per shell and platform.

See the documentation on [running commands and scripts](../concepts/projects/run.md) in projects for
more details.

## Building distributions

`fv build` can be used to build source distributions and binary distributions (wheel) for your
project.

By default, `fv build` will build the project in the current directory, and place the built
artifacts in a `dist/` subdirectory:

```console
$ fv build
$ ls dist/
hello-world-0.1.0-py3-none-any.whl
hello-world-0.1.0.tar.gz
```

See the documentation on [building projects](../concepts/projects/build.md) for more details.

## Next steps

To learn more about working on projects with fv, see the
[projects concept](../concepts/projects/index.md) page and the
[command reference](../reference/cli.md#fv).

Or, read on to learn how to
[export a fv lockfile to different formats](../concepts/projects/export.md).
