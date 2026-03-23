# Features

fv provides essential features for Python development — from installing Python and hacking on simple
scripts to working on large projects that support multiple Python versions and platforms.

fv's interface can be broken down into sections, which are usable independently or together.

## Python versions

Installing and managing Python itself.

- `fv python install`: Install Python versions.
- `fv python list`: View available Python versions.
- `fv python find`: Find an installed Python version.
- `fv python pin`: Pin the current project to use a specific Python version.
- `fv python uninstall`: Uninstall a Python version.

See the [guide on installing Python](../guides/install-python.md) to get started.

## Scripts

Executing standalone Python scripts, e.g., `example.py`.

- `fv run`: Run a script.
- `fv add --script`: Add a dependency to a script.
- `fv remove --script`: Remove a dependency from a script.

See the [guide on running scripts](../guides/scripts.md) to get started.

## Projects

Creating and working on Python projects, i.e., with a `pyproject.toml`.

- `fv init`: Create a new Python project.
- `fv add`: Add a dependency to the project.
- `fv remove`: Remove a dependency from the project.
- `fv sync`: Sync the project's dependencies with the environment.
- `fv lock`: Create a lockfile for the project's dependencies.
- `fv run`: Run a command in the project environment.
- `fv tree`: View the dependency tree for the project.
- `fv build`: Build the project into distribution archives.
- `fv publish`: Publish the project to a package index.

See the [guide on projects](../guides/projects.md) to get started.

## Tools

Running and installing tools published to Python package indexes, e.g., `ruff` or `black`.

- `fvx` / `fv tool run`: Run a tool in a temporary environment.
- `fv tool install`: Install a tool user-wide.
- `fv tool uninstall`: Uninstall a tool.
- `fv tool list`: List installed tools.
- `fv tool update-shell`: Update the shell to include tool executables.

See the [guide on tools](../guides/tools.md) to get started.

## The pip interface

Manually managing environments and packages — intended to be used in legacy workflows or cases where
the high-level commands do not provide enough control.

Creating virtual environments (replacing `venv` and `virtualenv`):

- `fv venv`: Create a new virtual environment.

See the documentation on [using environments](../pip/environments.md) for details.

Managing packages in an environment (replacing [`pip`](https://github.com/pypa/pip) and
[`pipdeptree`](https://github.com/tox-dev/pipdeptree)):

- `fv pip install`: Install packages into the current environment.
- `fv pip show`: Show details about an installed package.
- `fv pip freeze`: List installed packages and their versions.
- `fv pip check`: Check that the current environment has compatible packages.
- `fv pip list`: List installed packages.
- `fv pip uninstall`: Uninstall packages.
- `fv pip tree`: View the dependency tree for the environment.

See the documentation on [managing packages](../pip/packages.md) for details.

Locking packages in an environment (replacing [`pip-tools`](https://github.com/jazzband/pip-tools)):

- `fv pip compile`: Compile requirements into a lockfile.
- `fv pip sync`: Sync an environment with a lockfile.

See the documentation on [locking environments](../pip/compile.md) for details.

!!! important

    These commands do not exactly implement the interfaces and behavior of the tools they are based on. The further you stray from common workflows, the more likely you are to encounter differences. Consult the [pip-compatibility guide](../pip/compatibility.md) for details.

## Utility

Managing and inspecting fv's state, such as the cache, storage directories, or performing a
self-update:

- `fv cache clean`: Remove cache entries.
- `fv cache prune`: Remove outdated cache entries.
- `fv cache dir`: Show the fv cache directory path.
- `fv tool dir`: Show the fv tool directory path.
- `fv python dir`: Show the fv installed Python versions path.
- `fv self update`: Update fv to the latest version.

## Next steps

Read the [guides](../guides/index.md) for an introduction to each feature, check out the
[concept](../concepts/index.md) pages for in-depth details about fv's features, or learn how to
[get help](./help.md) if you run into any problems.
