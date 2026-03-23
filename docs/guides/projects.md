---
title: Working on projects
description:
  A guide to using fyn to create and manage Python projects, including adding dependencies, running
  commands, and building publishable distributions.
---

# Working on projects

fyn supports managing Python projects, which define their dependencies in a `pyproject.toml` file.

## Creating a new project

You can create a new Python project using the `fyn init` command:

```console
$ fyn init hello-world
$ cd hello-world
```

Alternatively, you can initialize a project in the working directory:

```console
$ mkdir hello-world
$ cd hello-world
$ fyn init
```

fyn will create the following files:

```text
├── .gitignore
├── .python-version
├── README.md
├── main.py
└── pyproject.toml
```

The `main.py` file contains a simple "Hello world" program. Try it out with `fyn run`:

```console
$ fyn run main.py
Hello from hello-world!
```

## Project structure

A project consists of a few important parts that work together and allow fyn to manage your project.
In addition to the files created by `fyn init`, fyn will create a virtual environment and `fyn.lock`
file in the root of your project the first time you run a project command, i.e., `fyn run`,
`fyn sync`, or `fyn lock`.

A complete listing would look like:

```text
.
├── .venv
│   ├── bin
│   ├── lib
│   └── fynenv.cfg
├── .python-version
├── README.md
├── main.py
├── pyproject.toml
└── fyn.lock
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
description or license. You can edit this file manually, or use commands like `fyn add` and
`fyn remove` to manage your project from the terminal.

!!! tip

    See the official [`pyproject.toml` guide](https://packaging.python.org/en/latest/guides/writing-pyproject-toml/)
    for more details on getting started with the `pyproject.toml` format.

You'll also use this file to specify fyn [configuration options](../concepts/configuration-files.md)
in a [`[tool.fyn]`](../reference/settings.md) section.

### `.python-version`

The `.python-version` file contains the project's default Python version. This file tells fyn which
Python version to use when creating the project's virtual environment.

### `.venv`

The `.venv` folder contains your project's virtual environment, a Python environment that is
isolated from the rest of your system. This is where fyn will install your project's dependencies.

See the [project environment](../concepts/projects/layout.md#the-project-environment) documentation
for more details.

### `fyn.lock`

`fyn.lock` is a cross-platform lockfile that contains exact information about your project's
dependencies. Unlike the `pyproject.toml` which is used to specify the broad requirements of your
project, the lockfile contains the exact resolved versions that are installed in the project
environment. This file should be checked into version control, allowing for consistent and
reproducible installations across machines.

`fyn.lock` is a human-readable TOML file but is managed by fyn and should not be edited manually.

See the [lockfile](../concepts/projects/layout.md#the-lockfile) documentation for more details.

## Managing dependencies

You can add dependencies to your `pyproject.toml` with the `fyn add` command. This will also update
the lockfile and project environment:

```console
$ fyn add requests
```

You can also specify version constraints or alternative sources:

```console
$ # Specify a version constraint
$ fyn add 'requests==2.31.0'

$ # Add a git dependency
$ fyn add git+https://github.com/psf/requests
```

If you're migrating from a `requirements.txt` file, you can use `fyn add` with the `-r` flag to add
all dependencies from the file:

```console
$ # Add all dependencies from `requirements.txt`.
$ fyn add -r requirements.txt -c constraints.txt
```

To remove a package, you can use `fyn remove`:

```console
$ fyn remove requests
```

To upgrade a package, run `fyn lock` with the `--upgrade-package` flag:

```console
$ fyn lock --upgrade-package requests
```

The `--upgrade-package` flag will attempt to update the specified package to the latest compatible
version, while keeping the rest of the lockfile intact.

See the documentation on [managing dependencies](../concepts/projects/dependencies.md) for more
details.

## Viewing your version

The `fyn version` command can be used to read your package's version.

To get the version of your package, run `fyn version`:

```console
$ fyn version
hello-world 0.7.0
```

To get the version without the package name, use the `--short` option:

```console
$ fyn version --short
0.7.0
```

To get version information in a JSON format, use the `--output-format json` option:

```console
$ fyn version --output-format json
{
    "package_name": "hello-world",
    "version": "0.7.0",
    "commit_info": null
}
```

See the [publishing guide](./package.md#updating-your-version) for details on updating your package
version.

## Running commands

`fyn run` can be used to run arbitrary scripts or commands in your project environment.

Prior to every `fyn run` invocation, fyn will verify that the lockfile is up-to-date with the
`pyproject.toml`, and that the environment is up-to-date with the lockfile, keeping your project
in-sync without the need for manual intervention. `fyn run` guarantees that your command is run in
an environment with all required dependencies at their locked versions.

!!! note

    `fyn run` does not remove extraneous packages (those not in the lockfile) from the environment
    by default. See [handling of extraneous packages](../concepts/projects/sync.md#handling-of-extraneous-packages)
    for details.

For example, to use `flask`:

```console
$ fyn add flask
$ fyn run -- flask run -p 3000
```

Or, to run a script:

```python title="example.py"
# Require a project dependency
import flask

print("hello world")
```

```console
$ fyn run example.py
```

Alternatively, you can use `fyn sync` to manually update the environment then activate it before
executing a command:

=== "macOS and Linux"

    ```console
    $ fyn sync
    $ source .venv/bin/activate
    $ flask run -p 3000
    $ python example.py
    ```

=== "Windows"

    ```pwsh-session
    PS> fyn sync
    PS> .venv\Scripts\activate
    PS> flask run -p 3000
    PS> python example.py
    ```

!!! note

    The virtual environment must be active to run scripts and commands in the project without `fyn run`. Virtual environment activation differs per shell and platform.

See the documentation on [running commands and scripts](../concepts/projects/run.md) in projects for
more details.

## Building distributions

`fyn build` can be used to build source distributions and binary distributions (wheel) for your
project.

By default, `fyn build` will build the project in the current directory, and place the built
artifacts in a `dist/` subdirectory:

```console
$ fyn build
$ ls dist/
hello-world-0.1.0-py3-none-any.whl
hello-world-0.1.0.tar.gz
```

See the documentation on [building projects](../concepts/projects/build.md) for more details.

## Next steps

To learn more about working on projects with fyn, see the
[projects concept](../concepts/projects/index.md) page and the
[command reference](../reference/cli.md#fyn).

Or, read on to learn how to
[export a fyn lockfile to different formats](../concepts/projects/export.md).
