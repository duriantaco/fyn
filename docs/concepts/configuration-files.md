# Configuration files

fv supports persistent configuration files at both the project- and user-level.

Specifically, fv will search for a `pyproject.toml` or `fv.toml` file in the current directory, or
in the nearest parent directory.

!!! note

    For `tool` commands, which operate at the user level, local configuration
    files will be ignored. Instead, fv will exclusively read from user-level configuration
    (e.g., `~/.config/fv/fv.toml`) and system-level configuration (e.g., `/etc/fv/fv.toml`).

In workspaces, fv will begin its search at the workspace root, ignoring any configuration defined in
workspace members. Since the workspace is locked as a single unit, configuration is shared across
all members.

If a `pyproject.toml` file is found, fv will read configuration from the `[tool.fv]` table. For
example, to set a persistent index URL, add the following to a `pyproject.toml`:

```toml title="pyproject.toml"
[[tool.fv.index]]
url = "https://test.pypi.org/simple"
default = true
```

(If there is no such table, the `pyproject.toml` file will be ignored, and fv will continue
searching in the directory hierarchy.)

fv will also search for `fv.toml` files, which follow an identical structure, but omit the
`[tool.fv]` prefix. For example:

```toml title="fv.toml"
[[index]]
url = "https://test.pypi.org/simple"
default = true
```

!!! note

    `fv.toml` files take precedence over `pyproject.toml` files, so if both `fv.toml` and
    `pyproject.toml` files are present in a directory, configuration will be read from `fv.toml`, and
    `[tool.fv]` section in the accompanying `pyproject.toml` will be ignored.

fv will also discover `fv.toml` configuration files in the user- and system-level
[configuration directories](../reference/storage.md#configuration-directories), e.g., user-level
configuration in `~/.config/fv/fv.toml` on macOS and Linux, or `%APPDATA%\fv\fv.toml` on Windows,
and system-level configuration at `/etc/fv/fv.toml` on macOS and Linux, or
`%PROGRAMDATA%\fv\fv.toml` on Windows.

!!! important

    User- and system-level configuration files cannot use the `pyproject.toml` format.

If project-, user-, and system-level configuration files are found, the settings will be merged,
with project-level configuration taking precedence over the user-level configuration, and user-level
configuration taking precedence over the system-level configuration. (If multiple system-level
configuration files are found, e.g., at both `/etc/fv/fv.toml` and `$XDG_CONFIG_DIRS/fv/fv.toml`,
only the first-discovered file will be used, with XDG taking priority.)

For example, if a string, number, or boolean is present in both the project- and user-level
configuration tables, the project-level value will be used, and the user-level value will be
ignored. If an array is present in both tables, the arrays will be concatenated, with the
project-level settings appearing earlier in the merged array.

Settings provided via environment variables take precedence over persistent configuration, and
settings provided via the command line take precedence over both.

fv accepts a `--no-config` command-line argument which, when provided, disables the discovery of any
persistent configuration.

fv also accepts a `--config-file` command-line argument, which accepts a path to a `fv.toml` to use
as the configuration file. When provided, this file will be used in place of _any_ discovered
configuration files (e.g., user-level configuration will be ignored).

## Settings

See the [settings reference](../reference/settings.md) for an enumeration of the available settings.

## Environment variable files

`fv run` can load environment variables from dotenv files (e.g., `.env`, `.env.local`,
`.env.development`), powered by the [`dotenvy`](https://github.com/allan2/dotenvy) crate.

To load a `.env` file from a dedicated location, set the `UV_ENV_FILE` environment variable, or pass
the `--env-file` flag to `fv run`.

For example, to load environment variables from a `.env` file in the current working directory:

```console
$ echo "MY_VAR='Hello, world!'" > .env
$ fv run --env-file .env -- python -c 'import os; print(os.getenv("MY_VAR"))'
Hello, world!
```

The `--env-file` flag can be provided multiple times, with subsequent files overriding values
defined in previous files. To provide multiple files via the `UV_ENV_FILE` environment variable,
separate the paths with a space (e.g., `UV_ENV_FILE="/path/to/file1 /path/to/file2"`).

To disable dotenv loading (e.g., to override `UV_ENV_FILE` or the `--env-file` command-line
argument), set the `UV_NO_ENV_FILE` environment variable to `1`, or pass the`--no-env-file` flag to
`fv run`.

If the same variable is defined in the environment and in a `.env` file, the value from the
environment will take precedence.

## Configuring the pip interface

A dedicated [`[tool.fv.pip]`](../reference/settings.md#pip) section is provided for configuring
_just_ the `fv pip` command line interface. Settings in this section will not apply to `fv` commands
outside the `fv pip` namespace. However, many of the settings in this section have corollaries in
the top-level namespace which _do_ apply to the `fv pip` interface unless they are overridden by a
value in the `fv.pip` section.

The `fv.pip` settings are designed to adhere closely to pip's interface and are declared separately
to retain compatibility while allowing the global settings to use alternate designs (e.g.,
`--no-build`).

As an example, setting the `index-url` under `[tool.fv.pip]`, as in the following `pyproject.toml`,
would only affect the `fv pip` subcommands (e.g., `fv pip install`, but not `fv sync`, `fv lock`, or
`fv run`):

```toml title="pyproject.toml"
[tool.fv.pip]
index-url = "https://test.pypi.org/simple"
```
