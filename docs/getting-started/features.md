# Features

fyn provides essential features for Python development — from installing Python and hacking on simple
scripts to working on large projects that support multiple Python versions and platforms.

fyn's interface can be broken down into sections, which are usable independently or together.

## Python versions

Installing and managing Python itself.

- `fyn python install`: Install Python versions.
- `fyn python list`: View available Python versions.
- `fyn python find`: Find an installed Python version.
- `fyn python pin`: Pin the current project to use a specific Python version.
- `fyn python uninstall`: Uninstall a Python version.

See the [guide on installing Python](../guides/install-python.md) to get started.

## Scripts

Executing standalone Python scripts, e.g., `example.py`.

- `fyn run`: Run a script.
- `fyn add --script`: Add a dependency to a script.
- `fyn remove --script`: Remove a dependency from a script.

See the [guide on running scripts](../guides/scripts.md) to get started.

## Projects

Creating and working on Python projects, i.e., with a `pyproject.toml`.

- `fyn init`: Create a new Python project.
- `fyn add`: Add a dependency to the project.
- `fyn remove`: Remove a dependency from the project.
- `fyn sync`: Sync the project's dependencies with the environment.
- `fyn lock`: Create a lockfile for the project's dependencies.
- `fyn run`: Run a command in the project environment.
- `fyn tree`: View the dependency tree for the project.
- `fyn build`: Build the project into distribution archives.
- `fyn publish`: Publish the project to a package index.

See the [guide on projects](../guides/projects.md) to get started.

## Tools

Running and installing tools published to Python package indexes, e.g., `ruff` or `black`.

- `fynx` / `fyn tool run`: Run a tool in a temporary environment.
- `fyn tool install`: Install a tool user-wide.
- `fyn tool uninstall`: Uninstall a tool.
- `fyn tool list`: List installed tools.
- `fyn tool update-shell`: Update the shell to include tool executables.

See the [guide on tools](../guides/tools.md) to get started.

## The pip interface

Manually managing environments and packages — intended to be used in legacy workflows or cases where
the high-level commands do not provide enough control.

Creating virtual environments (replacing `venv` and `virtualenv`):

- `fyn venv`: Create a new virtual environment.

See the documentation on [using environments](../pip/environments.md) for details.

Managing packages in an environment (replacing [`pip`](https://github.com/pypa/pip) and
[`pipdeptree`](https://github.com/tox-dev/pipdeptree)):

- `fyn pip install`: Install packages into the current environment.
- `fyn pip show`: Show details about an installed package.
- `fyn pip freeze`: List installed packages and their versions.
- `fyn pip check`: Check that the current environment has compatible packages.
- `fyn pip list`: List installed packages.
- `fyn pip uninstall`: Uninstall packages.
- `fyn pip tree`: View the dependency tree for the environment.

See the documentation on [managing packages](../pip/packages.md) for details.

Locking packages in an environment (replacing [`pip-tools`](https://github.com/jazzband/pip-tools)):

- `fyn pip compile`: Compile requirements into a lockfile.
- `fyn pip sync`: Sync an environment with a lockfile.

See the documentation on [locking environments](../pip/compile.md) for details.

!!! important

    These commands do not exactly implement the interfaces and behavior of the tools they are based on. The further you stray from common workflows, the more likely you are to encounter differences. Consult the [pip-compatibility guide](../pip/compatibility.md) for details.

## Utility

Managing and inspecting fyn's state, such as the cache, storage directories, or performing a
self-update:

- `fyn cache clean`: Remove cache entries.
- `fyn cache prune`: Remove outdated cache entries.
- `fyn cache dir`: Show the fyn cache directory path.
- `fyn tool dir`: Show the fyn tool directory path.
- `fyn python dir`: Show the fyn installed Python versions path.
- `fyn self update`: Update fyn to the latest version.

## Next steps

Read the [guides](../guides/index.md) for an introduction to each feature, check out the
[concept](../concepts/index.md) pages for in-depth details about fyn's features, or learn how to
[get help](./help.md) if you run into any problems.
