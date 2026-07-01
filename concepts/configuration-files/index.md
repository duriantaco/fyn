# [Configuration files](#configuration-files)

fyn supports persistent configuration files at both the project- and user-level.

Specifically, fyn will search for a `pyproject.toml` or `fyn.toml` file in the current directory, or in the nearest parent directory.

Note

For `tool` commands, which operate at the user level, local configuration files will be ignored. Instead, fyn will exclusively read from user-level configuration (e.g., `~/.config/fyn/fyn.toml`) and system-level configuration (e.g., `/etc/fyn/fyn.toml`).

In workspaces, fyn will begin its search at the workspace root, ignoring any configuration defined in workspace members. Since the workspace is locked as a single unit, configuration is shared across all members.

If a `pyproject.toml` file is found, fyn will read configuration from the `[tool.fyn]` table, falling back to `[tool.uv]` when `[tool.fyn]` is absent. For example, to set a persistent index URL, add the following to a `pyproject.toml`:

pyproject.toml

```
[[tool.fyn.index]]
url = "https://test.pypi.org/simple"
default = true
```

(If neither table is present, the `pyproject.toml` file will be ignored, and fyn will continue searching in the directory hierarchy.)

fyn will also search for `fyn.toml` files, which follow an identical structure, but omit the `[tool.fyn]` prefix. For example:

fyn.toml

```
[[index]]
url = "https://test.pypi.org/simple"
default = true
```

Note

`fyn.toml` files take precedence over `pyproject.toml` files, so if both `fyn.toml` and `pyproject.toml` files are present in a directory, configuration will be read from `fyn.toml`, and any supported tool section in the accompanying `pyproject.toml` will be ignored.

fyn will also discover `fyn.toml` configuration files in the user- and system-level [configuration directories](../../reference/storage/#configuration-directories), e.g., user-level configuration in `~/.config/fyn/fyn.toml` on macOS and Linux, or `%APPDATA%\fyn\fyn.toml` on Windows, and system-level configuration at `/etc/fyn/fyn.toml` on macOS and Linux, or `%PROGRAMDATA%\fyn\fyn.toml` on Windows.

Important

User- and system-level configuration files cannot use the `pyproject.toml` format.

If project-, user-, and system-level configuration files are found, the settings will be merged, with project-level configuration taking precedence over the user-level configuration, and user-level configuration taking precedence over the system-level configuration. (If multiple system-level configuration files are found, e.g., at both `/etc/fyn/fyn.toml` and `$XDG_CONFIG_DIRS/fyn/fyn.toml`, only the first-discovered file will be used, with XDG taking priority.)

For example, if a string, number, or boolean is present in both the project- and user-level configuration tables, the project-level value will be used, and the user-level value will be ignored. If an array is present in both tables, the arrays will be concatenated, with the project-level settings appearing earlier in the merged array.

One exception is named indexes referenced via `tool.fyn.sources`: the source pin itself must still be declared in project or workspace metadata, but a user- or system-level `fyn.toml` can replace the URL for that same index name locally. This allows teams to keep stable logical names in `pyproject.toml` while using different mirrors or private endpoints on developer machines and in CI.

Settings provided via environment variables take precedence over persistent configuration, and settings provided via the command line take precedence over both.

fyn accepts a `--no-config` command-line argument which, when provided, disables the discovery of any persistent configuration.

fyn also accepts a `--config-file` command-line argument, which accepts a path to a `fyn.toml` to use as the configuration file. When provided, this file will be used in place of *any* discovered configuration files (e.g., user-level configuration will be ignored).

## [Settings](#settings)

See the [settings reference](../../reference/settings/) for an enumeration of the available settings.

## [Environment variable files](#environment-variable-files)

`fyn run` can load environment variables from dotenv files (e.g., `.env`, `.env.local`, `.env.development`), powered by the [`dotenvy`](https://github.com/allan2/dotenvy) crate.

To load a `.env` file from a dedicated location, set the `UV_ENV_FILE` environment variable, or pass the `--env-file` flag to `fyn run`.

For example, to load environment variables from a `.env` file in the current working directory:

```
$ echo "MY_VAR='Hello, world!'" > .env
$ fyn run --env-file .env -- python -c 'import os; print(os.getenv("MY_VAR"))'
Hello, world!
```

The `--env-file` flag can be provided multiple times, with subsequent files overriding values defined in previous files. To provide multiple files via the `UV_ENV_FILE` environment variable, separate the paths with a space (e.g., `UV_ENV_FILE="/path/to/file1 /path/to/file2"`).

To disable dotenv loading (e.g., to override `UV_ENV_FILE` or the `--env-file` command-line argument), set the `UV_NO_ENV_FILE` environment variable to `1`, or pass the`--no-env-file` flag to `fyn run`.

If the same variable is defined in the environment and in a `.env` file, the value from the environment will take precedence.

## [Configuring the pip interface](#configuring-the-pip-interface)

A dedicated [`[tool.fyn.pip]`](../../reference/settings/#pip) section is provided for configuring *just* the `fyn pip` command line interface. Settings in this section will not apply to `fyn` commands outside the `fyn pip` namespace. However, many of the settings in this section have corollaries in the top-level namespace which *do* apply to the `fyn pip` interface unless they are overridden by a value in the `fyn.pip` section.

The `fyn.pip` settings are designed to adhere closely to pip's interface and are declared separately to retain compatibility while allowing the global settings to use alternate designs (e.g., `--no-build`).

As an example, setting the `index-url` under `[tool.fyn.pip]`, as in the following `pyproject.toml`, would only affect the `fyn pip` subcommands (e.g., `fyn pip install`, but not `fyn sync`, `fyn lock`, or `fyn run`):

pyproject.toml

```
[tool.fyn.pip]
index-url = "https://test.pypi.org/simple"
```
