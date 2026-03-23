# Storage

## Storage directories

fyn uses the following high-level directories for storage.

For each location, fyn checks for the existence of environment variables in the given order and uses
the first path found.

The paths of storage directories are platform-specific. fyn follows the
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

    1. `$XDG_CACHE_HOME/fyn`
    1. `$HOME/.cache/fyn`

=== "Windows"

    1. `%LOCALAPPDATA%\fyn\cache`
    1. `fyn\cache` within [`FOLDERID_LocalAppData`](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalAppData)

### Persistent data directory

The persistent data directory is used for non-disposable data.

=== "Unix"

    1. `$XDG_DATA_HOME/fyn`
    1. `$HOME/.local/share/fyn`
    1. `$CWD/.fyn`

=== "Windows"

    1. `%APPDATA%\fyn\data`
    1. `.\.fyn`

### Configuration directories

The configuration directories are used to store changes to fyn's settings.

User-level configuration

=== "Unix"

    1. `$XDG_CONFIG_HOME/fyn`
    1. `$HOME/.config/fyn`

=== "Windows"

    1. `%APPDATA%\fyn`
    1. `fyn` within [`FOLDERID_RoamingAppData`](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RoamingAppData)

System-level configuration

=== "Unix"

    1. `$XDG_CONFIG_DIRS/fyn`
    1. `/etc/fyn`

=== "Windows"

    1. `%PROGRAMDATA%\fyn`
    1. `fyn` within [`FOLDERID_AppDataProgramData`](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppDataProgramData)

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

fyn uses a local cache to avoid re-downloading and re-building dependencies.

By default, the cache is stored in the [cache directory](#cache-directory) but it can be overridden
via command line arguments, environment variables, or settings as detailed in
[the cache documentation](../concepts/cache.md#cache-directory). When the cache is disabled, the
cache will be stored in a [temporary directory](#temporary-directory).

Use `fyn cache dir` to show the current cache directory path.

!!! important

    For optimal performance, the cache directory needs to be on the same filesystem as virtual
    environments.

### Python versions

fyn can install managed [Python versions](../concepts/python-versions.md), e.g., with
`fyn python install`.

By default, Python versions managed by fyn are stored in a `python/` subdirectory of the
[persistent data directory](#persistent-data-directory), e.g., `~/.local/share/fyn/python`.

Use `fyn python dir` to show the Python installation directory.

Use the `UV_PYTHON_INSTALL_DIR` environment variable to override the installation directory.

!!! note

    Changing where Python is installed will not be automatically reflected in existing virtual environments; they will keep referring to the old location, and will need to be updated manually (e.g. by re-creating them).

### Python executables

fyn installs executables for [Python versions](#python-versions), e.g., `python3.13`.

By default, Python executables are stored in the [executable directory](#executable-directory).

Use `fyn python dir --bin` to show the Python executable directory.

Use the `UV_PYTHON_BIN_DIR` environment variable to override the Python executable directory.

### Tools

fyn can install Python packages as [command-line tools](../concepts/tools.md) using
`fyn tool install`.

By default, tools are installed in a `tools/` subdirectory of the
[persistent data directory](#persistent-data-directory), e.g., `~/.local/share/fyn/tools`.

Use `fyn tool dir` to show the tool installation directory.

Use the `UV_TOOL_DIR` environment variable to configure the installation directory.

### Tool executables

fyn installs executables for installed [tools](#tools), e.g., `ruff`.

By default, tool executables are stored in the [executable directory](#executable-directory).

Use `fyn tool dir --bin` to show the tool executable directory.

Use the `UV_TOOL_BIN_DIR` environment variable to configure the tool executable directory.

### The fyn executable

When using fyn's [standalone installer](./installer.md) to install fyn, the `fyn` and `fynx`
executables are installed into the [executable directory](#executable-directory).

Use the `UV_INSTALL_DIR` environment variable to configure fyn's installation directory.

### Configuration files

fyn's behavior can be configured through TOML files.

Configuration files are discovered in the [configuration directories](#configuration-directories).

For more details, see the [configuration files documentation](../concepts/configuration-files.md).

### Project virtual environments

When working on [projects](../concepts/projects/index.md), fyn creates a dedicated virtual
environment for each project.

By default, project virtual environments are created in `.venv` in the project or workspace root,
i.e., next to the `pyproject.toml`.

Use the `UV_PROJECT_ENVIRONMENT` environment variable to override this location. For more details,
see the
[projects environment documentation](../concepts/projects/config.md#project-environment-path).

### Script virtual environments

When running [scripts with inline metadata](../guides/scripts.md), fyn creates a dedicated virtual
environment for each script in the [cache directory](#cache-directory).
