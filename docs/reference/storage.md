# Storage

## Storage directories

fv uses the following high-level directories for storage.

For each location, fv checks for the existence of environment variables in the given order and uses
the first path found.

The paths of storage directories are platform-specific. fv follows the
[XDG](https://specifications.freedesktop.org/basedir-spec/latest/) conventions on Linux and macOS
and the [Known Folder](https://learn.microsoft.com/en-us/windows/win32/shell/known-folders) scheme
on Windows.

### Temporary directory

The temporary directory is used for ephemeral data.

=== "Unix"

    1. `$TMPDIR`
    1. `/tmp`

=== "Windows"

    1. `%TMP%`
    1. `%TEMP%`
    1. `%USERPROFILE%`

### Cache directory

The cache directory is used for data that is disposable, but is useful to be long-lived.

=== "Unix"

    1. `$XDG_CACHE_HOME/fv`
    1. `$HOME/.cache/fv`

=== "Windows"

    1. `%LOCALAPPDATA%\fv\cache`
    1. `fv\cache` within [`FOLDERID_LocalAppData`](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalAppData)

### Persistent data directory

The persistent data directory is used for non-disposable data.

=== "Unix"

    1. `$XDG_DATA_HOME/fv`
    1. `$HOME/.local/share/fv`
    1. `$CWD/.fv`

=== "Windows"

    1. `%APPDATA%\fv\data`
    1. `.\.fv`

### Configuration directories

The configuration directories are used to store changes to fv's settings.

User-level configuration

=== "Unix"

    1. `$XDG_CONFIG_HOME/fv`
    1. `$HOME/.config/fv`

=== "Windows"

    1. `%APPDATA%\fv`
    1. `fv` within [`FOLDERID_RoamingAppData`](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RoamingAppData)

System-level configuration

=== "Unix"

    1. `$XDG_CONFIG_DIRS/fv`
    1. `/etc/fv`

=== "Windows"

    1. `%PROGRAMDATA%\fv`
    1. `fv` within [`FOLDERID_AppDataProgramData`](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppDataProgramData)

### Executable directory

The executable directory is used to store files that can be run by the user, i.e., a directory that
should be on the `PATH`.

=== "Unix"

    1. `$XDG_BIN_HOME`
    1. `$XDG_DATA_HOME/../bin`
    1. `$HOME/.local/bin`

=== "Windows"

    1. `%XDG_BIN_HOME%`
    1. `%XDG_DATA_HOME%\..\bin`
    1. `%USERPROFILE%\.local\bin`

## Types of data

### Dependency cache

fv uses a local cache to avoid re-downloading and re-building dependencies.

By default, the cache is stored in the [cache directory](#cache-directory) but it can be overridden
via command line arguments, environment variables, or settings as detailed in
[the cache documentation](../concepts/cache.md#cache-directory). When the cache is disabled, the
cache will be stored in a [temporary directory](#temporary-directory).

Use `fv cache dir` to show the current cache directory path.

!!! important

    For optimal performance, the cache directory needs to be on the same filesystem as virtual
    environments.

### Python versions

fv can install managed [Python versions](../concepts/python-versions.md), e.g., with
`fv python install`.

By default, Python versions managed by fv are stored in a `python/` subdirectory of the
[persistent data directory](#persistent-data-directory), e.g., `~/.local/share/fv/python`.

Use `fv python dir` to show the Python installation directory.

Use the `UV_PYTHON_INSTALL_DIR` environment variable to override the installation directory.

!!! note

    Changing where Python is installed will not be automatically reflected in existing virtual environments; they will keep referring to the old location, and will need to be updated manually (e.g. by re-creating them).

### Python executables

fv installs executables for [Python versions](#python-versions), e.g., `python3.13`.

By default, Python executables are stored in the [executable directory](#executable-directory).

Use `fv python dir --bin` to show the Python executable directory.

Use the `UV_PYTHON_BIN_DIR` environment variable to override the Python executable directory.

### Tools

fv can install Python packages as [command-line tools](../concepts/tools.md) using
`fv tool install`.

By default, tools are installed in a `tools/` subdirectory of the
[persistent data directory](#persistent-data-directory), e.g., `~/.local/share/fv/tools`.

Use `fv tool dir` to show the tool installation directory.

Use the `UV_TOOL_DIR` environment variable to configure the installation directory.

### Tool executables

fv installs executables for installed [tools](#tools), e.g., `ruff`.

By default, tool executables are stored in the [executable directory](#executable-directory).

Use `fv tool dir --bin` to show the tool executable directory.

Use the `UV_TOOL_BIN_DIR` environment variable to configure the tool executable directory.

### The fv executable

When using fv's [standalone installer](./installer.md) to install fv, the `fv` and `fvx` executables
are installed into the [executable directory](#executable-directory).

Use the `UV_INSTALL_DIR` environment variable to configure fv's installation directory.

### Configuration files

fv's behavior can be configured through TOML files.

Configuration files are discovered in the [configuration directories](#configuration-directories).

For more details, see the [configuration files documentation](../concepts/configuration-files.md).

### Project virtual environments

When working on [projects](../concepts/projects/index.md), fv creates a dedicated virtual
environment for each project.

By default, project virtual environments are created in `.venv` in the project or workspace root,
i.e., next to the `pyproject.toml`.

Use the `UV_PROJECT_ENVIRONMENT` environment variable to override this location. For more details,
see the
[projects environment documentation](../concepts/projects/config.md#project-environment-path).

### Script virtual environments

When running [scripts with inline metadata](../guides/scripts.md), fv creates a dedicated virtual
environment for each script in the [cache directory](#cache-directory).
