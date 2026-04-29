# [CLI Reference](#cli-reference)

## [fyn](#fyn)

An extremely fast Python package and project manager.

### Usage

```
fyn [OPTIONS] <COMMAND>
```

### Commands

[`fyn auth`](#fyn-auth) : Manage authentication

[`fyn run`](#fyn-run) : Run a command or script

[`fyn init`](#fyn-init) : Create a new project

[`fyn add`](#fyn-add) : Add dependencies to the project

[`fyn remove`](#fyn-remove) : Remove dependencies from the project

[`fyn version`](#fyn-version) : Read or update the project's version

[`fyn sync`](#fyn-sync) : Update the project's environment

[`fyn lock`](#fyn-lock) : Update the project's lockfile

[`fyn upgrade`](#fyn-upgrade) : Upgrade project dependencies

[`fyn export`](#fyn-export) : Export the project's lockfile to an alternate format

[`fyn tree`](#fyn-tree) : Display the project's dependency tree

[`fyn format`](#fyn-format) : Format Python code in the project

[`fyn audit`](#fyn-audit) : Audit the project's dependencies

[`fyn tool`](#fyn-tool) : Run and install commands provided by Python packages

[`fyn python`](#fyn-python) : Manage Python versions and installations

[`fyn pip`](#fyn-pip) : Manage Python packages directly in an environment

[`fyn torch`](#fyn-torch) : Diagnose PyTorch backend selection for the current machine

[`fyn venv`](#fyn-venv) : Create a virtual environment

[`fyn shell`](#fyn-shell) : Activate the virtual environment in a new shell

[`fyn build`](#fyn-build) : Build Python packages into source distributions and wheels

[`fyn publish`](#fyn-publish) : Upload distributions to an index

[`fyn cache`](#fyn-cache) : Manage fyn's cache

[`fyn status`](#fyn-status) : Show the current project and environment status

[`fyn self`](#fyn-self) : Manage the fyn executable

[`fyn help`](#fyn-help) : Display documentation for a command

## [fyn auth](#fyn-auth)

Manage authentication

### Usage

```
fyn auth [OPTIONS] <COMMAND>
```

### Commands

[`fyn auth login`](#fyn-auth-login) : Login to a service

[`fyn auth logout`](#fyn-auth-logout) : Logout of a service

[`fyn auth token`](#fyn-auth-token) : Show the authentication token for a service

[`fyn auth dir`](#fyn-auth-dir) : Show the path to the fyn credentials directory

### [fyn auth login](#fyn-auth-login)

Login to a service

### Usage

```
fyn auth login [OPTIONS] <SERVICE>
```

### Arguments

[SERVICE](#fyn-auth-login--service) : The domain or URL of the service to log into

### Options

[`--allow-insecure-host`](#fyn-auth-login--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-auth-login--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-auth-login--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-auth-login--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-auth-login--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-auth-login--help), `-h` : Display the concise help for this command

[`--keyring-provider`](#fyn-auth-login--keyring-provider) *keyring-provider* : The keyring provider to use for storage of credentials.

```
Only `--keyring-provider native` is supported for `login`, which uses the system keyring via an integration built into fyn.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--managed-python`](#fyn-auth-login--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-auth-login--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-auth-login--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-auth-login--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-auth-login--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-auth-login--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-auth-login--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-auth-login--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--password`](#fyn-auth-login--password) *password* : The password to use for the service.

```
Use `-` to read the password from stdin.
```

[`--project`](#fyn-auth-login--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-auth-login--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--token`](#fyn-auth-login--token), `-t` *token* : The token to use for the service.

```
The username will be set to `__token__`.

Use `-` to read the token from stdin.
```

[`--username`](#fyn-auth-login--username), `-u` *username* : The username to use for the service

[`--verbose`](#fyn-auth-login--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn auth logout](#fyn-auth-logout)

Logout of a service

### Usage

```
fyn auth logout [OPTIONS] <SERVICE>
```

### Arguments

[SERVICE](#fyn-auth-logout--service) : The domain or URL of the service to logout from

### Options

[`--allow-insecure-host`](#fyn-auth-logout--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-auth-logout--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-auth-logout--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-auth-logout--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-auth-logout--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-auth-logout--help), `-h` : Display the concise help for this command

[`--keyring-provider`](#fyn-auth-logout--keyring-provider) *keyring-provider* : The keyring provider to use for storage of credentials.

```
Only `--keyring-provider native` is supported for `logout`, which uses the system keyring via an integration built into fyn.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--managed-python`](#fyn-auth-logout--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-auth-logout--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-auth-logout--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-auth-logout--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-auth-logout--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-auth-logout--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-auth-logout--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-auth-logout--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-auth-logout--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-auth-logout--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--username`](#fyn-auth-logout--username), `-u` *username* : The username to logout

[`--verbose`](#fyn-auth-logout--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn auth token](#fyn-auth-token)

Show the authentication token for a service

### Usage

```
fyn auth token [OPTIONS] <SERVICE>
```

### Arguments

[SERVICE](#fyn-auth-token--service) : The domain or URL of the service to lookup

### Options

[`--allow-insecure-host`](#fyn-auth-token--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-auth-token--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-auth-token--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-auth-token--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-auth-token--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-auth-token--help), `-h` : Display the concise help for this command

[`--keyring-provider`](#fyn-auth-token--keyring-provider) *keyring-provider* : The keyring provider to use for reading credentials

```
May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--managed-python`](#fyn-auth-token--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-auth-token--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-auth-token--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-auth-token--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-auth-token--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-auth-token--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-auth-token--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-auth-token--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-auth-token--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-auth-token--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--username`](#fyn-auth-token--username), `-u` *username* : The username to lookup

[`--verbose`](#fyn-auth-token--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn auth dir](#fyn-auth-dir)

Show the path to the fyn credentials directory.

By default, credentials are stored in the fyn data directory at `$XDG_DATA_HOME/fyn/credentials` or `$HOME/.local/share/fyn/credentials` on Unix and `%APPDATA%\fyn\data\credentials` on Windows.

The credentials directory may be overridden with `$UV_CREDENTIALS_DIR`.

Credentials are only stored in this directory when the plaintext backend is used, as opposed to the native backend, which uses the system keyring.

### Usage

```
fyn auth dir [OPTIONS] [SERVICE]
```

### Arguments

[SERVICE](#fyn-auth-dir--service) : The domain or URL of the service to lookup

### Options

[`--allow-insecure-host`](#fyn-auth-dir--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-auth-dir--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-auth-dir--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-auth-dir--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-auth-dir--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-auth-dir--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-auth-dir--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-auth-dir--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-auth-dir--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-auth-dir--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-auth-dir--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-auth-dir--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-auth-dir--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-auth-dir--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-auth-dir--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-auth-dir--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-auth-dir--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn run](#fyn-run)

Run a command or script.

Ensures that the command runs in a Python environment.

When used with a file ending in `.py` or an HTTP(S) URL, the file will be treated as a script and run with a Python interpreter, i.e., `fyn run file.py` is equivalent to `fyn run python file.py`. For URLs, the script is temporarily downloaded before execution. If the script contains inline dependency metadata, it will be installed into an isolated, ephemeral environment. When used with `-`, the input will be read from stdin, and treated as a Python script.

When used in a project, the project environment will be created and updated before invoking the command.

When used outside a project, if a virtual environment can be found in the current directory or a parent directory, the command will be run in that environment. Otherwise, the command will be run in the environment of the discovered interpreter.

By default, the project or workspace is discovered from the current working directory. However, when using `--preview-features target-workspace-discovery`, the project or workspace is instead discovered from the target script's directory.

Arguments following the command (or script) are not interpreted as arguments to fyn. All options to fyn must be provided before the command, e.g., `fyn run --verbose foo`. A `--` can be used to separate the command from fyn options for clarity, e.g., `fyn run --python 3.12 -- python`.

### Usage

```
fyn run [OPTIONS] [COMMAND]
```

### Options

[`--active`](#fyn-run--active) : Prefer the active virtual environment over the project's virtual environment.

```
If the project virtual environment is active or no virtual environment is active, this has no effect.
```

[`--all-extras`](#fyn-run--all-extras) : Include all optional dependencies.

```
This option is only available when running in a project.
```

[`--all-groups`](#fyn-run--all-groups) : Include dependencies from all dependency groups.

```
`--no-group` can be used to exclude specific groups.
```

[`--all-packages`](#fyn-run--all-packages) : Run the command with all workspace members installed.

```
The workspace's environment (`.venv`) is updated to include all workspace members.

Any extras or groups specified via `--extra`, `--group`, or related options will be applied to all workspace members.
```

[`--allow-insecure-host`](#fyn-run--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-run--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-run--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-run--compile-bytecode), `--compile` : Compile Python files to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-run--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-run--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-run--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--default-index`](#fyn-run--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-run--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--env-file`](#fyn-run--env-file) *env-file* : Load environment variables from a `.env` file.

```
Can be provided multiple times, with subsequent files overriding values defined in previous files.

May also be set with the `UV_ENV_FILE` environment variable.
```

[`--exact`](#fyn-run--exact) : Perform an exact sync, removing extraneous packages.

```
When enabled, fyn will remove any extraneous packages from the environment. By default, `fyn run` will make the minimum necessary changes to satisfy the requirements.
```

[`--exclude-newer`](#fyn-run--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-run--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra`](#fyn-run--extra) *extra* : Include optional dependencies from the specified extra name.

```
May be provided more than once.

This option is only available when running in a project.
```

[`--extra-index-url`](#fyn-run--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-run--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-run--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--frozen`](#fyn-run--frozen) : Run without updating the `fyn.lock` file [env: UV_FROZEN=]

```
Instead of checking if the lockfile is up-to-date, uses the versions in the lockfile as the source of truth. If the lockfile is missing, fyn will exit with an error. If the `pyproject.toml` includes changes to dependencies that have not been included in the lockfile yet, they will not be present in the environment.
```

[`--group`](#fyn-run--group) *group* : Include dependencies from the specified dependency group.

```
May be provided multiple times.
```

[`--gui-script`](#fyn-run--gui-script) : Run the given path as a Python GUI script.

```
Using `--gui-script` will attempt to parse the path as a PEP 723 script and run it with `pythonw.exe`, irrespective of its extension. Only available on Windows.
```

[`--help`](#fyn-run--help), `-h` : Display the concise help for this command

[`--index`](#fyn-run--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-run--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-run--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--isolated`](#fyn-run--isolated) : Run the command in an isolated virtual environment [env: UV_ISOLATED=]

```
Usually, the project environment is reused for performance. This option forces a fresh environment to be used for the project, enforcing strict isolation between dependencies and declaration of requirements.

An editable installation is still used for the project.

When used with `--with` or `--with-requirements`, the additional dependencies will still be layered in a second environment.
```

[`--keyring-provider`](#fyn-run--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-run--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--list-tasks`](#fyn-run--list-tasks) : List available tasks defined in `[tool.fyn.tasks]` and exit

[`--locked`](#fyn-run--locked) : Assert that the `fyn.lock` will remain unchanged [env: UV_LOCKED=]

```
Requires that the lockfile is up-to-date. If the lockfile is missing or needs to be updated, fyn will exit with an error.
```

[`--managed-python`](#fyn-run--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--module`](#fyn-run--module), `-m` : Run a Python module.

```
Equivalent to `python -m <module>`.
```

[`--native-tls`](#fyn-run--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-run--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-run--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-run--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-run--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-run--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-run--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-run--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-run--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-default-groups`](#fyn-run--no-default-groups) : Ignore the default dependency groups.

```
fyn includes the groups defined in `tool.fyn.default-groups` by default. This disables that option, however, specific groups can still be included with `--group`.

May also be set with the `UV_NO_DEFAULT_GROUPS` environment variable.
```

[`--no-dev`](#fyn-run--no-dev) : Disable the development dependency group [env: UV_NO_DEV=]

```
This option is an alias of `--no-group dev`. See `--no-default-groups` to disable all default groups instead.

This option is only available when running in a project.
```

[`--no-editable`](#fyn-run--no-editable) : Install any editable dependencies, including the project and any workspace members, as non-editable [env: UV_NO_EDITABLE=]

[`--no-env-file`](#fyn-run--no-env-file) : Avoid reading environment variables from a `.env` file [env: UV_NO_ENV_FILE=]

[`--no-extra`](#fyn-run--no-extra) *no-extra* : Exclude the specified optional dependencies, if `--all-extras` is supplied.

```
May be provided multiple times.
```

[`--no-group`](#fyn-run--no-group) *no-group* : Disable the specified dependency group.

```
This option always takes precedence over default groups, `--all-groups`, and `--group`.

May be provided multiple times.

May also be set with the `UV_NO_GROUP` environment variable.
```

[`--no-index`](#fyn-run--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-run--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-run--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-project`](#fyn-run--no-project), `--no_workspace` : Avoid discovering the project or workspace.

```
Instead of searching for projects in the current directory and parent directories, run in an isolated, ephemeral environment populated by the `--with` requirements.

If a virtual environment is active or found in a current or parent directory, it will be used as if there was no project or workspace.
```

[`--no-python-downloads`](#fyn-run--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-run--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-run--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--no-sync`](#fyn-run--no-sync) : Avoid syncing the virtual environment [env: UV_NO_SYNC=]

```
Implies `--frozen`, as the project dependencies will be ignored (i.e., the lockfile will not be updated, since the environment will not be synced regardless).
```

[`--offline`](#fyn-run--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-dev`](#fyn-run--only-dev) : Only include the development dependency group.

```
The project and its dependencies will be omitted.

This option is an alias for `--only-group dev`. Implies `--no-default-groups`.
```

[`--only-group`](#fyn-run--only-group) *only-group* : Only include dependencies from the specified dependency group.

```
The project and its dependencies will be omitted.

May be provided multiple times. Implies `--no-default-groups`.
```

[`--package`](#fyn-run--package) *package* : Run the command in a specific package in the workspace.

```
If the workspace member does not exist, fyn will exit with an error.
```

[`--prerelease`](#fyn-run--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-run--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-run--python), `-p` *python* : The Python interpreter to use for the run environment.

```
If the interpreter request is satisfied by a discovered environment, the environment will be
used.

See [fyn python](#fyn-python) to view supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--python-platform`](#fyn-run--python-platform) *python-platform* : The platform for which requirements should be installed.

```
Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

When targeting macOS (Darwin), the default minimum version is `13.0`. Use `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting iOS, the default minimum version is `13.0`. Use `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting Android, the default minimum Android API level is `24`. Use `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.

WARNING: When specified, fyn will select wheels that are compatible with the *target* platform; as a result, the installed distributions may not be compatible with the *current* platform. Conversely, any distributions that are built from source may be incompatible with the *target* platform, as they will be built for the *current* platform. The `--python-platform` option is intended for advanced use cases.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--quiet`](#fyn-run--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-run--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-run--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--reinstall`](#fyn-run--reinstall), `--force-reinstall` : Reinstall all packages, regardless of whether they're already installed. Implies `--refresh`

[`--reinstall-package`](#fyn-run--reinstall-package) *reinstall-package* : Reinstall a specific package, regardless of whether it's already installed. Implies `--refresh-package`

[`--resolution`](#fyn-run--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--script`](#fyn-run--script), `-s` : Run the given path as a Python script.

```
Using `--script` will attempt to parse the path as a PEP 723 script, irrespective of its extension.
```

[`--upgrade`](#fyn-run--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-run--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-run--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

[`--with`](#fyn-run--with), `-w` *with* : Run with the given packages installed.

```
When used in a project, these dependencies will be layered on top of the project environment in a separate, ephemeral environment. These dependencies are allowed to conflict with those specified by the project.
```

[`--with-editable`](#fyn-run--with-editable) *with-editable* : Run with the given packages installed in editable mode.

```
When used in a project, these dependencies will be layered on top of the project environment in a separate, ephemeral environment. These dependencies are allowed to conflict with those specified by the project.
```

[`--with-requirements`](#fyn-run--with-requirements) *with-requirements* : Run with the packages listed in the given files.

```
The following formats are supported: `requirements.txt`, `.py` files with inline metadata, and `pylock.toml`.

The same environment semantics as `--with` apply.

Using `pyproject.toml`, `setup.py`, or `setup.cfg` files is not allowed.
```

## [fyn init](#fyn-init)

Create a new project.

Follows the `pyproject.toml` specification.

If a `pyproject.toml` already exists at the target, fyn will exit with an error.

If a `pyproject.toml` is found in any of the parent directories of the target path, the project will be added as a workspace member of the parent.

Some project state is not created until needed, e.g., the project virtual environment (`.venv`) and lockfile (`fyn.lock`) are lazily created during the first sync.

### Usage

```
fyn init [OPTIONS] [PATH]
```

### Arguments

[PATH](#fyn-init--path) : The path to use for the project/script.

```
Defaults to the current working directory when initializing an app or library; required when initializing a script. Accepts relative and absolute paths.

If a `pyproject.toml` is found in any of the parent directories of the target path, the project will be added as a workspace member of the parent, unless `--no-workspace` is provided.
```

### Options

[`--allow-insecure-host`](#fyn-init--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--app`](#fyn-init--app), `--application` : Create a project for an application.

```
This is the default behavior if `--lib` is not requested.

This project kind is for web servers, scripts, and command-line interfaces.

By default, an application is not intended to be built and distributed as a Python package. The `--package` option can be used to create an application that is distributable, e.g., if you want to distribute a command-line interface via PyPI.
```

[`--author-from`](#fyn-init--author-from) *author-from* : Fill in the `authors` field in the `pyproject.toml`.

```
By default, fyn will attempt to infer the author information from some sources (e.g., Git) (`auto`). Use `--author-from git` to only infer from Git configuration. Use `--author-from none` to avoid inferring the author information.

Possible values:

- `auto`: Fetch the author information from some sources (e.g., Git) automatically
- `git`: Fetch the author information from Git configuration only
- `none`: Do not infer the author information
```

[`--bare`](#fyn-init--bare) : Only create a `pyproject.toml`.

```
Disables creating extra files like `README.md`, the `src/` tree, `.python-version` files, etc.

When combined with `--script`, the script will only contain the inline metadata header.
```

[`--build-backend`](#fyn-init--build-backend) *build-backend* : Initialize a build-backend of choice for the project.

```
Implicitly sets `--package`.

May also be set with the `UV_INIT_BUILD_BACKEND` environment variable.

Possible values:

- `uv`: Use uv as the project build backend
- `hatch`: Use [hatchling](https://pypi.org/project/hatchling) as the project build backend
- `flit`: Use [flit-core](https://pypi.org/project/flit-core) as the project build backend
- `pdm`: Use [pdm-backend](https://pypi.org/project/pdm-backend) as the project build backend
- `poetry`: Use [poetry-core](https://pypi.org/project/poetry-core) as the project build backend
- `setuptools`: Use [setuptools](https://pypi.org/project/setuptools) as the project build backend
- `maturin`: Use [maturin](https://pypi.org/project/maturin) as the project build backend
- `scikit`: Use [scikit-build-core](https://pypi.org/project/scikit-build-core) as the project build backend
```

[`--cache-dir`](#fyn-init--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-init--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-init--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--description`](#fyn-init--description) *description* : Set the project description

[`--directory`](#fyn-init--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-init--help), `-h` : Display the concise help for this command

[`--lib`](#fyn-init--lib), `--library` : Create a project for a library.

```
A library is a project that is intended to be built and distributed as a Python package.
```

[`--managed-python`](#fyn-init--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--name`](#fyn-init--name) *name* : The name of the project.

```
Defaults to the name of the directory.
```

[`--native-tls`](#fyn-init--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-init--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-init--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-description`](#fyn-init--no-description) : Disable the description for the project

[`--no-managed-python`](#fyn-init--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-package`](#fyn-init--no-package) : Do not set up the project to be built as a Python package.

```
Does not include a `[build-system]` for the project.

This is the default behavior when using `--app`.
```

[`--no-pin-python`](#fyn-init--no-pin-python) : Do not create a `.python-version` file for the project.

```
By default, fyn will create a `.python-version` file containing the minor version of the discovered Python interpreter, which will cause subsequent fyn commands to use that version.
```

[`--no-progress`](#fyn-init--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-init--no-python-downloads) : Disable automatic downloads of Python.

[`--no-readme`](#fyn-init--no-readme) : Do not create a `README.md` file

[`--no-workspace`](#fyn-init--no-workspace), `--no-project` : Avoid discovering a workspace and create a standalone project.

```
By default, fyn searches for workspaces in the current directory or any parent directory.
```

[`--offline`](#fyn-init--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--package`](#fyn-init--package) : Set up the project to be built as a Python package.

```
Defines a `[build-system]` for the project.

This is the default behavior when using `--lib` or `--build-backend`.

When using `--app`, this will include a `[project.scripts]` entrypoint and use a `src/` project structure.
```

[`--project`](#fyn-init--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-init--python), `-p` *python* : The Python interpreter to use to determine the minimum supported Python version.

```
See [fyn python](#fyn-python) to view supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-init--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--script`](#fyn-init--script) : Create a script.

```
A script is a standalone file with embedded metadata enumerating its dependencies, along with any Python version requirements, as defined in the PEP 723 specification.

PEP 723 scripts can be executed directly with `fyn run`.

By default, adds a requirement on the system Python version; use `--python` to specify an alternative Python version requirement.
```

[`--vcs`](#fyn-init--vcs) *vcs* : Initialize a version control system for the project.

```
By default, fyn will initialize a Git repository (`git`). Use `--vcs none` to explicitly avoid initializing a version control system.

Possible values:

- `git`: Use Git for version control
- `none`: Do not use any version control system
```

[`--verbose`](#fyn-init--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn add](#fyn-add)

Add dependencies to the project.

Dependencies are added to the project's `pyproject.toml` file.

If a given dependency exists already, it will be updated to the new version specifier unless it includes markers that differ from the existing specifier in which case another entry for the dependency will be added.

The lockfile and project environment will be updated to reflect the added dependencies. To skip updating the lockfile, use `--frozen`. To skip updating the environment, use `--no-sync`.

If any of the requested dependencies cannot be found, fyn will exit with an error, unless the `--frozen` flag is provided, in which case fyn will add the dependencies verbatim without checking that they exist or are compatible with the project.

fyn will search for a project in the current directory or any parent directory. If a project cannot be found, fyn will exit with an error.

### Usage

```
fyn add [OPTIONS] <PACKAGES|--requirements <REQUIREMENTS>>
```

### Arguments

[PACKAGES](#fyn-add--packages) : The packages to add, as PEP 508 requirements (e.g., `ruff==0.5.0`)

### Options

[`--active`](#fyn-add--active) : Prefer the active virtual environment over the project's virtual environment.

```
If the project virtual environment is active or no virtual environment is active, this has no effect.
```

[`--allow-insecure-host`](#fyn-add--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--bounds`](#fyn-add--bounds) *bounds* : The kind of version specifier to use when adding dependencies.

```
When adding a dependency to the project, if no constraint or URL is provided, a constraint is added based on the latest compatible version of the package. By default, a lower bound constraint is used, e.g., `>=1.2.3`.

When `--frozen` is provided, no resolution is performed, and dependencies are always added without constraints.

This option is in preview and may change in any future release.

Possible values:

- `lower`: Only a lower bound, e.g., `>=1.2.3`
- `major`: Allow the same major version, similar to the semver caret, e.g., `>=1.2.3, <2.0.0`
- `minor`: Allow the same minor version, similar to the semver tilde, e.g., `>=1.2.3, <1.3.0`
- `exact`: Pin the exact version, e.g., `==1.2.3`
```

[`--branch`](#fyn-add--branch) *branch* : Branch to use when adding a dependency from Git

[`--cache-dir`](#fyn-add--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-add--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-add--compile-bytecode), `--compile` : Compile Python files to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-add--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-add--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-add--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--constraints`](#fyn-add--constraints), `--constraint`, `-c` *constraints* : Constrain versions using the given requirements files.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. The constraints will *not* be added to the project's `pyproject.toml` file, but *will* be respected during dependency resolution.

This is equivalent to pip's `--constraint` option.

May also be set with the `UV_CONSTRAINT` environment variable.
```

[`--default-index`](#fyn-add--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--dev`](#fyn-add--dev) : Add the requirements to the development dependency group [env: UV_DEV=]

```
This option is an alias for `--group dev`.
```

[`--directory`](#fyn-add--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--editable`](#fyn-add--editable) : Add the requirements as editable

[`--exclude-newer`](#fyn-add--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-add--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra`](#fyn-add--extra) *extra* : Extras to enable for the dependency.

```
May be provided more than once.

To add this dependency to an optional extra instead, see `--optional`.
```

[`--extra-index-url`](#fyn-add--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-add--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-add--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--frozen`](#fyn-add--frozen) : Add dependencies without re-locking the project [env: UV_FROZEN=]

```
The project environment will not be synced.
```

[`--group`](#fyn-add--group) *group* : Add the requirements to the specified dependency group.

```
These requirements will not be included in the published metadata for the project.
```

[`--help`](#fyn-add--help), `-h` : Display the concise help for this command

[`--index`](#fyn-add--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-add--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-add--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-add--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--lfs`](#fyn-add--lfs) : Whether to use Git LFS when adding a dependency from Git

[`--link-mode`](#fyn-add--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--locked`](#fyn-add--locked) : Assert that the `fyn.lock` will remain unchanged [env: UV_LOCKED=]

```
Requires that the lockfile is up-to-date. If the lockfile is missing or needs to be updated, fyn will exit with an error.
```

[`--managed-python`](#fyn-add--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--marker`](#fyn-add--marker), `-m` *marker* : Apply this marker to all added packages

[`--native-tls`](#fyn-add--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-add--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-add--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-add--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-add--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-add--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-add--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-add--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-add--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-index`](#fyn-add--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-install-local`](#fyn-add--no-install-local) : Do not install local path dependencies

```
Skips the current project, workspace members, and any other local (path or editable) packages. Only remote/indexed dependencies are installed. Useful in Docker builds to cache heavy third-party dependencies first and layer local packages separately.

The inverse `--only-install-local` can be used to install *only* local packages, excluding all remote dependencies.
```

[`--no-install-package`](#fyn-add--no-install-package) *no-install-package* : Do not install the given package(s).

```
By default, all project's dependencies are installed into the environment. The `--no-install-package` option allows exclusion of specific packages. Note this can result in a broken environment, and should be used with caution.

The inverse `--only-install-package` can be used to install *only* the specified packages, excluding all others.
```

[`--no-install-project`](#fyn-add--no-install-project) : Do not install the current project.

```
By default, the current project is installed into the environment with all of its dependencies. The `--no-install-project` option allows the project to be excluded, but all of its dependencies are still installed. This is particularly useful in situations like building Docker images where installing the project separately from its dependencies allows optimal layer caching.

The inverse `--only-install-project` can be used to install *only* the project itself, excluding all dependencies.
```

[`--no-install-workspace`](#fyn-add--no-install-workspace) : Do not install any workspace members, including the current project.

```
By default, all workspace members and their dependencies are installed into the environment. The `--no-install-workspace` option allows exclusion of all the workspace members while retaining their dependencies. This is particularly useful in situations like building Docker images where installing the workspace separately from its dependencies allows optimal layer caching.

The inverse `--only-install-workspace` can be used to install *only* workspace members, excluding all other dependencies.
```

[`--no-managed-python`](#fyn-add--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-add--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-add--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-add--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-add--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--no-sync`](#fyn-add--no-sync) : Avoid syncing the virtual environment [env: UV_NO_SYNC=]

[`--no-workspace`](#fyn-add--no-workspace) : Don't add the dependency as a workspace member.

```
By default, when adding a dependency that's a local path and is within the workspace directory, fyn will add it as a workspace member; pass `--no-workspace` to add the package as direct path dependency instead.
```

[`--offline`](#fyn-add--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--optional`](#fyn-add--optional) *optional* : Add the requirements to the package's optional dependencies for the specified extra.

```
The group may then be activated when installing the project with the `--extra` flag.

To enable an optional extra for this requirement instead, see `--extra`.
```

[`--package`](#fyn-add--package) *package* : Add the dependency to a specific package in the workspace

[`--prerelease`](#fyn-add--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-add--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-add--python), `-p` *python* : The Python interpreter to use for resolving and syncing.

```
See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-add--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--raw`](#fyn-add--raw), `--raw-sources` : Add a dependency as provided.

```
By default, fyn will use the `tool.fyn.sources` section to record source information for Git, local, editable, and direct URL requirements. When `--raw` is provided, fyn will add source requirements to `project.dependencies`, rather than `tool.fyn.sources`.

Additionally, by default, fyn will add bounds to your dependency, e.g., `foo>=1.0.0`. When `--raw` is provided, fyn will add the dependency without bounds.
```

[`--refresh`](#fyn-add--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-add--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--reinstall`](#fyn-add--reinstall), `--force-reinstall` : Reinstall all packages, regardless of whether they're already installed. Implies `--refresh`

[`--reinstall-package`](#fyn-add--reinstall-package) *reinstall-package* : Reinstall a specific package, regardless of whether it's already installed. Implies `--refresh-package`

[`--requirements`](#fyn-add--requirements), `--requirement`, `-r` *requirements* : Add the packages listed in the given files.

```
The following formats are supported: `requirements.txt`, `.py` files with inline metadata, `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg`.
```

[`--resolution`](#fyn-add--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--rev`](#fyn-add--rev) *rev* : Commit to use when adding a dependency from Git

[`--script`](#fyn-add--script) *script* : Add the dependency to the specified Python script, rather than to a project.

```
If provided, fyn will add the dependency to the script's inline metadata table, in adherence with PEP 723. If no such inline metadata table is present, a new one will be created and added to the script. When executed via `fyn run`, fyn will create a temporary environment for the script with all inline dependencies installed.
```

[`--tag`](#fyn-add--tag) *tag* : Tag to use when adding a dependency from Git

[`--upgrade`](#fyn-add--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-add--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-add--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

[`--workspace`](#fyn-add--workspace) : Add the dependency as a workspace member.

```
By default, fyn will add path dependencies that are within the workspace directory as workspace members. When used with a path dependency, the package will be added to the workspace's `members` list in the root `pyproject.toml` file.
```

## [fyn remove](#fyn-remove)

Remove dependencies from the project.

Dependencies are removed from the project's `pyproject.toml` file.

If multiple entries exist for a given dependency, i.e., each with different markers, all of the entries will be removed.

The lockfile and project environment will be updated to reflect the removed dependencies. To skip updating the lockfile, use `--frozen`. To skip updating the environment, use `--no-sync`.

If any of the requested dependencies are not present in the project, fyn will exit with an error.

If a package has been manually installed in the environment, i.e., with `fyn pip install`, it will not be removed by `fyn remove`.

fyn will search for a project in the current directory or any parent directory. If a project cannot be found, fyn will exit with an error.

### Usage

```
fyn remove [OPTIONS] <PACKAGES>...
```

### Arguments

[PACKAGES](#fyn-remove--packages) : The names of the dependencies to remove (e.g., `ruff`)

### Options

[`--active`](#fyn-remove--active) : Prefer the active virtual environment over the project's virtual environment.

```
If the project virtual environment is active or no virtual environment is active, this has no effect.
```

[`--allow-insecure-host`](#fyn-remove--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-remove--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-remove--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-remove--compile-bytecode), `--compile` : Compile Python files to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-remove--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-remove--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-remove--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--default-index`](#fyn-remove--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--dev`](#fyn-remove--dev) : Remove the packages from the development dependency group [env: UV_DEV=]

```
This option is an alias for `--group dev`.
```

[`--directory`](#fyn-remove--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-remove--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-remove--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra-index-url`](#fyn-remove--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-remove--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-remove--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--frozen`](#fyn-remove--frozen) : Remove dependencies without re-locking the project [env: UV_FROZEN=]

```
The project environment will not be synced.
```

[`--group`](#fyn-remove--group) *group* : Remove the packages from the specified dependency group

[`--help`](#fyn-remove--help), `-h` : Display the concise help for this command

[`--index`](#fyn-remove--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-remove--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-remove--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-remove--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-remove--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--locked`](#fyn-remove--locked) : Assert that the `fyn.lock` will remain unchanged [env: UV_LOCKED=]

```
Requires that the lockfile is up-to-date. If the lockfile is missing or needs to be updated, fyn will exit with an error.
```

[`--managed-python`](#fyn-remove--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-remove--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-remove--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-remove--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-remove--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-remove--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-remove--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-remove--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-remove--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-remove--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-index`](#fyn-remove--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-remove--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-remove--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-remove--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-remove--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-remove--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--no-sync`](#fyn-remove--no-sync) : Avoid syncing the virtual environment after re-locking the project [env: UV_NO_SYNC=]

[`--offline`](#fyn-remove--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--optional`](#fyn-remove--optional) *optional* : Remove the packages from the project's optional dependencies for the specified extra

[`--package`](#fyn-remove--package) *package* : Remove the dependencies from a specific package in the workspace

[`--prerelease`](#fyn-remove--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-remove--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-remove--python), `-p` *python* : The Python interpreter to use for resolving and syncing.

```
See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-remove--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-remove--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-remove--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--reinstall`](#fyn-remove--reinstall), `--force-reinstall` : Reinstall all packages, regardless of whether they're already installed. Implies `--refresh`

[`--reinstall-package`](#fyn-remove--reinstall-package) *reinstall-package* : Reinstall a specific package, regardless of whether it's already installed. Implies `--refresh-package`

[`--resolution`](#fyn-remove--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--script`](#fyn-remove--script) *script* : Remove the dependency from the specified Python script, rather than from a project.

```
If provided, fyn will remove the dependency from the script's inline metadata table, in adherence with PEP 723.
```

[`--upgrade`](#fyn-remove--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-remove--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-remove--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn version](#fyn-version)

Read or update the project's version

### Usage

```
fyn version [OPTIONS] [VALUE]
```

### Arguments

[VALUE](#fyn-version--value) : Set the project version to this value

```
To update the project using semantic versioning components instead, use `--bump`.
```

### Options

[`--active`](#fyn-version--active) : Prefer the active virtual environment over the project's virtual environment.

```
If the project virtual environment is active or no virtual environment is active, this has no effect.
```

[`--allow-insecure-host`](#fyn-version--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--bump`](#fyn-version--bump) *bump[=value]* : Update the project version using the given semantics

```
This flag can be passed multiple times.

Possible values:

- `major`: Increase the major version (e.g., 1.2.3 => 2.0.0)
- `minor`: Increase the minor version (e.g., 1.2.3 => 1.3.0)
- `patch`: Increase the patch version (e.g., 1.2.3 => 1.2.4)
- `stable`: Move from a pre-release to stable version (e.g., 1.2.3b4.post5.dev6 => 1.2.3)
- `alpha`: Increase the alpha version (e.g., 1.2.3a4 => 1.2.3a5)
- `beta`: Increase the beta version (e.g., 1.2.3b4 => 1.2.3b5)
- `rc`: Increase the rc version (e.g., 1.2.3rc4 => 1.2.3rc5)
- `post`: Increase the post version (e.g., 1.2.3.post5 => 1.2.3.post6)
- `dev`: Increase the dev version (e.g., 1.2.3a4.dev6 => 1.2.3.dev7)
```

[`--cache-dir`](#fyn-version--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-version--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-version--compile-bytecode), `--compile` : Compile Python files to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-version--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-version--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-version--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--default-index`](#fyn-version--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-version--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--dry-run`](#fyn-version--dry-run) : Don't write a new version to the `pyproject.toml`

```
Instead, the version will be displayed.
```

[`--exclude-newer`](#fyn-version--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-version--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra-index-url`](#fyn-version--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-version--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-version--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--frozen`](#fyn-version--frozen) : Update the version without re-locking the project [env: UV_FROZEN=]

```
The project environment will not be synced.
```

[`--help`](#fyn-version--help), `-h` : Display the concise help for this command

[`--index`](#fyn-version--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-version--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-version--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-version--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-version--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--locked`](#fyn-version--locked) : Assert that the `fyn.lock` will remain unchanged [env: UV_LOCKED=]

```
Requires that the lockfile is up-to-date. If the lockfile is missing or needs to be updated, fyn will exit with an error.
```

[`--managed-python`](#fyn-version--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-version--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-version--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-version--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-version--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-version--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-version--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-version--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-version--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-version--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-index`](#fyn-version--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-version--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-version--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-version--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-version--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-version--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--no-sync`](#fyn-version--no-sync) : Avoid syncing the virtual environment after re-locking the project [env: UV_NO_SYNC=]

[`--offline`](#fyn-version--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--output-format`](#fyn-version--output-format) *output-format* : The format of the output

```
[default: text]

Possible values:

- `text`: Display the version as plain text
- `json`: Display the version as JSON
```

[`--package`](#fyn-version--package) *package* : Update the version of a specific package in the workspace

[`--prerelease`](#fyn-version--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-version--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-version--python), `-p` *python* : The Python interpreter to use for resolving and syncing.

```
See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-version--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-version--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-version--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--reinstall`](#fyn-version--reinstall), `--force-reinstall` : Reinstall all packages, regardless of whether they're already installed. Implies `--refresh`

[`--reinstall-package`](#fyn-version--reinstall-package) *reinstall-package* : Reinstall a specific package, regardless of whether it's already installed. Implies `--refresh-package`

[`--resolution`](#fyn-version--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--short`](#fyn-version--short) : Only show the version

```
By default, fyn will show the project name before the version.
```

[`--upgrade`](#fyn-version--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-version--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-version--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn sync](#fyn-sync)

Update the project's environment.

Syncing ensures that all project dependencies are installed and up-to-date with the lockfile.

By default, an exact sync is performed: fyn removes packages that are not declared as dependencies of the project. Use the `--inexact` flag to keep extraneous packages. Note that if an extraneous package conflicts with a project dependency, it will still be removed. Additionally, if `--no-build-isolation` is used, fyn will not remove extraneous packages to avoid removing possible build dependencies.

If the project virtual environment (`.venv`) does not exist, it will be created.

The project is re-locked before syncing unless the `--locked` or `--frozen` flag is provided.

fyn will search for a project in the current directory or any parent directory. If a project cannot be found, fyn will exit with an error.

Note that, when installing from a lockfile, fyn will not provide warnings for yanked package versions.

### Usage

```
fyn sync [OPTIONS]
```

### Options

[`--active`](#fyn-sync--active) : Sync dependencies to the active virtual environment.

```
Instead of creating or updating the virtual environment for the project or script, the active virtual environment will be preferred, if the `VIRTUAL_ENV` environment variable is set.
```

[`--all-extras`](#fyn-sync--all-extras) : Include all optional dependencies.

```
When two or more extras are declared as conflicting in `tool.fyn.conflicts`, using this flag will always result in an error.

Note that all optional dependencies are always included in the resolution; this option only affects the selection of packages to install.
```

[`--all-groups`](#fyn-sync--all-groups) : Include dependencies from all dependency groups.

```
`--no-group` can be used to exclude specific groups.
```

[`--all-packages`](#fyn-sync--all-packages) : Sync all packages in the workspace.

```
The workspace's environment (`.venv`) is updated to include all workspace members.

Any extras or groups specified via `--extra`, `--group`, or related options will be applied to all workspace members.
```

[`--allow-insecure-host`](#fyn-sync--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-sync--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--check`](#fyn-sync--check) : Check if the Python environment is synchronized with the project.

```
If the environment is not up to date, fyn will exit with an error.
```

[`--color`](#fyn-sync--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-sync--compile-bytecode), `--compile` : Compile Python files to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-sync--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-sync--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-sync--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--default-index`](#fyn-sync--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-sync--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--dry-run`](#fyn-sync--dry-run) : Perform a dry run, without writing the lockfile or modifying the project environment.

```
In dry-run mode, fyn will resolve the project's dependencies and report on the resulting changes to both the lockfile and the project environment, but will not modify either.
```

[`--exclude-newer`](#fyn-sync--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-sync--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra`](#fyn-sync--extra) *extra* : Include optional dependencies from the specified extra name.

```
May be provided more than once.

When multiple extras or groups are specified that appear in `tool.fyn.conflicts`, fyn will report an error.

Note that all optional dependencies are always included in the resolution; this option only affects the selection of packages to install.
```

[`--extra-index-url`](#fyn-sync--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-sync--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-sync--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--frozen`](#fyn-sync--frozen) : Sync without updating the `fyn.lock` file [env: UV_FROZEN=]

```
Instead of checking if the lockfile is up-to-date, uses the versions in the lockfile as the source of truth. If the lockfile is missing, fyn will exit with an error. If the `pyproject.toml` includes changes to dependencies that have not been included in the lockfile yet, they will not be present in the environment.
```

[`--group`](#fyn-sync--group) *group* : Include dependencies from the specified dependency group.

```
When multiple extras or groups are specified that appear in `tool.fyn.conflicts`, fyn will report an error.

May be provided multiple times.
```

[`--help`](#fyn-sync--help), `-h` : Display the concise help for this command

[`--index`](#fyn-sync--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-sync--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-sync--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--inexact`](#fyn-sync--inexact), `--no-exact` : Do not remove extraneous packages present in the environment.

```
When enabled, fyn will make the minimum necessary changes to satisfy the requirements. By default, syncing will remove any extraneous packages from the environment
```

[`--keyring-provider`](#fyn-sync--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-sync--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--locked`](#fyn-sync--locked) : Assert that the `fyn.lock` will remain unchanged [env: UV_LOCKED=]

```
Requires that the lockfile is up-to-date. If the lockfile is missing or needs to be updated, fyn will exit with an error.
```

[`--managed-python`](#fyn-sync--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-sync--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-sync--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-sync--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-sync--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-sync--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-sync--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-sync--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-sync--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-sync--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-default-groups`](#fyn-sync--no-default-groups) : Ignore the default dependency groups.

```
fyn includes the groups defined in `tool.fyn.default-groups` by default. This disables that option, however, specific groups can still be included with `--group`.

May also be set with the `UV_NO_DEFAULT_GROUPS` environment variable.
```

[`--no-dev`](#fyn-sync--no-dev) : Disable the development dependency group [env: UV_NO_DEV=]

```
This option is an alias of `--no-group dev`. See `--no-default-groups` to disable all default groups instead.
```

[`--no-editable`](#fyn-sync--no-editable) : Install any editable dependencies, including the project and any workspace members, as non-editable [env: UV_NO_EDITABLE=]

[`--no-extra`](#fyn-sync--no-extra) *no-extra* : Exclude the specified optional dependencies, if `--all-extras` is supplied.

```
May be provided multiple times.
```

[`--no-group`](#fyn-sync--no-group) *no-group* : Disable the specified dependency group.

```
This option always takes precedence over default groups, `--all-groups`, and `--group`.

May be provided multiple times.

May also be set with the `UV_NO_GROUP` environment variable.
```

[`--no-index`](#fyn-sync--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-install-local`](#fyn-sync--no-install-local) : Do not install local path dependencies

```
Skips the current project, workspace members, and any other local (path or editable) packages. Only remote/indexed dependencies are installed. Useful in Docker builds to cache heavy third-party dependencies first and layer local packages separately.

The inverse `--only-install-local` can be used to install *only* local packages, excluding all remote dependencies.
```

[`--no-install-package`](#fyn-sync--no-install-package) *no-install-package* : Do not install the given package(s).

```
By default, all of the project's dependencies are installed into the environment. The `--no-install-package` option allows exclusion of specific packages. Note this can result in a broken environment, and should be used with caution.

The inverse `--only-install-package` can be used to install *only* the specified packages, excluding all others.
```

[`--no-install-project`](#fyn-sync--no-install-project) : Do not install the current project.

```
By default, the current project is installed into the environment with all of its dependencies. The `--no-install-project` option allows the project to be excluded, but all of its dependencies are still installed. This is particularly useful in situations like building Docker images where installing the project separately from its dependencies allows optimal layer caching.

The inverse `--only-install-project` can be used to install *only* the project itself, excluding all dependencies.
```

[`--no-install-workspace`](#fyn-sync--no-install-workspace) : Do not install any workspace members, including the root project.

```
By default, all workspace members and their dependencies are installed into the environment. The `--no-install-workspace` option allows exclusion of all the workspace members while retaining their dependencies. This is particularly useful in situations like building Docker images where installing the workspace separately from its dependencies allows optimal layer caching.

The inverse `--only-install-workspace` can be used to install *only* workspace members, excluding all other dependencies.
```

[`--no-managed-python`](#fyn-sync--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-sync--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-sync--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-sync--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-sync--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--offline`](#fyn-sync--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-dev`](#fyn-sync--only-dev) : Only include the development dependency group.

```
The project and its dependencies will be omitted.

This option is an alias for `--only-group dev`. Implies `--no-default-groups`.
```

[`--only-group`](#fyn-sync--only-group) *only-group* : Only include dependencies from the specified dependency group.

```
The project and its dependencies will be omitted.

May be provided multiple times. Implies `--no-default-groups`.
```

[`--output-format`](#fyn-sync--output-format) *output-format* : Select the output format

```
[default: text]

Possible values:

- `text`: Display the result in a human-readable format
- `json`: Display the result in JSON format
```

[`--package`](#fyn-sync--package) *package* : Sync for specific packages in the workspace.

```
The workspace's environment (`.venv`) is updated to reflect the subset of dependencies declared by the specified workspace member packages.

If any workspace member does not exist, fyn will exit with an error.
```

[`--prerelease`](#fyn-sync--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-sync--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-sync--python), `-p` *python* : The Python interpreter to use for the project environment.

```
By default, the first interpreter that meets the project's `requires-python` constraint is
used.

If a Python interpreter in a virtual environment is provided, the packages will not be
synced to the given environment. The interpreter will be used to create a virtual
environment in the project.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--python-platform`](#fyn-sync--python-platform) *python-platform* : The platform for which requirements should be installed.

```
Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

When targeting macOS (Darwin), the default minimum version is `13.0`. Use `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting iOS, the default minimum version is `13.0`. Use `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting Android, the default minimum Android API level is `24`. Use `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.

WARNING: When specified, fyn will select wheels that are compatible with the *target* platform; as a result, the installed distributions may not be compatible with the *current* platform. Conversely, any distributions that are built from source may be incompatible with the *target* platform, as they will be built for the *current* platform. The `--python-platform` option is intended for advanced use cases.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--quiet`](#fyn-sync--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-sync--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-sync--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--reinstall`](#fyn-sync--reinstall), `--force-reinstall` : Reinstall all packages, regardless of whether they're already installed. Implies `--refresh`

[`--reinstall-package`](#fyn-sync--reinstall-package) *reinstall-package* : Reinstall a specific package, regardless of whether it's already installed. Implies `--refresh-package`

[`--resolution`](#fyn-sync--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--script`](#fyn-sync--script) *script* : Sync the environment for a Python script, rather than the current project.

```
If provided, fyn will sync the dependencies based on the script's inline metadata table, in adherence with PEP 723.
```

[`--upgrade`](#fyn-sync--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-sync--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-sync--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn lock](#fyn-lock)

Update the project's lockfile.

If the project lockfile (`fyn.lock`) does not exist, it will be created. If a lockfile is present, its contents will be used as preferences for the resolution.

If there are no changes to the project's dependencies, locking will have no effect unless the `--upgrade` flag is provided.

### Usage

```
fyn lock [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-lock--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-lock--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--check`](#fyn-lock--check) : Check if the lockfile is up-to-date.

```
Asserts that the `fyn.lock` would remain unchanged after a resolution. If the lockfile is missing or needs to be updated, fyn will exit with an error.

Equivalent to `--locked`.
```

[`--check-exists`](#fyn-lock--check-exists), `--frozen` : Assert that a `fyn.lock` exists without checking if it is up-to-date [env: UV_FROZEN=]

```
Equivalent to `--frozen`.
```

[`--color`](#fyn-lock--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-lock--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-lock--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-lock--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--default-index`](#fyn-lock--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-lock--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--dry-run`](#fyn-lock--dry-run) : Perform a dry run, without writing the lockfile.

```
In dry-run mode, fyn will resolve the project's dependencies and report on the resulting changes, but will not write the lockfile to disk.
```

[`--exclude-newer`](#fyn-lock--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-lock--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra-index-url`](#fyn-lock--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-lock--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-lock--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--help`](#fyn-lock--help), `-h` : Display the concise help for this command

[`--index`](#fyn-lock--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-lock--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-lock--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-lock--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-lock--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
This option is only used when building source distributions.

Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-lock--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-lock--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-lock--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-lock--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-lock--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-lock--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-lock--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-lock--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-lock--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-lock--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-index`](#fyn-lock--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-lock--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-lock--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-lock--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-lock--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-lock--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--offline`](#fyn-lock--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--prerelease`](#fyn-lock--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-lock--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-lock--python), `-p` *python* : The Python interpreter to use during resolution.

```
A Python interpreter is required for building source distributions to determine package
metadata when there are not wheels.

The interpreter is also used as the fallback value for the minimum Python version if
`requires-python` is not set.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-lock--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-lock--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-lock--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--resolution`](#fyn-lock--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--script`](#fyn-lock--script) *script* : Lock the specified Python script, rather than the current project.

```
If provided, fyn will lock the script (based on its inline metadata table, in adherence with PEP 723) to a `.lock` file adjacent to the script itself.
```

[`--upgrade`](#fyn-lock--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-lock--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-lock--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn upgrade](#fyn-upgrade)

Upgrade project dependencies.

Re-resolves all (or specified) project dependencies to their latest compatible versions, updates the lockfile, and syncs the environment.

This is equivalent to running `fyn lock --upgrade` followed by `fyn sync`.

To upgrade specific packages, pass them as arguments, e.g., `fyn upgrade requests flask`.

### Usage

```
fyn upgrade [OPTIONS] [PACKAGES]...
```

### Arguments

[PACKAGES](#fyn-upgrade--packages) : The packages to upgrade.

```
If not provided, all packages in the project will be upgraded to their latest compatible versions.
```

### Options

[`--allow-insecure-host`](#fyn-upgrade--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-upgrade--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-upgrade--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-upgrade--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-upgrade--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--dry-run`](#fyn-upgrade--dry-run) : Perform a dry run, showing what would be upgraded without making changes

[`--help`](#fyn-upgrade--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-upgrade--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-upgrade--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-upgrade--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-upgrade--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-upgrade--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-upgrade--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-upgrade--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sync`](#fyn-upgrade--no-sync) : Do not sync the environment after upgrading the lockfile

[`--offline`](#fyn-upgrade--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-upgrade--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-upgrade--python), `-p` *python* : The Python interpreter to use during resolution.

```
May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-upgrade--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-upgrade--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn export](#fyn-export)

Export the project's lockfile to an alternate format.

At present, `requirements.txt`, `pylock.toml` (PEP 751) and CycloneDX v1.5 JSON output formats are supported.

The project is re-locked before exporting unless the `--locked` or `--frozen` flag is provided.

fyn will search for a project in the current directory or any parent directory. If a project cannot be found, fyn will exit with an error.

If operating in a workspace, the root will be exported by default; however, specific members can be selected using the `--package` option.

### Usage

```
fyn export [OPTIONS]
```

### Options

[`--all-extras`](#fyn-export--all-extras) : Include all optional dependencies

[`--all-groups`](#fyn-export--all-groups) : Include dependencies from all dependency groups.

```
`--no-group` can be used to exclude specific groups.
```

[`--all-packages`](#fyn-export--all-packages) : Export the entire workspace.

```
The dependencies for all workspace members will be included in the exported requirements file.

Any extras or groups specified via `--extra`, `--group`, or related options will be applied to all workspace members.
```

[`--allow-insecure-host`](#fyn-export--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-export--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-export--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-export--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-export--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-export--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--default-index`](#fyn-export--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-export--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-export--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-export--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra`](#fyn-export--extra) *extra* : Include optional dependencies from the specified extra name.

```
May be provided more than once.
```

[`--extra-index-url`](#fyn-export--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-export--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-export--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--format`](#fyn-export--format) *format* : The format to which `fyn.lock` should be exported.

```
Supports `requirements.txt`, `pylock.toml` (PEP 751) and CycloneDX v1.5 JSON output formats.

fyn will infer the output format from the file extension of the output file, if provided. Otherwise, defaults to `requirements.txt`.

Possible values:

- `requirements.txt`: Export in `requirements.txt` format
- `pylock.toml`: Export in `pylock.toml` format
- `cyclonedx1.5`: Export in `CycloneDX` v1.5 JSON format
```

[`--frozen`](#fyn-export--frozen) : Do not update the `fyn.lock` before exporting [env: UV_FROZEN=]

```
If a `fyn.lock` does not exist, fyn will exit with an error.
```

[`--group`](#fyn-export--group) *group* : Include dependencies from the specified dependency group.

```
May be provided multiple times.
```

[`--help`](#fyn-export--help), `-h` : Display the concise help for this command

[`--index`](#fyn-export--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-export--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-export--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-export--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-export--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
This option is only used when building source distributions.

Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--locked`](#fyn-export--locked) : Assert that the `fyn.lock` will remain unchanged [env: UV_LOCKED=]

```
Requires that the lockfile is up-to-date. If the lockfile is missing or needs to be updated, fyn will exit with an error.
```

[`--managed-python`](#fyn-export--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-export--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-annotate`](#fyn-export--no-annotate) : Exclude comment annotations indicating the source of each package

[`--no-binary`](#fyn-export--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-export--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-export--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-export--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-export--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-export--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-export--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-export--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-default-groups`](#fyn-export--no-default-groups) : Ignore the default dependency groups.

```
fyn includes the groups defined in `tool.fyn.default-groups` by default. This disables that option, however, specific groups can still be included with `--group`.

May also be set with the `UV_NO_DEFAULT_GROUPS` environment variable.
```

[`--no-dev`](#fyn-export--no-dev) : Disable the development dependency group [env: UV_NO_DEV=]

```
This option is an alias of `--no-group dev`. See `--no-default-groups` to disable all default groups instead.
```

[`--no-editable`](#fyn-export--no-editable) : Export any editable dependencies, including the project and any workspace members, as non-editable [env: UV_NO_EDITABLE=]

[`--no-emit-local`](#fyn-export--no-emit-local), `--no-install-local` : Do not include local path dependencies in the exported requirements.

```
Omits the current project, workspace members, and any other local (path or editable) packages from the export. Only remote/indexed dependencies are written. Useful for Docker and CI flows that want to export and cache third-party dependencies first.

The inverse `--only-emit-local` can be used to emit *only* local packages, excluding all remote dependencies.
```

[`--no-emit-package`](#fyn-export--no-emit-package), `--no-install-package` *no-emit-package* : Do not emit the given package(s).

```
By default, all project's dependencies are included in the exported requirements file. The `--no-emit-package` option allows exclusion of specific packages.

The inverse `--only-emit-package` can be used to emit *only* the specified packages, excluding all others.
```

[`--no-emit-project`](#fyn-export--no-emit-project), `--no-install-project` : Do not emit the current project.

```
By default, the current project is included in the exported requirements file with all of its dependencies. The `--no-emit-project` option allows the project to be excluded, but all of its dependencies to remain included.

The inverse `--only-emit-project` can be used to emit *only* the project itself, excluding all dependencies.
```

[`--no-emit-workspace`](#fyn-export--no-emit-workspace), `--no-install-workspace` : Do not emit any workspace members, including the root project.

```
By default, all workspace members and their dependencies are included in the exported requirements file, with all of their dependencies. The `--no-emit-workspace` option allows exclusion of all the workspace members while retaining their dependencies.

The inverse `--only-emit-workspace` can be used to emit *only* workspace members, excluding all other dependencies.
```

[`--no-extra`](#fyn-export--no-extra) *no-extra* : Exclude the specified optional dependencies, if `--all-extras` is supplied.

```
May be provided multiple times.
```

[`--no-group`](#fyn-export--no-group) *no-group* : Disable the specified dependency group.

```
This option always takes precedence over default groups, `--all-groups`, and `--group`.

May be provided multiple times.

May also be set with the `UV_NO_GROUP` environment variable.
```

[`--no-hashes`](#fyn-export--no-hashes) : Omit hashes in the generated output

[`--no-header`](#fyn-export--no-header) : Exclude the comment header at the top of the generated output file

[`--no-index`](#fyn-export--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-export--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-export--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-export--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-export--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-export--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--offline`](#fyn-export--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-dev`](#fyn-export--only-dev) : Only include the development dependency group.

```
The project and its dependencies will be omitted.

This option is an alias for `--only-group dev`. Implies `--no-default-groups`.
```

[`--only-group`](#fyn-export--only-group) *only-group* : Only include dependencies from the specified dependency group.

```
The project and its dependencies will be omitted.

May be provided multiple times. Implies `--no-default-groups`.
```

[`--output-file`](#fyn-export--output-file), `-o` *output-file* : Write the exported requirements to the given file

[`--package`](#fyn-export--package) *package* : Export the dependencies for specific packages in the workspace.

```
If any workspace member does not exist, fyn will exit with an error.
```

[`--prerelease`](#fyn-export--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-export--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--prune`](#fyn-export--prune) *package* : Prune the given package from the dependency tree.

```
Pruned packages will be excluded from the exported requirements file, as will any dependencies that are no longer required after the pruned package is removed.
```

[`--python`](#fyn-export--python), `-p` *python* : The Python interpreter to use during resolution.

```
A Python interpreter is required for building source distributions to determine package
metadata when there are not wheels.

The interpreter is also used as the fallback value for the minimum Python version if
`requires-python` is not set.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-export--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-export--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-export--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--resolution`](#fyn-export--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--script`](#fyn-export--script) *script* : Export the dependencies for the specified PEP 723 Python script, rather than the current project.

```
If provided, fyn will resolve the dependencies based on its inline metadata table, in adherence with PEP 723.
```

[`--upgrade`](#fyn-export--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-export--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-export--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn tree](#fyn-tree)

Display the project's dependency tree

### Usage

```
fyn tree [OPTIONS]
```

### Options

[`--all-groups`](#fyn-tree--all-groups) : Include dependencies from all dependency groups.

```
`--no-group` can be used to exclude specific groups.
```

[`--allow-insecure-host`](#fyn-tree--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-tree--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-tree--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-tree--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-tree--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-tree--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--default-index`](#fyn-tree--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--depth`](#fyn-tree--depth), `-d` *depth* : Maximum display depth of the dependency tree

```
[default: 255]
```

[`--directory`](#fyn-tree--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-tree--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-tree--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra-index-url`](#fyn-tree--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-tree--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-tree--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--frozen`](#fyn-tree--frozen) : Display the requirements without locking the project [env: UV_FROZEN=]

```
If the lockfile is missing, fyn will exit with an error.
```

[`--group`](#fyn-tree--group) *group* : Include dependencies from the specified dependency group.

```
May be provided multiple times.
```

[`--help`](#fyn-tree--help), `-h` : Display the concise help for this command

[`--index`](#fyn-tree--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-tree--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-tree--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--invert`](#fyn-tree--invert), `--reverse` : Show the reverse dependencies for the given package. This flag will invert the tree and display the packages that depend on the given package

[`--keyring-provider`](#fyn-tree--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-tree--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
This option is only used when building source distributions.

Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--locked`](#fyn-tree--locked) : Assert that the `fyn.lock` will remain unchanged [env: UV_LOCKED=]

```
Requires that the lockfile is up-to-date. If the lockfile is missing or needs to be updated, fyn will exit with an error.
```

[`--managed-python`](#fyn-tree--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-tree--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-tree--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-tree--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-tree--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-tree--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-tree--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-tree--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-tree--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-tree--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-dedupe`](#fyn-tree--no-dedupe) : Do not de-duplicate repeated dependencies. Usually, when a package has already displayed its dependencies, further occurrences will not re-display its dependencies, and will include a (\*) to indicate it has already been shown. This flag will cause those duplicates to be repeated

[`--no-default-groups`](#fyn-tree--no-default-groups) : Ignore the default dependency groups.

```
fyn includes the groups defined in `tool.fyn.default-groups` by default. This disables that option, however, specific groups can still be included with `--group`.

May also be set with the `UV_NO_DEFAULT_GROUPS` environment variable.
```

[`--no-dev`](#fyn-tree--no-dev) : Disable the development dependency group [env: UV_NO_DEV=]

```
This option is an alias of `--no-group dev`. See `--no-default-groups` to disable all default groups instead.
```

[`--no-group`](#fyn-tree--no-group) *no-group* : Disable the specified dependency group.

```
This option always takes precedence over default groups, `--all-groups`, and `--group`.

May be provided multiple times.

May also be set with the `UV_NO_GROUP` environment variable.
```

[`--no-index`](#fyn-tree--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-tree--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-tree--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-tree--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-tree--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-tree--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--offline`](#fyn-tree--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-dev`](#fyn-tree--only-dev) : Only include the development dependency group.

```
The project and its dependencies will be omitted.

This option is an alias for `--only-group dev`. Implies `--no-default-groups`.
```

[`--only-group`](#fyn-tree--only-group) *only-group* : Only include dependencies from the specified dependency group.

```
The project and its dependencies will be omitted.

May be provided multiple times. Implies `--no-default-groups`.
```

[`--outdated`](#fyn-tree--outdated) : Show the latest available version of each package in the tree

[`--package`](#fyn-tree--package) *package* : Display only the specified packages

[`--prerelease`](#fyn-tree--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-tree--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--prune`](#fyn-tree--prune) *prune* : Prune the given package from the display of the dependency tree

[`--python`](#fyn-tree--python), `-p` *python* : The Python interpreter to use for locking and filtering.

```
By default, the tree is filtered to match the platform as reported by the Python
interpreter. Use `--universal` to display the tree for all platforms, or use
`--python-version` or `--python-platform` to override a subset of markers.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--python-platform`](#fyn-tree--python-platform) *python-platform* : The platform to use when filtering the tree.

```
For example, pass `--platform windows` to display the dependencies that would be included when installing on Windows.

Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--python-version`](#fyn-tree--python-version) *python-version* : The Python version to use when filtering the tree.

```
For example, pass `--python-version 3.10` to display the dependencies that would be included when installing on Python 3.10.

Defaults to the version of the discovered Python interpreter.
```

[`--quiet`](#fyn-tree--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--resolution`](#fyn-tree--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--script`](#fyn-tree--script) *script* : Show the dependency tree the specified PEP 723 Python script, rather than the current project.

```
If provided, fyn will resolve the dependencies based on its inline metadata table, in adherence with PEP 723.
```

[`--show-sizes`](#fyn-tree--show-sizes) : Show compressed wheel sizes for packages in the tree

[`--universal`](#fyn-tree--universal) : Show a platform-independent dependency tree.

```
Shows resolved package versions for all Python versions and platforms, rather than filtering to those that are relevant for the current environment.

Multiple versions may be shown for a each package.
```

[`--upgrade`](#fyn-tree--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-tree--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-tree--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn format](#fyn-format)

Format Python code in the project.

Formats Python code using the Ruff formatter. By default, all Python files in the project are formatted. This command has the same behavior as running `ruff format` in the project root.

To check if files are formatted without modifying them, use `--check`. To see a diff of formatting changes, use `--diff`.

Additional arguments can be passed to Ruff after `--`.

### Usage

```
fyn format [OPTIONS] [-- <EXTRA_ARGS>...]
```

### Arguments

[EXTRA_ARGS](#fyn-format--extra_args) : Additional arguments to pass to Ruff.

```
For example, use `fyn format -- --line-length 100` to set the line length or `fyn format -- src/module/foo.py` to format a specific file.
```

### Options

[`--allow-insecure-host`](#fyn-format--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-format--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--check`](#fyn-format--check) : Check if files are formatted without applying changes

[`--color`](#fyn-format--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-format--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--diff`](#fyn-format--diff) : Show a diff of formatting changes without applying them.

```
Implies `--check`.
```

[`--directory`](#fyn-format--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-format--exclude-newer) *exclude-newer* : Limit candidate Ruff versions to those released prior to the given date.

```
Accepts a superset of [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (e.g., `2006-12-02T02:07:43Z`) or local date in the same format (e.g. `2006-12-02`), as well as durations relative to "now" (e.g., `-1 week`).

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--help`](#fyn-format--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-format--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-format--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-format--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-format--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-format--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-format--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-project`](#fyn-format--no-project) : Avoid discovering a project or workspace.

```
Instead of running the formatter in the context of the current project, run it in the context of the current directory. This is useful when the current directory is not a project.
```

[`--no-python-downloads`](#fyn-format--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-format--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-format--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-format--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-format--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

[`--version`](#fyn-format--version) *version* : The version of Ruff to use for formatting.

```
Accepts either a version (e.g., `0.8.2`) which will be treated as an exact pin, a version specifier (e.g., `>=0.8.0`), or `latest` to use the latest available version.

By default, a constrained version range of Ruff will be used (e.g., `>=0.15,<0.16`).
```

## [fyn audit](#fyn-audit)

Audit the project's dependencies.

Dependencies are audited for known vulnerabilities, as well as 'adverse' statuses such as deprecation and quarantine.

### Usage

```
fyn audit [OPTIONS]
```

### Options

[`--all-extras`](#fyn-audit--all-extras) : Include all optional dependencies.

```
Optional dependencies are defined via `project.optional-dependencies` in a `pyproject.toml`.

This option is only available when running in a project.
```

[`--all-groups`](#fyn-audit--all-groups) : Include dependencies from all dependency groups.

```
`--no-group` can be used to exclude specific groups.
```

[`--allow-insecure-host`](#fyn-audit--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-audit--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-audit--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-audit--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-audit--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-audit--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--default-index`](#fyn-audit--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-audit--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-audit--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-audit--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra`](#fyn-audit--extra) *extra* : Include optional dependencies from the specified extra name.

```
May be provided more than once.

This option is only available when running in a project.
```

[`--extra-index-url`](#fyn-audit--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-audit--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-audit--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--frozen`](#fyn-audit--frozen) : Audit the requirements without locking the project [env: UV_FROZEN=]

```
If the lockfile is missing, fyn will exit with an error.
```

[`--group`](#fyn-audit--group) *group* : Include dependencies from the specified dependency group.

```
May be provided multiple times.
```

[`--help`](#fyn-audit--help), `-h` : Display the concise help for this command

[`--ignore`](#fyn-audit--ignore) *ignore* : A vulnerability ID to ignore during auditing.

```
May be provided multiple times.
```

[`--ignore-until-fixed`](#fyn-audit--ignore-until-fixed) *ignore-until-fixed* : A vulnerability ID to ignore during auditing, but only while no fix is available.

```
May be provided multiple times.
```

[`--index`](#fyn-audit--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-audit--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-audit--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-audit--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-audit--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
This option is only used when building source distributions.

Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--locked`](#fyn-audit--locked) : Assert that the `fyn.lock` will remain unchanged [env: UV_LOCKED=]

```
Requires that the lockfile is up-to-date. If the lockfile is missing or needs to be updated, fyn will exit with an error.
```

[`--managed-python`](#fyn-audit--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-audit--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-audit--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-audit--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-audit--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-audit--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-audit--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-audit--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-audit--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-audit--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-default-groups`](#fyn-audit--no-default-groups) : Ignore the default dependency groups.

```
fyn includes the groups defined in `tool.fyn.default-groups` by default. This disables that option, however, specific groups can still be included with `--group`.

May also be set with the `UV_NO_DEFAULT_GROUPS` environment variable.
```

[`--no-dev`](#fyn-audit--no-dev) : Disable the development dependency group [env: UV_NO_DEV=]

```
This option is an alias of `--no-group dev`. See `--no-default-groups` to disable all default groups instead.

This option is only available when running in a project.
```

[`--no-extra`](#fyn-audit--no-extra) *no-extra* : Exclude the specified optional dependencies, if `--all-extras` is supplied.

```
May be provided multiple times.
```

[`--no-group`](#fyn-audit--no-group) *no-group* : Disable the specified dependency group.

```
This option always takes precedence over default groups, `--all-groups`, and `--group`.

May be provided multiple times.

May also be set with the `UV_NO_GROUP` environment variable.
```

[`--no-index`](#fyn-audit--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-audit--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-audit--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-audit--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-audit--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-audit--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--offline`](#fyn-audit--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-dev`](#fyn-audit--only-dev) : Only include the development dependency group.

```
The project and its dependencies will be omitted.

This option is an alias for `--only-group dev`. Implies `--no-default-groups`.
```

[`--only-group`](#fyn-audit--only-group) *only-group* : Only include dependencies from the specified dependency group.

```
The project and its dependencies will be omitted.

May be provided multiple times. Implies `--no-default-groups`.
```

[`--prerelease`](#fyn-audit--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-audit--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python-platform`](#fyn-audit--python-platform) *python-platform* : The platform to use when auditing.

```
For example, pass `--platform windows` to audit the dependencies that would be included when installing on Windows.

Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--python-version`](#fyn-audit--python-version) *python-version* : The Python version to use when auditing.

```
For example, pass `--python-version 3.10` to audit the dependencies that would be included when installing on Python 3.10.

Defaults to the version of the discovered Python interpreter.
```

[`--quiet`](#fyn-audit--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--resolution`](#fyn-audit--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--script`](#fyn-audit--script) *script* : Audit the specified PEP 723 Python script, rather than the current project.

```
The specified script must be locked, i.e. with `fyn lock --script <script>` before it can be audited.
```

[`--service-format`](#fyn-audit--service-format) *service-format* : The service format to use for vulnerability lookups.

```
Each service format has a default URL, which can be changed with `--service-url`. The defaults are:

- OSV: <https://api.osv.dev/>

[default: osv]

Possible values:

- `osv`
```

[`--service-url`](#fyn-audit--service-url) *service-url* : The base URL for the vulnerability service API.

```
If not provided, the default URL for the selected service will be used.

The service needs to use the OSV protocol, unless a different format was requested by `--service-format`.
```

[`--upgrade`](#fyn-audit--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-audit--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-audit--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn tool](#fyn-tool)

Run and install commands provided by Python packages

### Usage

```
fyn tool [OPTIONS] <COMMAND>
```

### Commands

[`fyn tool run`](#fyn-tool-run) : Run a command provided by a Python package

[`fyn tool install`](#fyn-tool-install) : Install commands provided by a Python package

[`fyn tool upgrade`](#fyn-tool-upgrade) : Upgrade installed tools

[`fyn tool list`](#fyn-tool-list) : List installed tools

[`fyn tool uninstall`](#fyn-tool-uninstall) : Uninstall a tool

[`fyn tool update-shell`](#fyn-tool-update-shell) : Ensure that the tool executable directory is on the `PATH`

[`fyn tool dir`](#fyn-tool-dir) : Show the path to the fyn tools directory

### [fyn tool run](#fyn-tool-run)

Run a command provided by a Python package.

By default, the package to install is assumed to match the command name.

The name of the command can include an exact version in the format `<package>@<version>`, e.g., `fyn tool run ruff@0.3.0`. If more complex version specification is desired or if the command is provided by a different package, use `--from`.

`fynx` can be used to invoke Python, e.g., with `fynx python` or `fynx python@<version>`. A Python interpreter will be started in an isolated virtual environment.

If the tool was previously installed, i.e., via `fyn tool install`, the installed version will be used unless a version is requested or the `--isolated` flag is used.

`fynx` is provided as a convenient alias for `fyn tool run`, their behavior is identical.

If no command is provided, the installed tools are displayed.

Packages are installed into an ephemeral virtual environment in the fyn cache directory.

### Usage

```
fyn tool run [OPTIONS] [COMMAND]
```

### Options

[`--allow-insecure-host`](#fyn-tool-run--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--build-constraints`](#fyn-tool-run--build-constraints), `--build-constraint`, `-b` *build-constraints* : Constrain build dependencies using the given requirements files when building source distributions.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

May also be set with the `UV_BUILD_CONSTRAINT` environment variable.
```

[`--cache-dir`](#fyn-tool-run--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-tool-run--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-tool-run--compile-bytecode), `--compile` : Compile Python files to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-tool-run--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-tool-run--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-tool-run--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--constraints`](#fyn-tool-run--constraints), `--constraint`, `-c` *constraints* : Constrain versions using the given requirements files.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

This is equivalent to pip's `--constraint` option.

May also be set with the `UV_CONSTRAINT` environment variable.
```

[`--default-index`](#fyn-tool-run--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-tool-run--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--env-file`](#fyn-tool-run--env-file) *env-file* : Load environment variables from a `.env` file.

```
Can be provided multiple times, with subsequent files overriding values defined in previous files.

May also be set with the `UV_ENV_FILE` environment variable.
```

[`--exclude-newer`](#fyn-tool-run--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-tool-run--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra-index-url`](#fyn-tool-run--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-tool-run--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-tool-run--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--from`](#fyn-tool-run--from) *from* : Use the given package to provide the command.

```
By default, the package name is assumed to match the command name.
```

[`--help`](#fyn-tool-run--help), `-h` : Display the concise help for this command

[`--index`](#fyn-tool-run--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-tool-run--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-tool-run--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--isolated`](#fyn-tool-run--isolated) : Run the tool in an isolated virtual environment, ignoring any already-installed tools [env: UV_ISOLATED=]

[`--keyring-provider`](#fyn-tool-run--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--lfs`](#fyn-tool-run--lfs) : Whether to use Git LFS when adding a dependency from Git

[`--link-mode`](#fyn-tool-run--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-tool-run--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-tool-run--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-tool-run--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-tool-run--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-tool-run--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-tool-run--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-tool-run--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-tool-run--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-tool-run--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-tool-run--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-env-file`](#fyn-tool-run--no-env-file) : Avoid reading environment variables from a `.env` file [env: UV_NO_ENV_FILE=]

[`--no-index`](#fyn-tool-run--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-tool-run--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-tool-run--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-tool-run--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-tool-run--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-tool-run--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--offline`](#fyn-tool-run--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--overrides`](#fyn-tool-run--overrides), `--override` *overrides* : Override versions using the given requirements files.

```
Overrides files are `requirements.txt`-like files that force a specific version of a requirement to be installed, regardless of the requirements declared by any constituent package, and regardless of whether this would be considered an invalid resolution.

While constraints are *additive*, in that they're combined with the requirements of the constituent packages, overrides are *absolute*, in that they completely replace the requirements of the constituent packages.

May also be set with the `UV_OVERRIDE` environment variable.
```

[`--prerelease`](#fyn-tool-run--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-tool-run--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-tool-run--python), `-p` *python* : The Python interpreter to use to build the run environment.

```
See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--python-platform`](#fyn-tool-run--python-platform) *python-platform* : The platform for which requirements should be installed.

```
Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

When targeting macOS (Darwin), the default minimum version is `13.0`. Use `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting iOS, the default minimum version is `13.0`. Use `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting Android, the default minimum Android API level is `24`. Use `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.

WARNING: When specified, fyn will select wheels that are compatible with the *target* platform; as a result, the installed distributions may not be compatible with the *current* platform. Conversely, any distributions that are built from source may be incompatible with the *target* platform, as they will be built for the *current* platform. The `--python-platform` option is intended for advanced use cases.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--quiet`](#fyn-tool-run--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-tool-run--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-tool-run--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--reinstall`](#fyn-tool-run--reinstall), `--force-reinstall` : Reinstall all packages, regardless of whether they're already installed. Implies `--refresh`

[`--reinstall-package`](#fyn-tool-run--reinstall-package) *reinstall-package* : Reinstall a specific package, regardless of whether it's already installed. Implies `--refresh-package`

[`--resolution`](#fyn-tool-run--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--torch-backend`](#fyn-tool-run--torch-backend) *torch-backend* : The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`)

```
When set, fyn will ignore the configured index URLs for packages in the PyTorch ecosystem, and will instead use the defined backend.

For example, when set to `cpu`, fyn will use the CPU-only PyTorch index; when set to `cu126`, fyn will use the PyTorch index for CUDA 12.6.

The `auto` mode will attempt to detect the appropriate PyTorch index based on the currently installed CUDA drivers.

This option is in preview and may change in any future release.

May also be set with the `UV_TORCH_BACKEND` environment variable.

Possible values:

- `auto`: Select the appropriate PyTorch index based on the operating system and CUDA driver version
- `cpu`: Use the CPU-only PyTorch index
- `cu130`: Use the PyTorch index for CUDA 13.0
- `cu129`: Use the PyTorch index for CUDA 12.9
- `cu128`: Use the PyTorch index for CUDA 12.8
- `cu126`: Use the PyTorch index for CUDA 12.6
- `cu125`: Use the PyTorch index for CUDA 12.5
- `cu124`: Use the PyTorch index for CUDA 12.4
- `cu123`: Use the PyTorch index for CUDA 12.3
- `cu122`: Use the PyTorch index for CUDA 12.2
- `cu121`: Use the PyTorch index for CUDA 12.1
- `cu120`: Use the PyTorch index for CUDA 12.0
- `cu118`: Use the PyTorch index for CUDA 11.8
- `cu117`: Use the PyTorch index for CUDA 11.7
- `cu116`: Use the PyTorch index for CUDA 11.6
- `cu115`: Use the PyTorch index for CUDA 11.5
- `cu114`: Use the PyTorch index for CUDA 11.4
- `cu113`: Use the PyTorch index for CUDA 11.3
- `cu112`: Use the PyTorch index for CUDA 11.2
- `cu111`: Use the PyTorch index for CUDA 11.1
- `cu110`: Use the PyTorch index for CUDA 11.0
- `cu102`: Use the PyTorch index for CUDA 10.2
- `cu101`: Use the PyTorch index for CUDA 10.1
- `cu100`: Use the PyTorch index for CUDA 10.0
- `cu92`: Use the PyTorch index for CUDA 9.2
- `cu91`: Use the PyTorch index for CUDA 9.1
- `cu90`: Use the PyTorch index for CUDA 9.0
- `cu80`: Use the PyTorch index for CUDA 8.0
- `rocm7.1`: Use the PyTorch index for ROCm 7.1
- `rocm7.0`: Use the PyTorch index for ROCm 7.0
- `rocm6.4`: Use the PyTorch index for ROCm 6.4
- `rocm6.3`: Use the PyTorch index for ROCm 6.3
- `rocm6.2.4`: Use the PyTorch index for ROCm 6.2.4
- `rocm6.2`: Use the PyTorch index for ROCm 6.2
- `rocm6.1`: Use the PyTorch index for ROCm 6.1
- `rocm6.0`: Use the PyTorch index for ROCm 6.0
- `rocm5.7`: Use the PyTorch index for ROCm 5.7
- `rocm5.6`: Use the PyTorch index for ROCm 5.6
- `rocm5.5`: Use the PyTorch index for ROCm 5.5
- `rocm5.4.2`: Use the PyTorch index for ROCm 5.4.2
- `rocm5.4`: Use the PyTorch index for ROCm 5.4
- `rocm5.3`: Use the PyTorch index for ROCm 5.3
- `rocm5.2`: Use the PyTorch index for ROCm 5.2
- `rocm5.1.1`: Use the PyTorch index for ROCm 5.1.1
- `rocm4.2`: Use the PyTorch index for ROCm 4.2
- `rocm4.1`: Use the PyTorch index for ROCm 4.1
- `rocm4.0.1`: Use the PyTorch index for ROCm 4.0.1
- `xpu`: Use the PyTorch index for Intel XPU
```

[`--upgrade`](#fyn-tool-run--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-tool-run--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-tool-run--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

[`--with`](#fyn-tool-run--with), `-w` *with* : Run with the given packages installed

[`--with-editable`](#fyn-tool-run--with-editable) *with-editable* : Run with the given packages installed in editable mode

```
When used in a project, these dependencies will be layered on top of the fyn tool's environment in a separate, ephemeral environment. These dependencies are allowed to conflict with those specified.
```

[`--with-requirements`](#fyn-tool-run--with-requirements) *with-requirements* : Run with the packages listed in the given files.

```
The following formats are supported: `requirements.txt`, `.py` files with inline metadata, and `pylock.toml`.
```

### [fyn tool install](#fyn-tool-install)

Install commands provided by a Python package.

Packages are installed into an isolated virtual environment in the fyn tools directory. The executables are linked the tool executable directory, which is determined according to the XDG standard and can be retrieved with `fyn tool dir --bin`.

If the tool was previously installed, the existing tool will generally be replaced.

### Usage

```
fyn tool install [OPTIONS] <PACKAGE>
```

### Arguments

[PACKAGE](#fyn-tool-install--package) : The package to install commands from

### Options

[`--allow-insecure-host`](#fyn-tool-install--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--build-constraints`](#fyn-tool-install--build-constraints), `--build-constraint`, `-b` *build-constraints* : Constrain build dependencies using the given requirements files when building source distributions.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

May also be set with the `UV_BUILD_CONSTRAINT` environment variable.
```

[`--cache-dir`](#fyn-tool-install--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-tool-install--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-tool-install--compile-bytecode), `--compile` : Compile Python files to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-tool-install--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-tool-install--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-tool-install--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--constraints`](#fyn-tool-install--constraints), `--constraint`, `-c` *constraints* : Constrain versions using the given requirements files.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

This is equivalent to pip's `--constraint` option.

May also be set with the `UV_CONSTRAINT` environment variable.
```

[`--default-index`](#fyn-tool-install--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-tool-install--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--editable`](#fyn-tool-install--editable), `-e` : Install the target package in editable mode, such that changes in the package's source directory are reflected without reinstallation

[`--exclude-newer`](#fyn-tool-install--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-tool-install--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--excludes`](#fyn-tool-install--excludes), `--exclude` *excludes* : Exclude packages from resolution using the given requirements files.

```
Excludes files are `requirements.txt`-like files that specify packages to exclude from the resolution. When a package is excluded, it will be omitted from the dependency list entirely and its own dependencies will be ignored during the resolution phase. Excludes are unconditional in that requirement specifiers and markers are ignored; any package listed in the provided file will be omitted from all resolved environments.

May also be set with the `UV_EXCLUDE` environment variable.
```

[`--extra-index-url`](#fyn-tool-install--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-tool-install--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--force`](#fyn-tool-install--force) : Force installation of the tool.

```
Will recreate any existing environment for the tool and replace any existing entry points with the same name in the executable directory.
```

[`--fork-strategy`](#fyn-tool-install--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--help`](#fyn-tool-install--help), `-h` : Display the concise help for this command

[`--index`](#fyn-tool-install--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-tool-install--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-tool-install--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-tool-install--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--lfs`](#fyn-tool-install--lfs) : Whether to use Git LFS when adding a dependency from Git

[`--link-mode`](#fyn-tool-install--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-tool-install--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-tool-install--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-tool-install--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-tool-install--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-tool-install--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-tool-install--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-tool-install--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-tool-install--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-tool-install--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-tool-install--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-index`](#fyn-tool-install--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-tool-install--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-tool-install--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-tool-install--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-tool-install--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-tool-install--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--offline`](#fyn-tool-install--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--overrides`](#fyn-tool-install--overrides), `--override` *overrides* : Override versions using the given requirements files.

```
Overrides files are `requirements.txt`-like files that force a specific version of a requirement to be installed, regardless of the requirements declared by any constituent package, and regardless of whether this would be considered an invalid resolution.

While constraints are *additive*, in that they're combined with the requirements of the constituent packages, overrides are *absolute*, in that they completely replace the requirements of the constituent packages.

May also be set with the `UV_OVERRIDE` environment variable.
```

[`--prerelease`](#fyn-tool-install--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-tool-install--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-tool-install--python), `-p` *python* : The Python interpreter to use to build the tool environment.

```
See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--python-platform`](#fyn-tool-install--python-platform) *python-platform* : The platform for which requirements should be installed.

```
Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

When targeting macOS (Darwin), the default minimum version is `13.0`. Use `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting iOS, the default minimum version is `13.0`. Use `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting Android, the default minimum Android API level is `24`. Use `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.

WARNING: When specified, fyn will select wheels that are compatible with the *target* platform; as a result, the installed distributions may not be compatible with the *current* platform. Conversely, any distributions that are built from source may be incompatible with the *target* platform, as they will be built for the *current* platform. The `--python-platform` option is intended for advanced use cases.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--quiet`](#fyn-tool-install--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-tool-install--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-tool-install--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--reinstall`](#fyn-tool-install--reinstall), `--force-reinstall` : Reinstall all packages, regardless of whether they're already installed. Implies `--refresh`

[`--reinstall-package`](#fyn-tool-install--reinstall-package) *reinstall-package* : Reinstall a specific package, regardless of whether it's already installed. Implies `--refresh-package`

[`--resolution`](#fyn-tool-install--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--torch-backend`](#fyn-tool-install--torch-backend) *torch-backend* : The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`)

```
When set, fyn will ignore the configured index URLs for packages in the PyTorch ecosystem, and will instead use the defined backend.

For example, when set to `cpu`, fyn will use the CPU-only PyTorch index; when set to `cu126`, fyn will use the PyTorch index for CUDA 12.6.

The `auto` mode will attempt to detect the appropriate PyTorch index based on the currently installed CUDA drivers.

This option is in preview and may change in any future release.

May also be set with the `UV_TORCH_BACKEND` environment variable.

Possible values:

- `auto`: Select the appropriate PyTorch index based on the operating system and CUDA driver version
- `cpu`: Use the CPU-only PyTorch index
- `cu130`: Use the PyTorch index for CUDA 13.0
- `cu129`: Use the PyTorch index for CUDA 12.9
- `cu128`: Use the PyTorch index for CUDA 12.8
- `cu126`: Use the PyTorch index for CUDA 12.6
- `cu125`: Use the PyTorch index for CUDA 12.5
- `cu124`: Use the PyTorch index for CUDA 12.4
- `cu123`: Use the PyTorch index for CUDA 12.3
- `cu122`: Use the PyTorch index for CUDA 12.2
- `cu121`: Use the PyTorch index for CUDA 12.1
- `cu120`: Use the PyTorch index for CUDA 12.0
- `cu118`: Use the PyTorch index for CUDA 11.8
- `cu117`: Use the PyTorch index for CUDA 11.7
- `cu116`: Use the PyTorch index for CUDA 11.6
- `cu115`: Use the PyTorch index for CUDA 11.5
- `cu114`: Use the PyTorch index for CUDA 11.4
- `cu113`: Use the PyTorch index for CUDA 11.3
- `cu112`: Use the PyTorch index for CUDA 11.2
- `cu111`: Use the PyTorch index for CUDA 11.1
- `cu110`: Use the PyTorch index for CUDA 11.0
- `cu102`: Use the PyTorch index for CUDA 10.2
- `cu101`: Use the PyTorch index for CUDA 10.1
- `cu100`: Use the PyTorch index for CUDA 10.0
- `cu92`: Use the PyTorch index for CUDA 9.2
- `cu91`: Use the PyTorch index for CUDA 9.1
- `cu90`: Use the PyTorch index for CUDA 9.0
- `cu80`: Use the PyTorch index for CUDA 8.0
- `rocm7.1`: Use the PyTorch index for ROCm 7.1
- `rocm7.0`: Use the PyTorch index for ROCm 7.0
- `rocm6.4`: Use the PyTorch index for ROCm 6.4
- `rocm6.3`: Use the PyTorch index for ROCm 6.3
- `rocm6.2.4`: Use the PyTorch index for ROCm 6.2.4
- `rocm6.2`: Use the PyTorch index for ROCm 6.2
- `rocm6.1`: Use the PyTorch index for ROCm 6.1
- `rocm6.0`: Use the PyTorch index for ROCm 6.0
- `rocm5.7`: Use the PyTorch index for ROCm 5.7
- `rocm5.6`: Use the PyTorch index for ROCm 5.6
- `rocm5.5`: Use the PyTorch index for ROCm 5.5
- `rocm5.4.2`: Use the PyTorch index for ROCm 5.4.2
- `rocm5.4`: Use the PyTorch index for ROCm 5.4
- `rocm5.3`: Use the PyTorch index for ROCm 5.3
- `rocm5.2`: Use the PyTorch index for ROCm 5.2
- `rocm5.1.1`: Use the PyTorch index for ROCm 5.1.1
- `rocm4.2`: Use the PyTorch index for ROCm 4.2
- `rocm4.1`: Use the PyTorch index for ROCm 4.1
- `rocm4.0.1`: Use the PyTorch index for ROCm 4.0.1
- `xpu`: Use the PyTorch index for Intel XPU
```

[`--upgrade`](#fyn-tool-install--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-tool-install--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-tool-install--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

[`--with`](#fyn-tool-install--with), `-w` *with* : Include the following additional requirements

[`--with-editable`](#fyn-tool-install--with-editable) *with-editable* : Include the given packages in editable mode

[`--with-executables-from`](#fyn-tool-install--with-executables-from) *with-executables-from* : Install executables from the following packages

[`--with-requirements`](#fyn-tool-install--with-requirements) *with-requirements* : Run with the packages listed in the given files.

```
The following formats are supported: `requirements.txt`, `.py` files with inline metadata, and `pylock.toml`.
```

### [fyn tool upgrade](#fyn-tool-upgrade)

Upgrade installed tools.

If a tool was installed with version constraints, they will be respected on upgrade — to upgrade a tool beyond the originally provided constraints, use `fyn tool install` again.

If a tool was installed with specific settings, they will be respected on upgraded. For example, if `--prereleases allow` was provided during installation, it will continue to be respected in upgrades.

### Usage

```
fyn tool upgrade [OPTIONS] <NAME>...
```

### Arguments

[NAME](#fyn-tool-upgrade--name) : The name of the tool to upgrade, along with an optional version specifier

### Options

[`--all`](#fyn-tool-upgrade--all) : Upgrade all tools

[`--allow-insecure-host`](#fyn-tool-upgrade--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-tool-upgrade--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-tool-upgrade--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-tool-upgrade--compile-bytecode), `--compile` : Compile Python files to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-tool-upgrade--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-tool-upgrade--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-setting-package`](#fyn-tool-upgrade--config-setting-package), `--config-settings-package` *config-setting-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--default-index`](#fyn-tool-upgrade--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-tool-upgrade--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-tool-upgrade--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-tool-upgrade--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra-index-url`](#fyn-tool-upgrade--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-tool-upgrade--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-tool-upgrade--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--help`](#fyn-tool-upgrade--help), `-h` : Display the concise help for this command

[`--index`](#fyn-tool-upgrade--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-tool-upgrade--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-tool-upgrade--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-tool-upgrade--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-tool-upgrade--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-tool-upgrade--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-tool-upgrade--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-tool-upgrade--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-tool-upgrade--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-tool-upgrade--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-tool-upgrade--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-tool-upgrade--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-package`](#fyn-tool-upgrade--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-tool-upgrade--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-tool-upgrade--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-index`](#fyn-tool-upgrade--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-tool-upgrade--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-tool-upgrade--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-tool-upgrade--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-tool-upgrade--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-tool-upgrade--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--offline`](#fyn-tool-upgrade--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--prerelease`](#fyn-tool-upgrade--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-tool-upgrade--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-tool-upgrade--python), `-p` *python* : Upgrade a tool, and specify it to use the given Python interpreter to build its environment. Use with `--all` to apply to all tools.

```
See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--python-platform`](#fyn-tool-upgrade--python-platform) *python-platform* : The platform for which requirements should be installed.

```
Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

When targeting macOS (Darwin), the default minimum version is `13.0`. Use `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting iOS, the default minimum version is `13.0`. Use `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting Android, the default minimum Android API level is `24`. Use `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.

WARNING: When specified, fyn will select wheels that are compatible with the *target* platform; as a result, the installed distributions may not be compatible with the *current* platform. Conversely, any distributions that are built from source may be incompatible with the *target* platform, as they will be built for the *current* platform. The `--python-platform` option is intended for advanced use cases.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--quiet`](#fyn-tool-upgrade--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--reinstall`](#fyn-tool-upgrade--reinstall), `--force-reinstall` : Reinstall all packages, regardless of whether they're already installed. Implies `--refresh`

[`--reinstall-package`](#fyn-tool-upgrade--reinstall-package) *reinstall-package* : Reinstall a specific package, regardless of whether it's already installed. Implies `--refresh-package`

[`--resolution`](#fyn-tool-upgrade--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--verbose`](#fyn-tool-upgrade--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn tool list](#fyn-tool-list)

List installed tools

### Usage

```
fyn tool list [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-tool-list--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-tool-list--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-tool-list--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-tool-list--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-tool-list--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-tool-list--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-tool-list--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--help`](#fyn-tool-list--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-tool-list--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-tool-list--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-tool-list--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-tool-list--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-tool-list--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-tool-list--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--offline`](#fyn-tool-list--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--outdated`](#fyn-tool-list--outdated) : List outdated tools.

```
The latest version of each tool will be shown alongside the installed version. Up-to-date tools will be omitted from the output.
```

[`--project`](#fyn-tool-list--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-tool-list--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--show-extras`](#fyn-tool-list--show-extras) : Whether to display the extra requirements installed with each tool

[`--show-paths`](#fyn-tool-list--show-paths) : Whether to display the path to each tool environment and installed executable

[`--show-python`](#fyn-tool-list--show-python) : Whether to display the Python version associated with each tool

[`--show-version-specifiers`](#fyn-tool-list--show-version-specifiers) : Whether to display the version specifier(s) used to install each tool

[`--show-with`](#fyn-tool-list--show-with) : Whether to display the additional requirements installed with each tool

[`--verbose`](#fyn-tool-list--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn tool uninstall](#fyn-tool-uninstall)

Uninstall a tool

### Usage

```
fyn tool uninstall [OPTIONS] <NAME>...
```

### Arguments

[NAME](#fyn-tool-uninstall--name) : The name of the tool to uninstall

### Options

[`--all`](#fyn-tool-uninstall--all) : Uninstall all tools

[`--allow-insecure-host`](#fyn-tool-uninstall--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-tool-uninstall--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-tool-uninstall--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-tool-uninstall--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-tool-uninstall--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-tool-uninstall--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-tool-uninstall--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-tool-uninstall--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-tool-uninstall--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-tool-uninstall--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-tool-uninstall--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-tool-uninstall--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-tool-uninstall--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-tool-uninstall--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-tool-uninstall--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-tool-uninstall--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-tool-uninstall--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn tool update-shell](#fyn-tool-update-shell)

Ensure that the tool executable directory is on the `PATH`.

If the tool executable directory is not present on the `PATH`, fyn will attempt to add it to the relevant shell configuration files.

If the shell configuration files already include a blurb to add the executable directory to the path, but the directory is not present on the `PATH`, fyn will exit with an error.

The tool executable directory is determined according to the XDG standard and can be retrieved with `fyn tool dir --bin`.

### Usage

```
fyn tool update-shell [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-tool-update-shell--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-tool-update-shell--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-tool-update-shell--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-tool-update-shell--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-tool-update-shell--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-tool-update-shell--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-tool-update-shell--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-tool-update-shell--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-tool-update-shell--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-tool-update-shell--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-tool-update-shell--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-tool-update-shell--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-tool-update-shell--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-tool-update-shell--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-tool-update-shell--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-tool-update-shell--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-tool-update-shell--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn tool dir](#fyn-tool-dir)

Show the path to the fyn tools directory.

The tools directory is used to store environments and metadata for installed tools.

By default, tools are stored in the fyn data directory at `$XDG_DATA_HOME/fyn/tools` or `$HOME/.local/share/fyn/tools` on Unix and `%APPDATA%\fyn\data\tools` on Windows.

The tool installation directory may be overridden with `$UV_TOOL_DIR`.

To instead view the directory fyn installs executables into, use the `--bin` flag.

### Usage

```
fyn tool dir [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-tool-dir--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--bin`](#fyn-tool-dir--bin) : Show the directory into which `fyn tool` will install executables.

```
By default, `fyn tool dir` shows the directory into which the tool Python environments
themselves are installed, rather than the directory containing the linked executables.

The tool executable directory is determined according to the XDG standard and is derived
from the following environment variables, in order of preference:

- `$UV_TOOL_BIN_DIR`
- `$XDG_BIN_HOME`
- `$XDG_DATA_HOME/../bin`
- `$HOME/.local/bin`
```

[`--cache-dir`](#fyn-tool-dir--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-tool-dir--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-tool-dir--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-tool-dir--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-tool-dir--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-tool-dir--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-tool-dir--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-tool-dir--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-tool-dir--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-tool-dir--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-tool-dir--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-tool-dir--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-tool-dir--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-tool-dir--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-tool-dir--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-tool-dir--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn python](#fyn-python)

Manage Python versions and installations

Generally, fyn first searches for Python in a virtual environment, either active or in a `.venv` directory in the current working directory or any parent directory. If a virtual environment is not required, fyn will then search for a Python interpreter. Python interpreters are found by searching for Python executables in the `PATH` environment variable.

On Windows, the registry is also searched for Python executables.

By default, fyn will download Python if a version cannot be found. This behavior can be disabled with the `--no-python-downloads` flag or the `python-downloads` setting.

The `--python` option allows requesting a different interpreter.

The following Python version request formats are supported:

- `<version>` e.g. `3`, `3.12`, `3.12.3`
- `<version-specifier>` e.g. `>=3.12,<3.13`
- `<version><short-variant>` (e.g., `3.13t`, `3.12.0d`)
- `<version>+<variant>` (e.g., `3.13+freethreaded`, `3.12.0+debug`)
- `<implementation>` e.g. `cpython` or `cp`
- `<implementation>@<version>` e.g. `cpython@3.12`
- `<implementation><version>` e.g. `cpython3.12` or `cp312`
- `<implementation><version-specifier>` e.g. `cpython>=3.12,<3.13`
- `<implementation>-<version>-<os>-<arch>-<libc>` e.g. `cpython-3.12.3-macos-aarch64-none`

Additionally, a specific system Python interpreter can often be requested with:

- `<executable-path>` e.g. `/opt/homebrew/bin/python3`
- `<executable-name>` e.g. `mypython3`
- `<install-dir>` e.g. `/some/environment/`

When the `--python` option is used, normal discovery rules apply but discovered interpreters are checked for compatibility with the request, e.g., if `pypy` is requested, fyn will first check if the virtual environment contains a PyPy interpreter then check if each executable in the path is a PyPy interpreter.

fyn supports discovering CPython, PyPy, and GraalPy interpreters. Unsupported interpreters will be skipped during discovery. If an unsupported interpreter implementation is requested, fyn will exit with an error.

### Usage

```
fyn python [OPTIONS] <COMMAND>
```

### Commands

[`fyn python list`](#fyn-python-list) : List the available Python installations

[`fyn python install`](#fyn-python-install) : Download and install Python versions

[`fyn python upgrade`](#fyn-python-upgrade) : Upgrade installed Python versions

[`fyn python find`](#fyn-python-find) : Search for a Python installation

[`fyn python pin`](#fyn-python-pin) : Pin to a specific Python version

[`fyn python dir`](#fyn-python-dir) : Show the fyn Python installation directory

[`fyn python uninstall`](#fyn-python-uninstall) : Uninstall Python versions

[`fyn python install-shim`](#fyn-python-install-shim) : Install a `python` shim into the Python executable directory

[`fyn python update-shell`](#fyn-python-update-shell) : Ensure that the Python executable directory is on the `PATH`

### [fyn python list](#fyn-python-list)

List the available Python installations.

By default, installed Python versions and the downloads for latest available patch version of each supported Python major version are shown.

Use `--managed-python` to view only managed Python versions.

Use `--no-managed-python` to omit managed Python versions.

Use `--all-versions` to view all available patch versions.

Use `--only-installed` to omit available downloads.

### Usage

```
fyn python list [OPTIONS] [REQUEST]
```

### Arguments

[REQUEST](#fyn-python-list--request) : A Python request to filter by.

```
See [fyn python](#fyn-python) to view supported request formats.
```

### Options

[`--all-arches`](#fyn-python-list--all-arches), `--all_architectures` : List Python downloads for all architectures.

```
By default, only downloads for the current architecture are shown.
```

[`--all-platforms`](#fyn-python-list--all-platforms) : List Python downloads for all platforms.

```
By default, only downloads for the current platform are shown.
```

[`--all-versions`](#fyn-python-list--all-versions) : List all Python versions, including old patch versions.

```
By default, only the latest patch version is shown for each minor version.
```

[`--allow-insecure-host`](#fyn-python-list--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-python-list--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-python-list--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-python-list--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-python-list--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-python-list--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-python-list--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-python-list--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-python-list--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-python-list--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-python-list--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-python-list--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-python-list--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-python-list--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-downloads`](#fyn-python-list--only-downloads) : Only show available Python downloads.

```
By default, installed distributions and available downloads for the current platform are shown.
```

[`--only-installed`](#fyn-python-list--only-installed) : Only show installed Python versions.

```
By default, installed distributions and available downloads for the current platform are shown.
```

[`--output-format`](#fyn-python-list--output-format) *output-format* : Select the output format

```
[default: text]

Possible values:

- `text`: Plain text (for humans)
- `json`: JSON (for computers)
```

[`--project`](#fyn-python-list--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python-downloads-json-url`](#fyn-python-list--python-downloads-json-url) *python-downloads-json-url* : URL pointing to JSON of custom Python installations

[`--quiet`](#fyn-python-list--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--show-urls`](#fyn-python-list--show-urls) : Show the URLs of available Python downloads.

```
By default, these display as `<download available>`.
```

[`--verbose`](#fyn-python-list--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn python install](#fyn-python-install)

Download and install Python versions.

Supports CPython and PyPy. CPython distributions are downloaded from the Astral `python-build-standalone` project. PyPy distributions are downloaded from `python.org`. The available Python versions are bundled with each fyn release. To install new Python versions, you may need upgrade fyn.

Python versions are installed into the fyn Python directory, which can be retrieved with `fyn python dir`.

By default, Python executables are added to a directory on the path with a minor version suffix, e.g., `python3.13`. To install `python3` and `python`, use the `--default` flag. Use `fyn python dir --bin` to see the target directory.

Multiple Python versions may be requested.

See `fyn help python` to view supported request formats.

### Usage

```
fyn python install [OPTIONS] [TARGETS]...
```

### Arguments

[TARGETS](#fyn-python-install--targets) : The Python version(s) to install.

```
If not provided, the requested Python version(s) will be read from the `UV_PYTHON` environment variable then `.python-versions` or `.python-version` files. If none of the above are present, fyn will check if it has installed any Python versions. If not, it will install the latest stable version of Python.

See [fyn python](#fyn-python) to view supported request formats.
```

### Options

[`--allow-insecure-host`](#fyn-python-install--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-python-install--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-python-install--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-python-install--compile-bytecode), `--compile` : Compile Python's standard library to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is important, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times and some additional disk space for faster start times.

When enabled, fyn will process the Python version's `stdlib` directory. It will ignore any compilation errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-python-install--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--default`](#fyn-python-install--default) : Use as the default Python version.

```
By default, only a `python{major}.{minor}` executable is installed, e.g., `python3.10`. When the `--default` flag is used, `python{major}`, e.g., `python3`, and `python` executables are also installed.

Alternative Python variants will still include their tag. For example, installing 3.13+freethreaded with `--default` will include `python3t` and `pythont` instead of `python3` and `python`.

If multiple Python versions are requested, fyn will exit with an error.
```

[`--directory`](#fyn-python-install--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--force`](#fyn-python-install--force), `-f` : Replace existing Python executables during installation.

```
By default, fyn will refuse to replace executables that it does not manage.

Implies `--reinstall`.
```

[`--help`](#fyn-python-install--help), `-h` : Display the concise help for this command

[`--install-dir`](#fyn-python-install--install-dir), `-i` *install-dir* : The directory to store the Python installation in.

```
If provided, `UV_PYTHON_INSTALL_DIR` will need to be set for subsequent operations for fyn to discover the Python installation.

See `fyn python dir` to view the current Python installation directory. Defaults to `~/.local/share/fyn/python`.

May also be set with the `UV_PYTHON_INSTALL_DIR` environment variable.
```

[`--managed-python`](#fyn-python-install--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--mirror`](#fyn-python-install--mirror) *mirror* : Set the URL to use as the source for downloading Python installations.

```
The provided URL will replace `https://github.com/astral-sh/python-build-standalone/releases/download` in, e.g., `https://github.com/astral-sh/python-build-standalone/releases/download/20240713/cpython-3.12.4%2B20240713-aarch64-apple-darwin-install_only.tar.gz`.

Distributions can be read from a local directory by using the `file://` URL scheme.
```

[`--native-tls`](#fyn-python-install--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-bin`](#fyn-python-install--no-bin) : Do not install a Python executable into the `bin` directory.

```
This can also be set with `UV_PYTHON_INSTALL_BIN=0`.
```

[`--no-cache`](#fyn-python-install--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-python-install--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-python-install--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-python-install--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-python-install--no-python-downloads) : Disable automatic downloads of Python.

[`--no-registry`](#fyn-python-install--no-registry) : Do not register the Python installation in the Windows registry.

```
This can also be set with `UV_PYTHON_INSTALL_REGISTRY=0`.
```

[`--offline`](#fyn-python-install--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-python-install--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--pypy-mirror`](#fyn-python-install--pypy-mirror) *pypy-mirror* : Set the URL to use as the source for downloading PyPy installations.

```
The provided URL will replace `https://downloads.python.org/pypy` in, e.g., `https://downloads.python.org/pypy/pypy3.8-v7.3.7-osx64.tar.bz2`.

Distributions can be read from a local directory by using the `file://` URL scheme.
```

[`--python-downloads-json-url`](#fyn-python-install--python-downloads-json-url) *python-downloads-json-url* : URL pointing to JSON of custom Python installations

[`--quiet`](#fyn-python-install--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--reinstall`](#fyn-python-install--reinstall), `-r` : Reinstall the requested Python version, if it's already installed.

```
By default, fyn will exit successfully if the version is already installed.
```

[`--upgrade`](#fyn-python-install--upgrade), `-U` : Upgrade existing Python installations to the latest patch version.

```
By default, fyn will not upgrade already-installed Python versions to newer patch releases. With `--upgrade`, fyn will upgrade to the latest available patch version for the specified minor version(s).

If the requested versions are not yet installed, fyn will install them.

This option is only supported for minor version requests, e.g., `3.12`; fyn will exit with an error if a patch version, e.g., `3.12.2`, is requested.
```

[`--verbose`](#fyn-python-install--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn python upgrade](#fyn-python-upgrade)

Upgrade installed Python versions.

Upgrades versions to the latest supported patch release. Requires the `python-upgrade` preview feature.

A target Python minor version to upgrade may be provided, e.g., `3.13`. Multiple versions may be provided to perform more than one upgrade.

If no target version is provided, then fyn will upgrade all managed CPython versions.

During an upgrade, fyn will not uninstall outdated patch versions.

When an upgrade is performed, virtual environments created by fyn will automatically use the new version. However, if the virtual environment was created before the upgrade functionality was added, it will continue to use the old Python version; to enable upgrades, the environment must be recreated.

Upgrades are not yet supported for alternative implementations, like PyPy.

### Usage

```
fyn python upgrade [OPTIONS] [TARGETS]...
```

### Arguments

[TARGETS](#fyn-python-upgrade--targets) : The Python minor version(s) to upgrade.

```
If no target version is provided, then fyn will upgrade all managed CPython versions.
```

### Options

[`--allow-insecure-host`](#fyn-python-upgrade--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-python-upgrade--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-python-upgrade--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-python-upgrade--compile-bytecode), `--compile` : Compile Python's standard library to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is important, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times and some additional disk space for faster start times.

When enabled, fyn will process the Python version's `stdlib` directory. It will ignore any compilation errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-python-upgrade--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-python-upgrade--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-python-upgrade--help), `-h` : Display the concise help for this command

[`--install-dir`](#fyn-python-upgrade--install-dir), `-i` *install-dir* : The directory Python installations are stored in.

```
If provided, `UV_PYTHON_INSTALL_DIR` will need to be set for subsequent operations for fyn to discover the Python installation.

See `fyn python dir` to view the current Python installation directory. Defaults to `~/.local/share/fyn/python`.

May also be set with the `UV_PYTHON_INSTALL_DIR` environment variable.
```

[`--managed-python`](#fyn-python-upgrade--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--mirror`](#fyn-python-upgrade--mirror) *mirror* : Set the URL to use as the source for downloading Python installations.

```
The provided URL will replace `https://github.com/astral-sh/python-build-standalone/releases/download` in, e.g., `https://github.com/astral-sh/python-build-standalone/releases/download/20240713/cpython-3.12.4%2B20240713-aarch64-apple-darwin-install_only.tar.gz`.

Distributions can be read from a local directory by using the `file://` URL scheme.
```

[`--native-tls`](#fyn-python-upgrade--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-python-upgrade--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-python-upgrade--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-python-upgrade--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-python-upgrade--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-python-upgrade--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-python-upgrade--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-python-upgrade--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--pypy-mirror`](#fyn-python-upgrade--pypy-mirror) *pypy-mirror* : Set the URL to use as the source for downloading PyPy installations.

```
The provided URL will replace `https://downloads.python.org/pypy` in, e.g., `https://downloads.python.org/pypy/pypy3.8-v7.3.7-osx64.tar.bz2`.

Distributions can be read from a local directory by using the `file://` URL scheme.
```

[`--python-downloads-json-url`](#fyn-python-upgrade--python-downloads-json-url) *python-downloads-json-url* : URL pointing to JSON of custom Python installations

[`--quiet`](#fyn-python-upgrade--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--reinstall`](#fyn-python-upgrade--reinstall), `-r` : Reinstall the latest Python patch, if it's already installed.

```
By default, fyn will exit successfully if the latest patch is already installed.
```

[`--verbose`](#fyn-python-upgrade--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn python find](#fyn-python-find)

Search for a Python installation.

Displays the path to the Python executable.

See `fyn help python` to view supported request formats and details on discovery behavior.

### Usage

```
fyn python find [OPTIONS] [REQUEST]
```

### Arguments

[REQUEST](#fyn-python-find--request) : The Python request.

```
See [fyn python](#fyn-python) to view supported request formats.
```

### Options

[`--allow-insecure-host`](#fyn-python-find--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-python-find--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-python-find--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-python-find--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-python-find--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-python-find--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-python-find--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-python-find--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-python-find--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-python-find--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-python-find--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-python-find--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-project`](#fyn-python-find--no-project), `--no_workspace` : Avoid discovering a project or workspace.

```
Otherwise, when no request is provided, the Python requirement of a project in the current directory or parent directories will be used.
```

[`--no-python-downloads`](#fyn-python-find--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-python-find--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-python-find--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python-downloads-json-url`](#fyn-python-find--python-downloads-json-url) *python-downloads-json-url* : URL pointing to JSON of custom Python installations

[`--quiet`](#fyn-python-find--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--resolve-links`](#fyn-python-find--resolve-links) : Resolve symlinks in the output path.

```
When enabled, the output path will be canonicalized, resolving any symlinks.
```

[`--script`](#fyn-python-find--script) *script* : Find the environment for a Python script, rather than the current project

[`--show-version`](#fyn-python-find--show-version) : Show the Python version that would be used instead of the path to the interpreter

[`--system`](#fyn-python-find--system) : Only find system Python interpreters.

```
By default, fyn will report the first Python interpreter it would use, including those in an active virtual environment or a virtual environment in the current working directory or any parent directory.

The `--system` option instructs fyn to skip virtual environment Python interpreters and restrict its search to the system path.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--verbose`](#fyn-python-find--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn python pin](#fyn-python-pin)

Pin to a specific Python version.

Writes the pinned Python version to a `.python-version` file, which is used by other fyn commands to determine the required Python version.

If no version is provided, fyn will look for an existing `.python-version` file and display the currently pinned version. If no `.python-version` file is found, fyn will exit with an error.

See `fyn help python` to view supported request formats.

### Usage

```
fyn python pin [OPTIONS] [REQUEST]
```

### Arguments

[REQUEST](#fyn-python-pin--request) : The Python version request.

```
fyn supports more formats than other tools that read `.python-version` files, i.e., `pyenv`. If compatibility with those tools is needed, only use version numbers instead of complex requests such as `cpython@3.10`.

If no request is provided, the currently pinned version will be shown.

See [fyn python](#fyn-python) to view supported request formats.
```

### Options

[`--allow-insecure-host`](#fyn-python-pin--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-python-pin--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-python-pin--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-python-pin--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-python-pin--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--global`](#fyn-python-pin--global) : Update the global Python version pin.

```
Writes the pinned Python version to a `.python-version` file in the fyn user configuration directory: `XDG_CONFIG_HOME/fyn` on Linux/macOS and `%APPDATA%/fyn` on Windows.

When a local Python version pin is not found in the working directory or an ancestor directory, this version will be used instead.
```

[`--help`](#fyn-python-pin--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-python-pin--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-python-pin--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-python-pin--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-python-pin--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-python-pin--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-python-pin--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-project`](#fyn-python-pin--no-project), `--no-workspace` : Avoid validating the Python pin is compatible with the project or workspace.

```
By default, a project or workspace is discovered in the current directory or any parent directory. If a workspace is found, the Python pin is validated against the workspace's `requires-python` constraint.
```

[`--no-python-downloads`](#fyn-python-pin--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-python-pin--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-python-pin--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python-downloads-json-url`](#fyn-python-pin--python-downloads-json-url) *python-downloads-json-url* : URL pointing to JSON of custom Python installations

[`--quiet`](#fyn-python-pin--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--resolved`](#fyn-python-pin--resolved) : Write the resolved Python interpreter path instead of the request.

```
Ensures that the exact same interpreter is used.

This option is usually not safe to use when committing the `.python-version` file to version control.
```

[`--rm`](#fyn-python-pin--rm) : Remove the Python version pin

[`--upgrade`](#fyn-python-pin--upgrade), `-U` : Upgrade the Python pin to the latest compatible exact version.

```
If a request is provided, fyn upgrades that request. Otherwise, fyn upgrades the first entry from the discovered `.python-version` or `.python-versions` file.

When a project or workspace is discovered, the selected version must also satisfy its `requires-python` constraint.
```

[`--verbose`](#fyn-python-pin--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn python dir](#fyn-python-dir)

Show the fyn Python installation directory.

By default, Python installations are stored in the fyn data directory at `$XDG_DATA_HOME/fyn/python` or `$HOME/.local/share/fyn/python` on Unix and `%APPDATA%\fyn\data\python` on Windows.

The Python installation directory may be overridden with `$UV_PYTHON_INSTALL_DIR`.

To view the directory where fyn installs Python executables instead, use the `--bin` flag. The Python executable directory may be overridden with `$UV_PYTHON_BIN_DIR`. Note that Python executables are only installed when preview mode is enabled.

### Usage

```
fyn python dir [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-python-dir--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--bin`](#fyn-python-dir--bin) : Show the directory into which `fyn python` will install Python executables.

```
Note that this directory is only used when installing Python with preview mode enabled.

The Python executable directory is determined according to the XDG standard and is derived
from the following environment variables, in order of preference:

- `$UV_PYTHON_BIN_DIR`
- `$XDG_BIN_HOME`
- `$XDG_DATA_HOME/../bin`
- `$HOME/.local/bin`
```

[`--cache-dir`](#fyn-python-dir--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-python-dir--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-python-dir--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-python-dir--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-python-dir--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-python-dir--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-python-dir--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-python-dir--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-python-dir--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-python-dir--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-python-dir--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-python-dir--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-python-dir--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-python-dir--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-python-dir--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-python-dir--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn python uninstall](#fyn-python-uninstall)

Uninstall Python versions

### Usage

```
fyn python uninstall [OPTIONS] <TARGETS>...
```

### Arguments

[TARGETS](#fyn-python-uninstall--targets) : The Python version(s) to uninstall.

```
See [fyn python](#fyn-python) to view supported request formats.
```

### Options

[`--all`](#fyn-python-uninstall--all) : Uninstall all managed Python versions

[`--allow-insecure-host`](#fyn-python-uninstall--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-python-uninstall--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-python-uninstall--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-python-uninstall--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-python-uninstall--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-python-uninstall--help), `-h` : Display the concise help for this command

[`--install-dir`](#fyn-python-uninstall--install-dir), `-i` *install-dir* : The directory where the Python was installed

```
May also be set with the `UV_PYTHON_INSTALL_DIR` environment variable.
```

[`--managed-python`](#fyn-python-uninstall--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-python-uninstall--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-python-uninstall--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-python-uninstall--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-python-uninstall--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-python-uninstall--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-python-uninstall--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-python-uninstall--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-python-uninstall--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-python-uninstall--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-python-uninstall--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn python install-shim](#fyn-python-install-shim)

Install a `python` shim into the Python executable directory.

The shim resolves the appropriate interpreter for the current directory before executing it, which makes a plain `python` command available on your shell `PATH` without pinning it to a single managed installation.

### Usage

```
fyn python install-shim [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-python-install-shim--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-python-install-shim--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-python-install-shim--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-python-install-shim--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-python-install-shim--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--force`](#fyn-python-install-shim--force), `-f` : Replace an existing shim target.

```
By default, fyn refuses to replace an existing `python` executable unless it already matches the expected shim contents.
```

[`--help`](#fyn-python-install-shim--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-python-install-shim--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-python-install-shim--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-python-install-shim--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-python-install-shim--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-python-install-shim--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-python-install-shim--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-python-install-shim--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-python-install-shim--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-python-install-shim--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-python-install-shim--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-python-install-shim--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn python update-shell](#fyn-python-update-shell)

Ensure that the Python executable directory is on the `PATH`.

If the Python executable directory is not present on the `PATH`, fyn will attempt to add it to the relevant shell configuration files.

If the shell configuration files already include a blurb to add the executable directory to the path, but the directory is not present on the `PATH`, fyn will exit with an error.

The Python executable directory is determined according to the XDG standard and can be retrieved with `fyn python dir --bin`.

### Usage

```
fyn python update-shell [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-python-update-shell--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-python-update-shell--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-python-update-shell--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-python-update-shell--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-python-update-shell--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-python-update-shell--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-python-update-shell--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-python-update-shell--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-python-update-shell--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-python-update-shell--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-python-update-shell--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-python-update-shell--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-python-update-shell--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-python-update-shell--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-python-update-shell--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-python-update-shell--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-python-update-shell--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn pip](#fyn-pip)

Manage Python packages directly in an environment

### Usage

```
fyn pip [OPTIONS] <COMMAND>
```

### Commands

[`fyn pip compile`](#fyn-pip-compile) : Compile a `requirements.in` file to a `requirements.txt` or `pylock.toml` file

[`fyn pip sync`](#fyn-pip-sync) : Sync an environment with a `requirements.txt` or `pylock.toml` file

[`fyn pip install`](#fyn-pip-install) : Install packages into an environment

[`fyn pip download`](#fyn-pip-download) : Download distribution archives into a directory

[`fyn pip wheel`](#fyn-pip-wheel) : Build wheels into a directory

[`fyn pip index`](#fyn-pip-index) : Inspect package indexes

[`fyn pip upgrade`](#fyn-pip-upgrade) : Upgrade packages in an environment

[`fyn pip uninstall`](#fyn-pip-uninstall) : Uninstall packages from an environment

[`fyn pip freeze`](#fyn-pip-freeze) : List, in requirements format, packages installed in an environment

[`fyn pip list`](#fyn-pip-list) : List, in tabular format, packages installed in an environment

[`fyn pip show`](#fyn-pip-show) : Show information about one or more installed packages

[`fyn pip tree`](#fyn-pip-tree) : Display the dependency tree for an environment

[`fyn pip check`](#fyn-pip-check) : Verify installed packages have compatible dependencies

### [fyn pip compile](#fyn-pip-compile)

Compile a `requirements.in` file to a `requirements.txt` or `pylock.toml` file

### Usage

```
fyn pip compile [OPTIONS] <SRC_FILE|--group <GROUP>>
```

### Arguments

[SRC_FILE](#fyn-pip-compile--src_file) : Include the packages listed in the given files.

```
The following formats are supported: `requirements.txt`, `.py` files with inline metadata, `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg`.

If a `pyproject.toml`, `setup.py`, or `setup.cfg` file is provided, fyn will extract the requirements for the relevant project.

If `-` is provided, then requirements will be read from stdin.

The order of the requirements files and the requirements in them is used to determine priority during resolution.
```

### Options

[`--all-extras`](#fyn-pip-compile--all-extras) : Include all optional dependencies.

```
Only applies to `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
```

[`--allow-insecure-host`](#fyn-pip-compile--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--annotation-style`](#fyn-pip-compile--annotation-style) *annotation-style* : The style of the annotation comments included in the output file, used to indicate the source of each package.

```
Defaults to `split`.

Possible values:

- `line`: Render the annotations on a single, comma-separated line
- `split`: Render each annotation on its own line
```

[`--build-constraints`](#fyn-pip-compile--build-constraints), `--build-constraint`, `-b` *build-constraints* : Constrain build dependencies using the given requirements files when building source distributions.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

May also be set with the `UV_BUILD_CONSTRAINT` environment variable.
```

[`--cache-dir`](#fyn-pip-compile--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-compile--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-pip-compile--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-pip-compile--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-pip-compile--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--constraints`](#fyn-pip-compile--constraints), `--constraint`, `-c` *constraints* : Constrain versions using the given requirements files.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

This is equivalent to pip's `--constraint` option.

May also be set with the `UV_CONSTRAINT` environment variable.
```

[`--custom-compile-command`](#fyn-pip-compile--custom-compile-command) *custom-compile-command* : The header comment to include at the top of the output file generated by `fyn pip compile`.

```
Used to reflect custom build scripts and commands that wrap `fyn pip compile`.

May also be set with the `UV_CUSTOM_COMPILE_COMMAND` environment variable.
```

[`--default-index`](#fyn-pip-compile--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-pip-compile--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--emit-build-options`](#fyn-pip-compile--emit-build-options) : Include `--no-binary` and `--only-binary` entries in the generated output file

[`--emit-find-links`](#fyn-pip-compile--emit-find-links) : Include `--find-links` entries in the generated output file

[`--emit-index-annotation`](#fyn-pip-compile--emit-index-annotation) : Include comment annotations indicating the index used to resolve each package (e.g., `# from https://pypi.org/simple`)

[`--emit-index-url`](#fyn-pip-compile--emit-index-url) : Include `--index-url` and `--extra-index-url` entries in the generated output file

[`--exclude-newer`](#fyn-pip-compile--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-pip-compile--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--excludes`](#fyn-pip-compile--excludes), `--exclude` *excludes* : Exclude packages from resolution using the given requirements files.

```
Excludes files are `requirements.txt`-like files that specify packages to exclude from the resolution. When a package is excluded, it will be omitted from the dependency list entirely and its own dependencies will be ignored during the resolution phase. Excludes are unconditional in that requirement specifiers and markers are ignored; any package listed in the provided file will be omitted from all resolved environments.

May also be set with the `UV_EXCLUDE` environment variable.
```

[`--extra`](#fyn-pip-compile--extra) *extra* : Include optional dependencies from the specified extra name; may be provided more than once.

```
Only applies to `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
```

[`--extra-index-url`](#fyn-pip-compile--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-pip-compile--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-pip-compile--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--format`](#fyn-pip-compile--format) *format* : The format in which the resolution should be output.

```
Supports both `requirements.txt` and `pylock.toml` (PEP 751) output formats.

fyn will infer the output format from the file extension of the output file, if provided. Otherwise, defaults to `requirements.txt`.

Possible values:

- `requirements.txt`: Export in `requirements.txt` format
- `pylock.toml`: Export in `pylock.toml` format
```

[`--generate-hashes`](#fyn-pip-compile--generate-hashes) : Include distribution hashes in the output file

[`--group`](#fyn-pip-compile--group) *group* : Install the specified dependency group from a `pyproject.toml`.

```
If no path is provided, the `pyproject.toml` in the working directory is used.

May be provided multiple times.
```

[`--help`](#fyn-pip-compile--help), `-h` : Display the concise help for this command

[`--index`](#fyn-pip-compile--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-pip-compile--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-pip-compile--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-pip-compile--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-pip-compile--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
This option is only used when building source distributions.

Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-pip-compile--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-compile--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-annotate`](#fyn-pip-compile--no-annotate) : Exclude comment annotations indicating the source of each package

[`--no-binary`](#fyn-pip-compile--no-binary) *no-binary* : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--no-build`](#fyn-pip-compile--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

Alias for `--only-binary :all:`.
```

[`--no-build-isolation`](#fyn-pip-compile--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-pip-compile--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-cache`](#fyn-pip-compile--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-deps`](#fyn-pip-compile--no-deps) : Ignore package dependencies, instead only add those packages explicitly listed on the command line to the resulting requirements file

[`--no-emit-package`](#fyn-pip-compile--no-emit-package), `--unsafe-package` *no-emit-package* : Specify a package to omit from the output resolution. Its dependencies will still be included in the resolution. Equivalent to pip-compile's `--unsafe-package` option

[`--no-header`](#fyn-pip-compile--no-header) : Exclude the comment header at the top of the generated output file

[`--no-index`](#fyn-pip-compile--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-pip-compile--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-compile--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-compile--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-pip-compile--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-pip-compile--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--no-strip-extras`](#fyn-pip-compile--no-strip-extras) : Include extras in the output file.

```
By default, fyn strips extras, as any packages pulled in by the extras are already included as dependencies in the output file directly. Further, output files generated with `--no-strip-extras` cannot be used as constraints files in `install` and `sync` invocations.
```

[`--no-strip-markers`](#fyn-pip-compile--no-strip-markers) : Include environment markers in the output file.

```
By default, fyn strips environment markers, as the resolution generated by `compile` is only guaranteed to be correct for the target environment.
```

[`--offline`](#fyn-pip-compile--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-binary`](#fyn-pip-compile--only-binary) *only-binary* : Only use pre-built wheels; don't build source distributions.

```
When enabled, resolving will not run code from the given packages. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--output-file`](#fyn-pip-compile--output-file), `-o` *output-file* : Write the compiled requirements to the given `requirements.txt` or `pylock.toml` file.

```
If the file already exists, the existing versions will be preferred when resolving dependencies, unless `--upgrade` is also specified.
```

[`--overrides`](#fyn-pip-compile--overrides), `--override` *overrides* : Override versions using the given requirements files.

```
Overrides files are `requirements.txt`-like files that force a specific version of a requirement to be installed, regardless of the requirements declared by any constituent package, and regardless of whether this would be considered an invalid resolution.

While constraints are *additive*, in that they're combined with the requirements of the constituent packages, overrides are *absolute*, in that they completely replace the requirements of the constituent packages.

May also be set with the `UV_OVERRIDE` environment variable.
```

[`--prerelease`](#fyn-pip-compile--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-pip-compile--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-compile--python), `-p` *python* : The Python interpreter to use during resolution.

```
A Python interpreter is required for building source distributions to determine package
metadata when there are not wheels.

The interpreter is also used to determine the default minimum Python version, unless
`--python-version` is provided.

This option respects `UV_PYTHON`, but when set via environment variable, it is overridden
by `--python-version`.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.
```

[`--python-platform`](#fyn-pip-compile--python-platform) *python-platform* : The platform for which requirements should be resolved.

```
Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

When targeting macOS (Darwin), the default minimum version is `13.0`. Use `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting iOS, the default minimum version is `13.0`. Use `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting Android, the default minimum Android API level is `24`. Use `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--python-version`](#fyn-pip-compile--python-version) *python-version* : The Python version to use for resolution.

```
For example, `3.8` or `3.8.17`.

Defaults to the version of the Python interpreter used for resolution.

Defines the minimum Python version that must be supported by the resolved requirements.

If a patch version is omitted, the minimum patch version is assumed. For example, `3.8` is mapped to `3.8.0`.
```

[`--quiet`](#fyn-pip-compile--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-pip-compile--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-pip-compile--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--resolution`](#fyn-pip-compile--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--system`](#fyn-pip-compile--system) : Install packages into the system Python environment.

```
By default, fyn uses the virtual environment in the current working directory or any parent directory, falling back to searching for a Python executable in `PATH`. The `--system` option instructs fyn to avoid using a virtual environment Python and restrict its search to the system path.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--torch-backend`](#fyn-pip-compile--torch-backend) *torch-backend* : The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`).

```
When set, fyn will ignore the configured index URLs for packages in the PyTorch ecosystem, and will instead use the defined backend.

For example, when set to `cpu`, fyn will use the CPU-only PyTorch index; when set to `cu126`, fyn will use the PyTorch index for CUDA 12.6.

The `auto` mode will attempt to detect the appropriate PyTorch index based on the currently installed CUDA drivers.

This option is in preview and may change in any future release.

May also be set with the `UV_TORCH_BACKEND` environment variable.

Possible values:

- `auto`: Select the appropriate PyTorch index based on the operating system and CUDA driver version
- `cpu`: Use the CPU-only PyTorch index
- `cu130`: Use the PyTorch index for CUDA 13.0
- `cu129`: Use the PyTorch index for CUDA 12.9
- `cu128`: Use the PyTorch index for CUDA 12.8
- `cu126`: Use the PyTorch index for CUDA 12.6
- `cu125`: Use the PyTorch index for CUDA 12.5
- `cu124`: Use the PyTorch index for CUDA 12.4
- `cu123`: Use the PyTorch index for CUDA 12.3
- `cu122`: Use the PyTorch index for CUDA 12.2
- `cu121`: Use the PyTorch index for CUDA 12.1
- `cu120`: Use the PyTorch index for CUDA 12.0
- `cu118`: Use the PyTorch index for CUDA 11.8
- `cu117`: Use the PyTorch index for CUDA 11.7
- `cu116`: Use the PyTorch index for CUDA 11.6
- `cu115`: Use the PyTorch index for CUDA 11.5
- `cu114`: Use the PyTorch index for CUDA 11.4
- `cu113`: Use the PyTorch index for CUDA 11.3
- `cu112`: Use the PyTorch index for CUDA 11.2
- `cu111`: Use the PyTorch index for CUDA 11.1
- `cu110`: Use the PyTorch index for CUDA 11.0
- `cu102`: Use the PyTorch index for CUDA 10.2
- `cu101`: Use the PyTorch index for CUDA 10.1
- `cu100`: Use the PyTorch index for CUDA 10.0
- `cu92`: Use the PyTorch index for CUDA 9.2
- `cu91`: Use the PyTorch index for CUDA 9.1
- `cu90`: Use the PyTorch index for CUDA 9.0
- `cu80`: Use the PyTorch index for CUDA 8.0
- `rocm7.1`: Use the PyTorch index for ROCm 7.1
- `rocm7.0`: Use the PyTorch index for ROCm 7.0
- `rocm6.4`: Use the PyTorch index for ROCm 6.4
- `rocm6.3`: Use the PyTorch index for ROCm 6.3
- `rocm6.2.4`: Use the PyTorch index for ROCm 6.2.4
- `rocm6.2`: Use the PyTorch index for ROCm 6.2
- `rocm6.1`: Use the PyTorch index for ROCm 6.1
- `rocm6.0`: Use the PyTorch index for ROCm 6.0
- `rocm5.7`: Use the PyTorch index for ROCm 5.7
- `rocm5.6`: Use the PyTorch index for ROCm 5.6
- `rocm5.5`: Use the PyTorch index for ROCm 5.5
- `rocm5.4.2`: Use the PyTorch index for ROCm 5.4.2
- `rocm5.4`: Use the PyTorch index for ROCm 5.4
- `rocm5.3`: Use the PyTorch index for ROCm 5.3
- `rocm5.2`: Use the PyTorch index for ROCm 5.2
- `rocm5.1.1`: Use the PyTorch index for ROCm 5.1.1
- `rocm4.2`: Use the PyTorch index for ROCm 4.2
- `rocm4.1`: Use the PyTorch index for ROCm 4.1
- `rocm4.0.1`: Use the PyTorch index for ROCm 4.0.1
- `xpu`: Use the PyTorch index for Intel XPU
```

[`--universal`](#fyn-pip-compile--universal) : Perform a universal resolution, attempting to generate a single `requirements.txt` output file that is compatible with all operating systems, architectures, and Python implementations.

```
In universal mode, the current Python version (or user-provided `--python-version`) will be treated as a lower bound. For example, `--universal --python-version 3.7` would produce a universal resolution for Python 3.7 and later.

Implies `--no-strip-markers`.
```

[`--upgrade`](#fyn-pip-compile--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-pip-compile--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-pip-compile--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn pip sync](#fyn-pip-sync)

Sync an environment with a `requirements.txt` or `pylock.toml` file.

When syncing an environment, any packages not listed in the `requirements.txt` or `pylock.toml` file will be removed. To retain extraneous packages, use `fyn pip install` instead.

The input file is presumed to be the output of a `pip compile` or `fyn export` operation, in which it will include all transitive dependencies. If transitive dependencies are not present in the file, they will not be installed. Use `--strict` to warn if any transitive dependencies are missing.

### Usage

```
fyn pip sync [OPTIONS] <SRC_FILE>...
```

### Arguments

[SRC_FILE](#fyn-pip-sync--src_file) : Include the packages listed in the given files.

```
The following formats are supported: `requirements.txt`, `.py` files with inline metadata, `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg`.

If a `pyproject.toml`, `setup.py`, or `setup.cfg` file is provided, fyn will extract the requirements for the relevant project.

If `-` is provided, then requirements will be read from stdin.
```

### Options

[`--all-extras`](#fyn-pip-sync--all-extras) : Include all optional dependencies.

```
Only applies to `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
```

[`--allow-empty-requirements`](#fyn-pip-sync--allow-empty-requirements) : Allow sync of empty requirements, which will clear the environment of all packages

[`--allow-insecure-host`](#fyn-pip-sync--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--break-system-packages`](#fyn-pip-sync--break-system-packages) : Allow fyn to modify an `EXTERNALLY-MANAGED` Python installation.

```
WARNING: `--break-system-packages` is intended for use in continuous integration (CI) environments, when installing into Python installations that are managed by an external package manager, like `apt`. It should be used with caution, as such Python installations explicitly recommend against modifications by other package managers (like fyn or `pip`).

May also be set with the `UV_BREAK_SYSTEM_PACKAGES` environment variable.
```

[`--build-constraints`](#fyn-pip-sync--build-constraints), `--build-constraint`, `-b` *build-constraints* : Constrain build dependencies using the given requirements files when building source distributions.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

May also be set with the `UV_BUILD_CONSTRAINT` environment variable.
```

[`--cache-dir`](#fyn-pip-sync--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-sync--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-pip-sync--compile-bytecode), `--compile` : Compile Python files to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-pip-sync--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-pip-sync--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-pip-sync--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--constraints`](#fyn-pip-sync--constraints), `--constraint`, `-c` *constraints* : Constrain versions using the given requirements files.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

This is equivalent to pip's `--constraint` option.

May also be set with the `UV_CONSTRAINT` environment variable.
```

[`--default-index`](#fyn-pip-sync--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-pip-sync--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--dry-run`](#fyn-pip-sync--dry-run) : Perform a dry run, i.e., don't actually install anything but resolve the dependencies and print the resulting plan

[`--exclude-newer`](#fyn-pip-sync--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-pip-sync--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra`](#fyn-pip-sync--extra) *extra* : Include optional dependencies from the specified extra name; may be provided more than once.

```
Only applies to `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
```

[`--extra-index-url`](#fyn-pip-sync--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-pip-sync--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--group`](#fyn-pip-sync--group) *group* : Install the specified dependency group from a `pylock.toml` or `pyproject.toml`.

```
If no path is provided, the `pylock.toml` or `pyproject.toml` in the working directory is used.

May be provided multiple times.
```

[`--help`](#fyn-pip-sync--help), `-h` : Display the concise help for this command

[`--index`](#fyn-pip-sync--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-pip-sync--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-pip-sync--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-pip-sync--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-pip-sync--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-pip-sync--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-sync--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-allow-empty-requirements`](#fyn-pip-sync--no-allow-empty-requirements)

[`--no-binary`](#fyn-pip-sync--no-binary) *no-binary* : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--no-break-system-packages`](#fyn-pip-sync--no-break-system-packages)

[`--no-build`](#fyn-pip-sync--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

Alias for `--only-binary :all:`.
```

[`--no-build-isolation`](#fyn-pip-sync--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-cache`](#fyn-pip-sync--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-index`](#fyn-pip-sync--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-pip-sync--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-sync--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-sync--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-pip-sync--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-pip-sync--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--no-verify-hashes`](#fyn-pip-sync--no-verify-hashes) : Disable validation of hashes in the requirements file.

```
By default, fyn will verify any available hashes in the requirements file, but will not require that all requirements have an associated hash. To enforce hash validation, use `--require-hashes`.

May also be set with the `UV_NO_VERIFY_HASHES` environment variable.
```

[`--offline`](#fyn-pip-sync--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-binary`](#fyn-pip-sync--only-binary) *only-binary* : Only use pre-built wheels; don't build source distributions.

```
When enabled, resolving will not run code from the given packages. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--prefix`](#fyn-pip-sync--prefix) *prefix* : Install packages into `lib`, `bin`, and other top-level folders under the specified directory, as if a virtual environment were present at that location.

```
In general, prefer the use of `--python` to install into an alternate environment, as scripts and other artifacts installed via `--prefix` will reference the installing interpreter, rather than any interpreter added to the `--prefix` directory, rendering them non-portable.

Unlike other install operations, this command does not require discovery of an existing Python environment and only searches for a Python interpreter to use for package resolution. If a suitable Python interpreter cannot be found, fyn will install one. To disable this, add `--no-python-downloads`.
```

[`--project`](#fyn-pip-sync--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-sync--python), `-p` *python* : The Python interpreter into which packages should be installed.

```
By default, syncing requires a virtual environment. A path to an alternative Python can be
provided, but it is only recommended in continuous integration (CI) environments and should
be used with caution, as it can modify the system Python installation.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--python-platform`](#fyn-pip-sync--python-platform) *python-platform* : The platform for which requirements should be installed.

```
Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

When targeting macOS (Darwin), the default minimum version is `13.0`. Use `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting iOS, the default minimum version is `13.0`. Use `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting Android, the default minimum Android API level is `24`. Use `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.

WARNING: When specified, fyn will select wheels that are compatible with the *target* platform; as a result, the installed distributions may not be compatible with the *current* platform. Conversely, any distributions that are built from source may be incompatible with the *target* platform, as they will be built for the *current* platform. The `--python-platform` option is intended for advanced use cases.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--python-version`](#fyn-pip-sync--python-version) *python-version* : The minimum Python version that should be supported by the requirements (e.g., `3.7` or `3.7.9`).

```
If a patch version is omitted, the minimum patch version is assumed. For example, `3.7` is mapped to `3.7.0`.
```

[`--quiet`](#fyn-pip-sync--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-pip-sync--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-pip-sync--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--reinstall`](#fyn-pip-sync--reinstall), `--force-reinstall` : Reinstall all packages, regardless of whether they're already installed. Implies `--refresh`

[`--reinstall-package`](#fyn-pip-sync--reinstall-package) *reinstall-package* : Reinstall a specific package, regardless of whether it's already installed. Implies `--refresh-package`

[`--require-hashes`](#fyn-pip-sync--require-hashes) : Require a matching hash for each requirement.

```
By default, fyn will verify any available hashes in the requirements file, but will not require that all requirements have an associated hash.

When `--require-hashes` is enabled, *all* requirements must include a hash or set of hashes, and *all* requirements must either be pinned to exact versions (e.g., `==1.0.0`), or be specified via direct URL.

Hash-checking mode introduces a number of additional constraints:

- Git dependencies are not supported. - Editable installations are not supported. - Local dependencies are not supported, unless they point to a specific wheel (`.whl`) or source archive (`.zip`, `.tar.gz`), as opposed to a directory.

May also be set with the `UV_REQUIRE_HASHES` environment variable.
```

[`--strict`](#fyn-pip-sync--strict) : Validate the Python environment after completing the installation, to detect packages with missing dependencies or other issues

[`--system`](#fyn-pip-sync--system) : Install packages into the system Python environment.

```
By default, fyn installs into the virtual environment in the current working directory or any parent directory. The `--system` option instructs fyn to instead use the first Python found in the system `PATH`.

WARNING: `--system` is intended for use in continuous integration (CI) environments and should be used with caution, as it can modify the system Python installation.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--target`](#fyn-pip-sync--target), `-t` *target* : Install packages into the specified directory, rather than into the virtual or system Python environment. The packages will be installed at the top-level of the directory.

```
Unlike other install operations, this command does not require discovery of an existing Python environment and only searches for a Python interpreter to use for package resolution. If a suitable Python interpreter cannot be found, fyn will install one. To disable this, add `--no-python-downloads`.
```

[`--torch-backend`](#fyn-pip-sync--torch-backend) *torch-backend* : The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`).

```
When set, fyn will ignore the configured index URLs for packages in the PyTorch ecosystem, and will instead use the defined backend.

For example, when set to `cpu`, fyn will use the CPU-only PyTorch index; when set to `cu126`, fyn will use the PyTorch index for CUDA 12.6.

The `auto` mode will attempt to detect the appropriate PyTorch index based on the currently installed CUDA drivers.

This option is in preview and may change in any future release.

May also be set with the `UV_TORCH_BACKEND` environment variable.

Possible values:

- `auto`: Select the appropriate PyTorch index based on the operating system and CUDA driver version
- `cpu`: Use the CPU-only PyTorch index
- `cu130`: Use the PyTorch index for CUDA 13.0
- `cu129`: Use the PyTorch index for CUDA 12.9
- `cu128`: Use the PyTorch index for CUDA 12.8
- `cu126`: Use the PyTorch index for CUDA 12.6
- `cu125`: Use the PyTorch index for CUDA 12.5
- `cu124`: Use the PyTorch index for CUDA 12.4
- `cu123`: Use the PyTorch index for CUDA 12.3
- `cu122`: Use the PyTorch index for CUDA 12.2
- `cu121`: Use the PyTorch index for CUDA 12.1
- `cu120`: Use the PyTorch index for CUDA 12.0
- `cu118`: Use the PyTorch index for CUDA 11.8
- `cu117`: Use the PyTorch index for CUDA 11.7
- `cu116`: Use the PyTorch index for CUDA 11.6
- `cu115`: Use the PyTorch index for CUDA 11.5
- `cu114`: Use the PyTorch index for CUDA 11.4
- `cu113`: Use the PyTorch index for CUDA 11.3
- `cu112`: Use the PyTorch index for CUDA 11.2
- `cu111`: Use the PyTorch index for CUDA 11.1
- `cu110`: Use the PyTorch index for CUDA 11.0
- `cu102`: Use the PyTorch index for CUDA 10.2
- `cu101`: Use the PyTorch index for CUDA 10.1
- `cu100`: Use the PyTorch index for CUDA 10.0
- `cu92`: Use the PyTorch index for CUDA 9.2
- `cu91`: Use the PyTorch index for CUDA 9.1
- `cu90`: Use the PyTorch index for CUDA 9.0
- `cu80`: Use the PyTorch index for CUDA 8.0
- `rocm7.1`: Use the PyTorch index for ROCm 7.1
- `rocm7.0`: Use the PyTorch index for ROCm 7.0
- `rocm6.4`: Use the PyTorch index for ROCm 6.4
- `rocm6.3`: Use the PyTorch index for ROCm 6.3
- `rocm6.2.4`: Use the PyTorch index for ROCm 6.2.4
- `rocm6.2`: Use the PyTorch index for ROCm 6.2
- `rocm6.1`: Use the PyTorch index for ROCm 6.1
- `rocm6.0`: Use the PyTorch index for ROCm 6.0
- `rocm5.7`: Use the PyTorch index for ROCm 5.7
- `rocm5.6`: Use the PyTorch index for ROCm 5.6
- `rocm5.5`: Use the PyTorch index for ROCm 5.5
- `rocm5.4.2`: Use the PyTorch index for ROCm 5.4.2
- `rocm5.4`: Use the PyTorch index for ROCm 5.4
- `rocm5.3`: Use the PyTorch index for ROCm 5.3
- `rocm5.2`: Use the PyTorch index for ROCm 5.2
- `rocm5.1.1`: Use the PyTorch index for ROCm 5.1.1
- `rocm4.2`: Use the PyTorch index for ROCm 4.2
- `rocm4.1`: Use the PyTorch index for ROCm 4.1
- `rocm4.0.1`: Use the PyTorch index for ROCm 4.0.1
- `xpu`: Use the PyTorch index for Intel XPU
```

[`--verbose`](#fyn-pip-sync--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn pip install](#fyn-pip-install)

Install packages into an environment

### Usage

```
fyn pip install [OPTIONS] <PACKAGE|--requirements <REQUIREMENTS>|--editable <EDITABLE>|--group <GROUP>>
```

### Arguments

[PACKAGE](#fyn-pip-install--package) : Install all listed packages.

```
The order of the packages is used to determine priority during resolution.
```

### Options

[`--all-extras`](#fyn-pip-install--all-extras) : Include all optional dependencies.

```
Only applies to `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
```

[`--allow-insecure-host`](#fyn-pip-install--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--break-system-packages`](#fyn-pip-install--break-system-packages) : Allow fyn to modify an `EXTERNALLY-MANAGED` Python installation.

```
WARNING: `--break-system-packages` is intended for use in continuous integration (CI) environments, when installing into Python installations that are managed by an external package manager, like `apt`. It should be used with caution, as such Python installations explicitly recommend against modifications by other package managers (like fyn or `pip`).

May also be set with the `UV_BREAK_SYSTEM_PACKAGES` environment variable.
```

[`--build-constraints`](#fyn-pip-install--build-constraints), `--build-constraint`, `-b` *build-constraints* : Constrain build dependencies using the given requirements files when building source distributions.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

May also be set with the `UV_BUILD_CONSTRAINT` environment variable.
```

[`--cache-dir`](#fyn-pip-install--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-install--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-pip-install--compile-bytecode), `--compile` : Compile Python files to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-pip-install--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-pip-install--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-pip-install--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--constraints`](#fyn-pip-install--constraints), `--constraint`, `-c` *constraints* : Constrain versions using the given requirements files.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

This is equivalent to pip's `--constraint` option.

May also be set with the `UV_CONSTRAINT` environment variable.
```

[`--default-index`](#fyn-pip-install--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-pip-install--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--dry-run`](#fyn-pip-install--dry-run) : Perform a dry run, i.e., don't actually install anything but resolve the dependencies and print the resulting plan

[`--editable`](#fyn-pip-install--editable), `-e` *editable* : Install the editable package based on the provided local file path

[`--exact`](#fyn-pip-install--exact) : Perform an exact sync, removing extraneous packages.

```
By default, installing will make the minimum necessary changes to satisfy the requirements. When enabled, fyn will update the environment to exactly match the requirements, removing packages that are not included in the requirements.
```

[`--exclude-newer`](#fyn-pip-install--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-pip-install--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--excludes`](#fyn-pip-install--excludes), `--exclude` *excludes* : Exclude packages from resolution using the given requirements files.

```
Excludes files are `requirements.txt`-like files that specify packages to exclude from the resolution. When a package is excluded, it will be omitted from the dependency list entirely and its own dependencies will be ignored during the resolution phase. Excludes are unconditional in that requirement specifiers and markers are ignored; any package listed in the provided file will be omitted from all resolved environments.

May also be set with the `UV_EXCLUDE` environment variable.
```

[`--extra`](#fyn-pip-install--extra) *extra* : Include optional dependencies from the specified extra name; may be provided more than once.

```
Only applies to `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
```

[`--extra-index-url`](#fyn-pip-install--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-pip-install--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-pip-install--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--group`](#fyn-pip-install--group) *group* : Install the specified dependency group from a `pylock.toml` or `pyproject.toml`.

```
If no path is provided, the `pylock.toml` or `pyproject.toml` in the working directory is used.

May be provided multiple times.
```

[`--help`](#fyn-pip-install--help), `-h` : Display the concise help for this command

[`--index`](#fyn-pip-install--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-pip-install--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-pip-install--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-pip-install--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-pip-install--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-pip-install--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-install--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-pip-install--no-binary) *no-binary* : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--no-break-system-packages`](#fyn-pip-install--no-break-system-packages)

[`--no-build`](#fyn-pip-install--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

Alias for `--only-binary :all:`.
```

[`--no-build-isolation`](#fyn-pip-install--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-pip-install--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-cache`](#fyn-pip-install--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-pip-install--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-deps`](#fyn-pip-install--no-deps) : Ignore package dependencies, instead only installing those packages explicitly listed on the command line or in the requirements files

[`--no-index`](#fyn-pip-install--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-pip-install--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-install--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-install--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-pip-install--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-pip-install--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--no-verify-hashes`](#fyn-pip-install--no-verify-hashes) : Disable validation of hashes in the requirements file.

```
By default, fyn will verify any available hashes in the requirements file, but will not require that all requirements have an associated hash. To enforce hash validation, use `--require-hashes`.

May also be set with the `UV_NO_VERIFY_HASHES` environment variable.
```

[`--offline`](#fyn-pip-install--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-binary`](#fyn-pip-install--only-binary) *only-binary* : Only use pre-built wheels; don't build source distributions.

```
When enabled, resolving will not run code from the given packages. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--overrides`](#fyn-pip-install--overrides), `--override` *overrides* : Override versions using the given requirements files.

```
Overrides files are `requirements.txt`-like files that force a specific version of a requirement to be installed, regardless of the requirements declared by any constituent package, and regardless of whether this would be considered an invalid resolution.

While constraints are *additive*, in that they're combined with the requirements of the constituent packages, overrides are *absolute*, in that they completely replace the requirements of the constituent packages.

May also be set with the `UV_OVERRIDE` environment variable.
```

[`--prefix`](#fyn-pip-install--prefix) *prefix* : Install packages into `lib`, `bin`, and other top-level folders under the specified directory, as if a virtual environment were present at that location.

```
In general, prefer the use of `--python` to install into an alternate environment, as scripts and other artifacts installed via `--prefix` will reference the installing interpreter, rather than any interpreter added to the `--prefix` directory, rendering them non-portable.

Unlike other install operations, this command does not require discovery of an existing Python environment and only searches for a Python interpreter to use for package resolution. If a suitable Python interpreter cannot be found, fyn will install one. To disable this, add `--no-python-downloads`.
```

[`--prerelease`](#fyn-pip-install--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-pip-install--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-install--python), `-p` *python* : The Python interpreter into which packages should be installed.

```
By default, installation requires a virtual environment. A path to an alternative Python can
be provided, but it is only recommended in continuous integration (CI) environments and
should be used with caution, as it can modify the system Python installation.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--python-platform`](#fyn-pip-install--python-platform) *python-platform* : The platform for which requirements should be installed.

```
Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

When targeting macOS (Darwin), the default minimum version is `13.0`. Use `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting iOS, the default minimum version is `13.0`. Use `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting Android, the default minimum Android API level is `24`. Use `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.

WARNING: When specified, fyn will select wheels that are compatible with the *target* platform; as a result, the installed distributions may not be compatible with the *current* platform. Conversely, any distributions that are built from source may be incompatible with the *target* platform, as they will be built for the *current* platform. The `--python-platform` option is intended for advanced use cases.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--python-version`](#fyn-pip-install--python-version) *python-version* : The minimum Python version that should be supported by the requirements (e.g., `3.7` or `3.7.9`).

```
If a patch version is omitted, the minimum patch version is assumed. For example, `3.7` is mapped to `3.7.0`.
```

[`--quiet`](#fyn-pip-install--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-pip-install--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-pip-install--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--reinstall`](#fyn-pip-install--reinstall), `--force-reinstall` : Reinstall all packages, regardless of whether they're already installed. Implies `--refresh`

[`--reinstall-package`](#fyn-pip-install--reinstall-package) *reinstall-package* : Reinstall a specific package, regardless of whether it's already installed. Implies `--refresh-package`

[`--require-hashes`](#fyn-pip-install--require-hashes) : Require a matching hash for each requirement.

```
By default, fyn will verify any available hashes in the requirements file, but will not require that all requirements have an associated hash.

When `--require-hashes` is enabled, *all* requirements must include a hash or set of hashes, and *all* requirements must either be pinned to exact versions (e.g., `==1.0.0`), or be specified via direct URL.

Hash-checking mode introduces a number of additional constraints:

- Git dependencies are not supported. - Editable installations are not supported. - Local dependencies are not supported, unless they point to a specific wheel (`.whl`) or source archive (`.zip`, `.tar.gz`), as opposed to a directory.

May also be set with the `UV_REQUIRE_HASHES` environment variable.
```

[`--requirements`](#fyn-pip-install--requirements), `--requirement`, `-r` *requirements* : Install the packages listed in the given files.

```
The following formats are supported: `requirements.txt`, `.py` files with inline metadata, `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg`.

If a `pyproject.toml`, `setup.py`, or `setup.cfg` file is provided, fyn will extract the requirements for the relevant project.

If `-` is provided, then requirements will be read from stdin.
```

[`--resolution`](#fyn-pip-install--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--strict`](#fyn-pip-install--strict) : Validate the Python environment after completing the installation, to detect packages with missing dependencies or other issues

[`--system`](#fyn-pip-install--system) : Install packages into the system Python environment.

```
By default, fyn installs into the virtual environment in the current working directory or any parent directory. The `--system` option instructs fyn to instead use the first Python found in the system `PATH`.

WARNING: `--system` is intended for use in continuous integration (CI) environments and should be used with caution, as it can modify the system Python installation.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--target`](#fyn-pip-install--target), `-t` *target* : Install packages into the specified directory, rather than into the virtual or system Python environment. The packages will be installed at the top-level of the directory.

```
Unlike other install operations, this command does not require discovery of an existing Python environment and only searches for a Python interpreter to use for package resolution. If a suitable Python interpreter cannot be found, fyn will install one. To disable this, add `--no-python-downloads`.
```

[`--torch-backend`](#fyn-pip-install--torch-backend) *torch-backend* : The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`)

```
When set, fyn will ignore the configured index URLs for packages in the PyTorch ecosystem, and will instead use the defined backend.

For example, when set to `cpu`, fyn will use the CPU-only PyTorch index; when set to `cu126`, fyn will use the PyTorch index for CUDA 12.6.

The `auto` mode will attempt to detect the appropriate PyTorch index based on the currently installed CUDA drivers.

This option is in preview and may change in any future release.

May also be set with the `UV_TORCH_BACKEND` environment variable.

Possible values:

- `auto`: Select the appropriate PyTorch index based on the operating system and CUDA driver version
- `cpu`: Use the CPU-only PyTorch index
- `cu130`: Use the PyTorch index for CUDA 13.0
- `cu129`: Use the PyTorch index for CUDA 12.9
- `cu128`: Use the PyTorch index for CUDA 12.8
- `cu126`: Use the PyTorch index for CUDA 12.6
- `cu125`: Use the PyTorch index for CUDA 12.5
- `cu124`: Use the PyTorch index for CUDA 12.4
- `cu123`: Use the PyTorch index for CUDA 12.3
- `cu122`: Use the PyTorch index for CUDA 12.2
- `cu121`: Use the PyTorch index for CUDA 12.1
- `cu120`: Use the PyTorch index for CUDA 12.0
- `cu118`: Use the PyTorch index for CUDA 11.8
- `cu117`: Use the PyTorch index for CUDA 11.7
- `cu116`: Use the PyTorch index for CUDA 11.6
- `cu115`: Use the PyTorch index for CUDA 11.5
- `cu114`: Use the PyTorch index for CUDA 11.4
- `cu113`: Use the PyTorch index for CUDA 11.3
- `cu112`: Use the PyTorch index for CUDA 11.2
- `cu111`: Use the PyTorch index for CUDA 11.1
- `cu110`: Use the PyTorch index for CUDA 11.0
- `cu102`: Use the PyTorch index for CUDA 10.2
- `cu101`: Use the PyTorch index for CUDA 10.1
- `cu100`: Use the PyTorch index for CUDA 10.0
- `cu92`: Use the PyTorch index for CUDA 9.2
- `cu91`: Use the PyTorch index for CUDA 9.1
- `cu90`: Use the PyTorch index for CUDA 9.0
- `cu80`: Use the PyTorch index for CUDA 8.0
- `rocm7.1`: Use the PyTorch index for ROCm 7.1
- `rocm7.0`: Use the PyTorch index for ROCm 7.0
- `rocm6.4`: Use the PyTorch index for ROCm 6.4
- `rocm6.3`: Use the PyTorch index for ROCm 6.3
- `rocm6.2.4`: Use the PyTorch index for ROCm 6.2.4
- `rocm6.2`: Use the PyTorch index for ROCm 6.2
- `rocm6.1`: Use the PyTorch index for ROCm 6.1
- `rocm6.0`: Use the PyTorch index for ROCm 6.0
- `rocm5.7`: Use the PyTorch index for ROCm 5.7
- `rocm5.6`: Use the PyTorch index for ROCm 5.6
- `rocm5.5`: Use the PyTorch index for ROCm 5.5
- `rocm5.4.2`: Use the PyTorch index for ROCm 5.4.2
- `rocm5.4`: Use the PyTorch index for ROCm 5.4
- `rocm5.3`: Use the PyTorch index for ROCm 5.3
- `rocm5.2`: Use the PyTorch index for ROCm 5.2
- `rocm5.1.1`: Use the PyTorch index for ROCm 5.1.1
- `rocm4.2`: Use the PyTorch index for ROCm 4.2
- `rocm4.1`: Use the PyTorch index for ROCm 4.1
- `rocm4.0.1`: Use the PyTorch index for ROCm 4.0.1
- `xpu`: Use the PyTorch index for Intel XPU
```

[`--upgrade`](#fyn-pip-install--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-pip-install--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--user`](#fyn-pip-install--user)

[`--verbose`](#fyn-pip-install--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn pip download](#fyn-pip-download)

Download distribution archives into a directory

### Usage

```
fyn pip download [OPTIONS] <PACKAGE|--requirements <REQUIREMENTS>|--group <GROUP>>
```

### Arguments

[PACKAGE](#fyn-pip-download--package) : Download all listed packages.

```
The order of the packages is used to determine priority during resolution.
```

### Options

[`--all-extras`](#fyn-pip-download--all-extras) : Include all optional dependencies.

```
Only applies to `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
```

[`--allow-insecure-host`](#fyn-pip-download--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--build-constraints`](#fyn-pip-download--build-constraints), `--build-constraint`, `-b` *build-constraints* : Constrain build dependencies using the given requirements files when building source distributions.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

May also be set with the `UV_BUILD_CONSTRAINT` environment variable.
```

[`--cache-dir`](#fyn-pip-download--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-download--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-pip-download--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-pip-download--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-pip-download--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--constraints`](#fyn-pip-download--constraints), `--constraint`, `-c` *constraints* : Constrain versions using the given requirements files.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a requirement that's installed. However, including a package in a constraints file will *not* trigger the installation of that package.

This is equivalent to pip's `--constraint` option.

May also be set with the `UV_CONSTRAINT` environment variable.
```

[`--default-index`](#fyn-pip-download--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--dest`](#fyn-pip-download--dest), `-d` *dest* : Store downloaded distributions in the given directory.

```
Defaults to the current working directory.
```

[`--directory`](#fyn-pip-download--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-pip-download--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-pip-download--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--excludes`](#fyn-pip-download--excludes), `--exclude` *excludes* : Exclude packages from resolution using the given requirements files.

```
Excludes files are `requirements.txt`-like files that specify packages to exclude from the resolution. When a package is excluded, it will be omitted from the dependency list entirely and its own dependencies will be ignored during the resolution phase. Excludes are unconditional in that requirement specifiers and markers are ignored; any package listed in the provided file will be omitted from all resolved environments.

May also be set with the `UV_EXCLUDE` environment variable.
```

[`--extra`](#fyn-pip-download--extra) *extra* : Include optional dependencies from the specified extra name; may be provided more than once.

```
Only applies to `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
```

[`--extra-index-url`](#fyn-pip-download--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-pip-download--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-pip-download--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--group`](#fyn-pip-download--group) *group* : Download the specified dependency group from a `pyproject.toml`.

```
If no path is provided, the `pyproject.toml` in the working directory is used.

May be provided multiple times.
```

[`--help`](#fyn-pip-download--help), `-h` : Display the concise help for this command

[`--index`](#fyn-pip-download--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-pip-download--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-pip-download--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-pip-download--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-pip-download--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
This option is only used when building source distributions.

Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-pip-download--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-download--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-pip-download--no-binary) *no-binary* : Don't download pre-built wheels.

```
Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--no-build`](#fyn-pip-download--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

Alias for `--only-binary :all:`.
```

[`--no-build-isolation`](#fyn-pip-download--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-pip-download--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-cache`](#fyn-pip-download--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-pip-download--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-deps`](#fyn-pip-download--no-deps) : Ignore package dependencies, instead only downloading those packages explicitly listed on the command line or in the requirements files

[`--no-index`](#fyn-pip-download--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-pip-download--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-download--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-download--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-pip-download--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-pip-download--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--offline`](#fyn-pip-download--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-binary`](#fyn-pip-download--only-binary) *only-binary* : Only download pre-built wheels; don't build source distributions.

```
Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--overrides`](#fyn-pip-download--overrides), `--override` *overrides* : Override versions using the given requirements files.

```
Overrides files are `requirements.txt`-like files that force a specific version of a requirement to be installed, regardless of the requirements declared by any constituent package, and regardless of whether this would be considered an invalid resolution.

While constraints are *additive*, in that they're combined with the requirements of the constituent packages, overrides are *absolute*, in that they completely replace the requirements of the constituent packages.

May also be set with the `UV_OVERRIDE` environment variable.
```

[`--prerelease`](#fyn-pip-download--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-pip-download--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-download--python), `-p` *python* : The Python interpreter to use during resolution.

```
A Python interpreter is required for building source distributions to determine package
metadata when there are not wheels.

The interpreter is also used to determine the default minimum Python version, unless
`--python-version` is provided.

This option respects `UV_PYTHON`, but when set via environment variable, it is overridden
by `--python-version`.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.
```

[`--python-platform`](#fyn-pip-download--python-platform) *python-platform* : The platform for which requirements should be downloaded.

```
Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--python-version`](#fyn-pip-download--python-version) *python-version* : The minimum Python version that should be supported by the downloaded requirements (e.g., `3.7` or `3.7.9`).

```
If a patch version is omitted, the minimum patch version is assumed. For example, `3.7` is mapped to `3.7.0`.
```

[`--quiet`](#fyn-pip-download--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-pip-download--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-pip-download--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--requirements`](#fyn-pip-download--requirements), `--requirement`, `-r` *requirements* : Download the packages listed in the given files.

```
The following formats are supported: `requirements.txt`, `.py` files with inline metadata, `pyproject.toml`, `setup.py`, and `setup.cfg`.

If a `pyproject.toml`, `setup.py`, or `setup.cfg` file is provided, fyn will extract the requirements for the relevant project.

If `-` is provided, then requirements will be read from stdin.
```

[`--resolution`](#fyn-pip-download--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--system`](#fyn-pip-download--system) : Use a system Python interpreter during resolution.

```
By default, fyn uses a virtual environment in the current working directory or any parent directory, falling back to searching for a Python executable in `PATH`. The `--system` option instructs fyn to avoid using a virtual environment Python and restrict its search to the system path.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--torch-backend`](#fyn-pip-download--torch-backend) *torch-backend* : The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`).

```
When set, fyn will ignore the configured index URLs for packages in the PyTorch ecosystem, and will instead use the defined backend.

The `auto` mode will attempt to detect the appropriate PyTorch index based on the currently installed CUDA drivers.

This option is in preview and may change in any future release.

May also be set with the `UV_TORCH_BACKEND` environment variable.

Possible values:

- `auto`: Select the appropriate PyTorch index based on the operating system and CUDA driver version
- `cpu`: Use the CPU-only PyTorch index
- `cu130`: Use the PyTorch index for CUDA 13.0
- `cu129`: Use the PyTorch index for CUDA 12.9
- `cu128`: Use the PyTorch index for CUDA 12.8
- `cu126`: Use the PyTorch index for CUDA 12.6
- `cu125`: Use the PyTorch index for CUDA 12.5
- `cu124`: Use the PyTorch index for CUDA 12.4
- `cu123`: Use the PyTorch index for CUDA 12.3
- `cu122`: Use the PyTorch index for CUDA 12.2
- `cu121`: Use the PyTorch index for CUDA 12.1
- `cu120`: Use the PyTorch index for CUDA 12.0
- `cu118`: Use the PyTorch index for CUDA 11.8
- `cu117`: Use the PyTorch index for CUDA 11.7
- `cu116`: Use the PyTorch index for CUDA 11.6
- `cu115`: Use the PyTorch index for CUDA 11.5
- `cu114`: Use the PyTorch index for CUDA 11.4
- `cu113`: Use the PyTorch index for CUDA 11.3
- `cu112`: Use the PyTorch index for CUDA 11.2
- `cu111`: Use the PyTorch index for CUDA 11.1
- `cu110`: Use the PyTorch index for CUDA 11.0
- `cu102`: Use the PyTorch index for CUDA 10.2
- `cu101`: Use the PyTorch index for CUDA 10.1
- `cu100`: Use the PyTorch index for CUDA 10.0
- `cu92`: Use the PyTorch index for CUDA 9.2
- `cu91`: Use the PyTorch index for CUDA 9.1
- `cu90`: Use the PyTorch index for CUDA 9.0
- `cu80`: Use the PyTorch index for CUDA 8.0
- `rocm7.1`: Use the PyTorch index for ROCm 7.1
- `rocm7.0`: Use the PyTorch index for ROCm 7.0
- `rocm6.4`: Use the PyTorch index for ROCm 6.4
- `rocm6.3`: Use the PyTorch index for ROCm 6.3
- `rocm6.2.4`: Use the PyTorch index for ROCm 6.2.4
- `rocm6.2`: Use the PyTorch index for ROCm 6.2
- `rocm6.1`: Use the PyTorch index for ROCm 6.1
- `rocm6.0`: Use the PyTorch index for ROCm 6.0
- `rocm5.7`: Use the PyTorch index for ROCm 5.7
- `rocm5.6`: Use the PyTorch index for ROCm 5.6
- `rocm5.5`: Use the PyTorch index for ROCm 5.5
- `rocm5.4.2`: Use the PyTorch index for ROCm 5.4.2
- `rocm5.4`: Use the PyTorch index for ROCm 5.4
- `rocm5.3`: Use the PyTorch index for ROCm 5.3
- `rocm5.2`: Use the PyTorch index for ROCm 5.2
- `rocm5.1.1`: Use the PyTorch index for ROCm 5.1.1
- `rocm4.2`: Use the PyTorch index for ROCm 4.2
- `rocm4.1`: Use the PyTorch index for ROCm 4.1
- `rocm4.0.1`: Use the PyTorch index for ROCm 4.0.1
- `xpu`: Use the PyTorch index for Intel XPU
```

[`--verbose`](#fyn-pip-download--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn pip wheel](#fyn-pip-wheel)

Build wheels into a directory

### Usage

```
fyn pip wheel [OPTIONS] <PACKAGE|--requirements <REQUIREMENTS>|--group <GROUP>>
```

### Arguments

[PACKAGE](#fyn-pip-wheel--package) : Build wheels for all listed packages.

```
The order of the packages is used to determine priority during resolution.
```

### Options

[`--all-extras`](#fyn-pip-wheel--all-extras) : Include all optional dependencies.

```
Only applies to `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
```

[`--allow-insecure-host`](#fyn-pip-wheel--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--build-constraints`](#fyn-pip-wheel--build-constraints), `--build-constraint`, `-b` *build-constraints* : Constrain build dependencies using the given requirements files when building source distributions

```
May also be set with the `UV_BUILD_CONSTRAINT` environment variable.
```

[`--cache-dir`](#fyn-pip-wheel--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-wheel--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-pip-wheel--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-pip-wheel--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-pip-wheel--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--constraints`](#fyn-pip-wheel--constraints), `--constraint`, `-c` *constraints* : Constrain versions using the given requirements files

```
May also be set with the `UV_CONSTRAINT` environment variable.
```

[`--default-index`](#fyn-pip-wheel--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-pip-wheel--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-pip-wheel--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-pip-wheel--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--excludes`](#fyn-pip-wheel--excludes), `--exclude` *excludes* : Exclude packages from resolution using the given requirements files

```
May also be set with the `UV_EXCLUDE` environment variable.
```

[`--extra`](#fyn-pip-wheel--extra) *extra* : Include optional dependencies from the specified extra name; may be provided more than once.

```
Only applies to `pyproject.toml`, `setup.py`, and `setup.cfg` sources.
```

[`--extra-index-url`](#fyn-pip-wheel--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-pip-wheel--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-pip-wheel--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--group`](#fyn-pip-wheel--group) *group* : Build wheels for the specified dependency group from a `pyproject.toml`.

```
If no path is provided, the `pyproject.toml` in the working directory is used.

May be provided multiple times.
```

[`--help`](#fyn-pip-wheel--help), `-h` : Display the concise help for this command

[`--index`](#fyn-pip-wheel--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-pip-wheel--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-pip-wheel--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-pip-wheel--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-pip-wheel--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
This option is only used when building source distributions.

Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-pip-wheel--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-wheel--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-pip-wheel--no-binary) *no-binary* : Don't download pre-built wheels.

```
Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--no-build`](#fyn-pip-wheel--no-build) : Don't build source distributions.

```
Alias for `--only-binary :all:`.
```

[`--no-build-isolation`](#fyn-pip-wheel--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-pip-wheel--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-cache`](#fyn-pip-wheel--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-pip-wheel--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-deps`](#fyn-pip-wheel--no-deps) : Ignore package dependencies, instead only building wheels for those packages explicitly listed on the command line or in the requirements files

[`--no-index`](#fyn-pip-wheel--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-pip-wheel--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-wheel--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-wheel--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-pip-wheel--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-pip-wheel--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--offline`](#fyn-pip-wheel--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-binary`](#fyn-pip-wheel--only-binary) *only-binary* : Only download pre-built wheels; don't build source distributions.

```
Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--overrides`](#fyn-pip-wheel--overrides), `--override` *overrides* : Override versions using the given requirements files

```
May also be set with the `UV_OVERRIDE` environment variable.
```

[`--prerelease`](#fyn-pip-wheel--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-pip-wheel--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-wheel--python), `-p` *python* : The Python interpreter to use during resolution and wheel builds.

```
This option respects `UV_PYTHON`, but when set via environment variable, it is overridden
by `--python-version`.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.
```

[`--python-platform`](#fyn-pip-wheel--python-platform) *python-platform* : The platform for which wheels should be built

```
Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--python-version`](#fyn-pip-wheel--python-version) *python-version* : The minimum Python version that should be supported by the built wheels (e.g., `3.7` or `3.7.9`)

[`--quiet`](#fyn-pip-wheel--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-pip-wheel--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-pip-wheel--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--requirements`](#fyn-pip-wheel--requirements), `--requirement`, `-r` *requirements* : Build wheels for the packages listed in the given files.

```
The following formats are supported: `requirements.txt`, `.py` files with inline metadata, `pyproject.toml`, `setup.py`, and `setup.cfg`.

If a `pyproject.toml`, `setup.py`, or `setup.cfg` file is provided, fyn will extract the requirements for the relevant project.

If `-` is provided, then requirements will be read from stdin.
```

[`--resolution`](#fyn-pip-wheel--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--system`](#fyn-pip-wheel--system) : Use a system Python interpreter during resolution

```
May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--torch-backend`](#fyn-pip-wheel--torch-backend) *torch-backend* : The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`)

```
May also be set with the `UV_TORCH_BACKEND` environment variable.

Possible values:

- `auto`: Select the appropriate PyTorch index based on the operating system and CUDA driver version
- `cpu`: Use the CPU-only PyTorch index
- `cu130`: Use the PyTorch index for CUDA 13.0
- `cu129`: Use the PyTorch index for CUDA 12.9
- `cu128`: Use the PyTorch index for CUDA 12.8
- `cu126`: Use the PyTorch index for CUDA 12.6
- `cu125`: Use the PyTorch index for CUDA 12.5
- `cu124`: Use the PyTorch index for CUDA 12.4
- `cu123`: Use the PyTorch index for CUDA 12.3
- `cu122`: Use the PyTorch index for CUDA 12.2
- `cu121`: Use the PyTorch index for CUDA 12.1
- `cu120`: Use the PyTorch index for CUDA 12.0
- `cu118`: Use the PyTorch index for CUDA 11.8
- `cu117`: Use the PyTorch index for CUDA 11.7
- `cu116`: Use the PyTorch index for CUDA 11.6
- `cu115`: Use the PyTorch index for CUDA 11.5
- `cu114`: Use the PyTorch index for CUDA 11.4
- `cu113`: Use the PyTorch index for CUDA 11.3
- `cu112`: Use the PyTorch index for CUDA 11.2
- `cu111`: Use the PyTorch index for CUDA 11.1
- `cu110`: Use the PyTorch index for CUDA 11.0
- `cu102`: Use the PyTorch index for CUDA 10.2
- `cu101`: Use the PyTorch index for CUDA 10.1
- `cu100`: Use the PyTorch index for CUDA 10.0
- `cu92`: Use the PyTorch index for CUDA 9.2
- `cu91`: Use the PyTorch index for CUDA 9.1
- `cu90`: Use the PyTorch index for CUDA 9.0
- `cu80`: Use the PyTorch index for CUDA 8.0
- `rocm7.1`: Use the PyTorch index for ROCm 7.1
- `rocm7.0`: Use the PyTorch index for ROCm 7.0
- `rocm6.4`: Use the PyTorch index for ROCm 6.4
- `rocm6.3`: Use the PyTorch index for ROCm 6.3
- `rocm6.2.4`: Use the PyTorch index for ROCm 6.2.4
- `rocm6.2`: Use the PyTorch index for ROCm 6.2
- `rocm6.1`: Use the PyTorch index for ROCm 6.1
- `rocm6.0`: Use the PyTorch index for ROCm 6.0
- `rocm5.7`: Use the PyTorch index for ROCm 5.7
- `rocm5.6`: Use the PyTorch index for ROCm 5.6
- `rocm5.5`: Use the PyTorch index for ROCm 5.5
- `rocm5.4.2`: Use the PyTorch index for ROCm 5.4.2
- `rocm5.4`: Use the PyTorch index for ROCm 5.4
- `rocm5.3`: Use the PyTorch index for ROCm 5.3
- `rocm5.2`: Use the PyTorch index for ROCm 5.2
- `rocm5.1.1`: Use the PyTorch index for ROCm 5.1.1
- `rocm4.2`: Use the PyTorch index for ROCm 4.2
- `rocm4.1`: Use the PyTorch index for ROCm 4.1
- `rocm4.0.1`: Use the PyTorch index for ROCm 4.0.1
- `xpu`: Use the PyTorch index for Intel XPU
```

[`--verbose`](#fyn-pip-wheel--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

[`--wheel-dir`](#fyn-pip-wheel--wheel-dir), `-w` *wheel-dir* : Store built wheels in the given directory.

```
Defaults to the current working directory.
```

### [fyn pip index](#fyn-pip-index)

Inspect package indexes

### Usage

```
fyn pip index [OPTIONS] <COMMAND>
```

### Commands

[`fyn pip index versions`](#fyn-pip-index-versions) : Show available versions for a package

#### [fyn pip index versions](#fyn-pip-index-versions)

Show available versions for a package

### Usage

```
fyn pip index versions [OPTIONS] <PACKAGE>
```

### Arguments

[PACKAGE](#fyn-pip-index-versions--package) : The package to inspect

### Options

[`--allow-insecure-host`](#fyn-pip-index-versions--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-pip-index-versions--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-index-versions--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-pip-index-versions--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--default-index`](#fyn-pip-index-versions--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-pip-index-versions--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-pip-index-versions--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--extra-index-url`](#fyn-pip-index-versions--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-pip-index-versions--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--help`](#fyn-pip-index-versions--help), `-h` : Display the concise help for this command

[`--index`](#fyn-pip-index-versions--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-pip-index-versions--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-pip-index-versions--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-pip-index-versions--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--managed-python`](#fyn-pip-index-versions--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-index-versions--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-pip-index-versions--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-pip-index-versions--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-index`](#fyn-pip-index-versions--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-pip-index-versions--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-index-versions--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-index-versions--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-pip-index-versions--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-pip-index-versions--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-index-versions--python), `-p` *python* : The Python interpreter to use when filtering compatible versions.

```
By default, fyn uses the current Python interpreter and its platform tags.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--python-platform`](#fyn-pip-index-versions--python-platform) *python-platform* : The platform to use when filtering compatible versions.

```
Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--python-version`](#fyn-pip-index-versions--python-version) *python-version* : The Python version to use when filtering compatible versions.

```
Defaults to the version of the discovered Python interpreter.
```

[`--quiet`](#fyn-pip-index-versions--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--system`](#fyn-pip-index-versions--system) : Use the system Python environment when filtering compatible versions.

```
Disables discovery of virtual environments.

See [fyn python](#fyn-python) for details on Python discovery.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--verbose`](#fyn-pip-index-versions--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn pip upgrade](#fyn-pip-upgrade)

Upgrade packages in an environment

### Usage

```
fyn pip upgrade [OPTIONS] --all
```

### Options

[`--all`](#fyn-pip-upgrade--all) : Upgrade all installed packages in the current environment

[`--allow-insecure-host`](#fyn-pip-upgrade--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--break-system-packages`](#fyn-pip-upgrade--break-system-packages) : Allow fyn to modify an `EXTERNALLY-MANAGED` Python installation.

```
WARNING: `--break-system-packages` is intended for use in continuous integration (CI) environments, when installing into Python installations that are managed by an external package manager, like `apt`. It should be used with caution, as such Python installations explicitly recommend against modifications by other package managers (like fyn or `pip`).

May also be set with the `UV_BREAK_SYSTEM_PACKAGES` environment variable.
```

[`--cache-dir`](#fyn-pip-upgrade--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-upgrade--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--compile-bytecode`](#fyn-pip-upgrade--compile-bytecode), `--compile` : Compile Python files to bytecode after installation.

```
By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

May also be set with the `UV_COMPILE_BYTECODE` environment variable.
```

[`--config-file`](#fyn-pip-upgrade--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-pip-upgrade--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-pip-upgrade--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--default-index`](#fyn-pip-upgrade--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-pip-upgrade--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--dry-run`](#fyn-pip-upgrade--dry-run) : Perform a dry run, i.e., don't actually install anything but resolve the dependencies and print the resulting plan

[`--exclude-newer`](#fyn-pip-upgrade--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-pip-upgrade--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra-index-url`](#fyn-pip-upgrade--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-pip-upgrade--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--fork-strategy`](#fyn-pip-upgrade--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--help`](#fyn-pip-upgrade--help), `-h` : Display the concise help for this command

[`--index`](#fyn-pip-upgrade--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-pip-upgrade--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-pip-upgrade--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-pip-upgrade--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-pip-upgrade--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-pip-upgrade--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-upgrade--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-pip-upgrade--no-binary) *no-binary* : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--no-break-system-packages`](#fyn-pip-upgrade--no-break-system-packages)

[`--no-build`](#fyn-pip-upgrade--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

Alias for `--only-binary :all:`.
```

[`--no-build-isolation`](#fyn-pip-upgrade--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-pip-upgrade--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-cache`](#fyn-pip-upgrade--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-pip-upgrade--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-index`](#fyn-pip-upgrade--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-pip-upgrade--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-upgrade--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-upgrade--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-pip-upgrade--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-pip-upgrade--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--offline`](#fyn-pip-upgrade--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--only-binary`](#fyn-pip-upgrade--only-binary) *only-binary* : Only use pre-built wheels; don't build source distributions.

```
When enabled, resolving will not run code from the given packages. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.
```

[`--prefix`](#fyn-pip-upgrade--prefix) *prefix* : Upgrade packages in `lib`, `bin`, and other top-level folders under the specified directory, as if a virtual environment were present at that location.

```
In general, prefer the use of `--python` to upgrade an alternate environment, as scripts and other artifacts installed via `--prefix` will reference the installing interpreter, rather than any interpreter added to the `--prefix` directory, rendering them non-portable.

Unlike other upgrade operations, this command does not require discovery of an existing Python environment and only searches for a Python interpreter to use for package resolution. If a suitable Python interpreter cannot be found, fyn will install one. To disable this, add `--no-python-downloads`.
```

[`--prerelease`](#fyn-pip-upgrade--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-pip-upgrade--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-upgrade--python), `-p` *python* : The Python interpreter into which packages should be upgraded.

```
By default, upgrading requires a virtual environment. A path to an alternative Python can
be provided, but it is only recommended in continuous integration (CI) environments and
should be used with caution, as it can modify the system Python installation.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--python-platform`](#fyn-pip-upgrade--python-platform) *python-platform* : The platform for which requirements should be installed.

```
Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

When targeting macOS (Darwin), the default minimum version is `13.0`. Use `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting iOS, the default minimum version is `13.0`. Use `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting Android, the default minimum Android API level is `24`. Use `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.

WARNING: When specified, fyn will select wheels that are compatible with the *target* platform; as a result, the installed distributions may not be compatible with the *current* platform. Conversely, any distributions that are built from source may be incompatible with the *target* platform, as they will be built for the *current* platform. The `--python-platform` option is intended for advanced use cases.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--python-version`](#fyn-pip-upgrade--python-version) *python-version* : The minimum Python version that should be supported by the requirements (e.g., `3.7` or `3.7.9`).

```
If a patch version is omitted, the minimum patch version is assumed. For example, `3.7` is mapped to `3.7.0`.
```

[`--quiet`](#fyn-pip-upgrade--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--reinstall`](#fyn-pip-upgrade--reinstall), `--force-reinstall` : Reinstall all packages, regardless of whether they're already installed. Implies `--refresh`

[`--reinstall-package`](#fyn-pip-upgrade--reinstall-package) *reinstall-package* : Reinstall a specific package, regardless of whether it's already installed. Implies `--refresh-package`

[`--resolution`](#fyn-pip-upgrade--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--strict`](#fyn-pip-upgrade--strict) : Validate the Python environment after completing the upgrade, to detect packages with missing dependencies or other issues

[`--system`](#fyn-pip-upgrade--system) : Upgrade packages in the system Python environment.

```
By default, fyn upgrades packages in the virtual environment in the current working directory or any parent directory. The `--system` option instructs fyn to instead use the first Python found in the system `PATH`.

WARNING: `--system` is intended for use in continuous integration (CI) environments and should be used with caution, as it can modify the system Python installation.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--target`](#fyn-pip-upgrade--target), `-t` *target* : Upgrade packages in the specified directory, rather than in the virtual or system Python environment. The packages will be installed at the top-level of the directory.

```
Unlike other upgrade operations, this command does not require discovery of an existing Python environment and only searches for a Python interpreter to use for package resolution. If a suitable Python interpreter cannot be found, fyn will install one. To disable this, add `--no-python-downloads`.
```

[`--torch-backend`](#fyn-pip-upgrade--torch-backend) *torch-backend* : The backend to use when fetching packages in the PyTorch ecosystem (e.g., `cpu`, `cu126`, or `auto`).

```
When set, fyn will ignore the configured index URLs for packages in the PyTorch ecosystem, and will instead use the defined backend.

For example, when set to `cpu`, fyn will use the CPU-only PyTorch index; when set to `cu126`, fyn will use the PyTorch index for CUDA 12.6.

The `auto` mode will attempt to detect the appropriate PyTorch index based on the currently installed CUDA drivers.

This option is in preview and may change in any future release.

May also be set with the `UV_TORCH_BACKEND` environment variable.

Possible values:

- `auto`: Select the appropriate PyTorch index based on the operating system and CUDA driver version
- `cpu`: Use the CPU-only PyTorch index
- `cu130`: Use the PyTorch index for CUDA 13.0
- `cu129`: Use the PyTorch index for CUDA 12.9
- `cu128`: Use the PyTorch index for CUDA 12.8
- `cu126`: Use the PyTorch index for CUDA 12.6
- `cu125`: Use the PyTorch index for CUDA 12.5
- `cu124`: Use the PyTorch index for CUDA 12.4
- `cu123`: Use the PyTorch index for CUDA 12.3
- `cu122`: Use the PyTorch index for CUDA 12.2
- `cu121`: Use the PyTorch index for CUDA 12.1
- `cu120`: Use the PyTorch index for CUDA 12.0
- `cu118`: Use the PyTorch index for CUDA 11.8
- `cu117`: Use the PyTorch index for CUDA 11.7
- `cu116`: Use the PyTorch index for CUDA 11.6
- `cu115`: Use the PyTorch index for CUDA 11.5
- `cu114`: Use the PyTorch index for CUDA 11.4
- `cu113`: Use the PyTorch index for CUDA 11.3
- `cu112`: Use the PyTorch index for CUDA 11.2
- `cu111`: Use the PyTorch index for CUDA 11.1
- `cu110`: Use the PyTorch index for CUDA 11.0
- `cu102`: Use the PyTorch index for CUDA 10.2
- `cu101`: Use the PyTorch index for CUDA 10.1
- `cu100`: Use the PyTorch index for CUDA 10.0
- `cu92`: Use the PyTorch index for CUDA 9.2
- `cu91`: Use the PyTorch index for CUDA 9.1
- `cu90`: Use the PyTorch index for CUDA 9.0
- `cu80`: Use the PyTorch index for CUDA 8.0
- `rocm7.1`: Use the PyTorch index for ROCm 7.1
- `rocm7.0`: Use the PyTorch index for ROCm 7.0
- `rocm6.4`: Use the PyTorch index for ROCm 6.4
- `rocm6.3`: Use the PyTorch index for ROCm 6.3
- `rocm6.2.4`: Use the PyTorch index for ROCm 6.2.4
- `rocm6.2`: Use the PyTorch index for ROCm 6.2
- `rocm6.1`: Use the PyTorch index for ROCm 6.1
- `rocm6.0`: Use the PyTorch index for ROCm 6.0
- `rocm5.7`: Use the PyTorch index for ROCm 5.7
- `rocm5.6`: Use the PyTorch index for ROCm 5.6
- `rocm5.5`: Use the PyTorch index for ROCm 5.5
- `rocm5.4.2`: Use the PyTorch index for ROCm 5.4.2
- `rocm5.4`: Use the PyTorch index for ROCm 5.4
- `rocm5.3`: Use the PyTorch index for ROCm 5.3
- `rocm5.2`: Use the PyTorch index for ROCm 5.2
- `rocm5.1.1`: Use the PyTorch index for ROCm 5.1.1
- `rocm4.2`: Use the PyTorch index for ROCm 4.2
- `rocm4.1`: Use the PyTorch index for ROCm 4.1
- `rocm4.0.1`: Use the PyTorch index for ROCm 4.0.1
- `xpu`: Use the PyTorch index for Intel XPU
```

[`--verbose`](#fyn-pip-upgrade--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn pip uninstall](#fyn-pip-uninstall)

Uninstall packages from an environment

### Usage

```
fyn pip uninstall [OPTIONS] <PACKAGE|--requirements <REQUIREMENTS>>
```

### Arguments

[PACKAGE](#fyn-pip-uninstall--package) : Uninstall all listed packages

### Options

[`--allow-insecure-host`](#fyn-pip-uninstall--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--break-system-packages`](#fyn-pip-uninstall--break-system-packages) : Allow fyn to modify an `EXTERNALLY-MANAGED` Python installation.

```
WARNING: `--break-system-packages` is intended for use in continuous integration (CI) environments, when installing into Python installations that are managed by an external package manager, like `apt`. It should be used with caution, as such Python installations explicitly recommend against modifications by other package managers (like fyn or `pip`).

May also be set with the `UV_BREAK_SYSTEM_PACKAGES` environment variable.
```

[`--cache-dir`](#fyn-pip-uninstall--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-uninstall--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-pip-uninstall--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-pip-uninstall--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--dry-run`](#fyn-pip-uninstall--dry-run) : Perform a dry run, i.e., don't actually uninstall anything but print the resulting plan

[`--help`](#fyn-pip-uninstall--help), `-h` : Display the concise help for this command

[`--keyring-provider`](#fyn-pip-uninstall--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for remote requirements files.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--managed-python`](#fyn-pip-uninstall--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-uninstall--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-break-system-packages`](#fyn-pip-uninstall--no-break-system-packages)

[`--no-cache`](#fyn-pip-uninstall--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-pip-uninstall--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-pip-uninstall--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-uninstall--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-uninstall--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-pip-uninstall--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--prefix`](#fyn-pip-uninstall--prefix) *prefix* : Uninstall packages from the specified `--prefix` directory

[`--project`](#fyn-pip-uninstall--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-uninstall--python), `-p` *python* : The Python interpreter from which packages should be uninstalled.

```
By default, uninstallation requires a virtual environment. A path to an alternative Python
can be provided, but it is only recommended in continuous integration (CI) environments and
should be used with caution, as it can modify the system Python installation.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-pip-uninstall--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--requirements`](#fyn-pip-uninstall--requirements), `--requirement`, `-r` *requirements* : Uninstall the packages listed in the given files.

```
The following formats are supported: `requirements.txt`, `.py` files with inline metadata, `pylock.toml`, `pyproject.toml`, `setup.py`, and `setup.cfg`.
```

[`--system`](#fyn-pip-uninstall--system) : Use the system Python to uninstall packages.

```
By default, fyn uninstalls from the virtual environment in the current working directory or any parent directory. The `--system` option instructs fyn to instead use the first Python found in the system `PATH`.

WARNING: `--system` is intended for use in continuous integration (CI) environments and should be used with caution, as it can modify the system Python installation.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--target`](#fyn-pip-uninstall--target), `-t` *target* : Uninstall packages from the specified `--target` directory

[`--verbose`](#fyn-pip-uninstall--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn pip freeze](#fyn-pip-freeze)

List, in requirements format, packages installed in an environment

### Usage

```
fyn pip freeze [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-pip-freeze--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-pip-freeze--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-freeze--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-pip-freeze--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-pip-freeze--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude`](#fyn-pip-freeze--exclude) *exclude* : Exclude the specified package(s) from the output

[`--exclude-editable`](#fyn-pip-freeze--exclude-editable) : Exclude any editable packages from output

[`--help`](#fyn-pip-freeze--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-pip-freeze--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-freeze--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-pip-freeze--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-pip-freeze--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-pip-freeze--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-freeze--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-freeze--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-pip-freeze--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--path`](#fyn-pip-freeze--path) *paths* : Restrict to the specified installation path for listing packages (can be used multiple times)

[`--prefix`](#fyn-pip-freeze--prefix) *prefix* : List packages from the specified `--prefix` directory

[`--project`](#fyn-pip-freeze--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-freeze--python), `-p` *python* : The Python interpreter for which packages should be listed.

```
By default, fyn lists packages in a virtual environment but will show packages in a system
Python environment if no virtual environment is found.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-pip-freeze--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--strict`](#fyn-pip-freeze--strict) : Validate the Python environment, to detect packages with missing dependencies and other issues

[`--system`](#fyn-pip-freeze--system) : List packages in the system Python environment.

```
Disables discovery of virtual environments.

See [fyn python](#fyn-python) for details on Python discovery.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--target`](#fyn-pip-freeze--target), `-t` *target* : List packages from the specified `--target` directory

[`--verbose`](#fyn-pip-freeze--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn pip list](#fyn-pip-list)

List, in tabular format, packages installed in an environment

### Usage

```
fyn pip list [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-pip-list--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-pip-list--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-list--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-pip-list--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--default-index`](#fyn-pip-list--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-pip-list--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--editable`](#fyn-pip-list--editable), `-e` : Only include editable projects

[`--exclude`](#fyn-pip-list--exclude) *exclude* : Exclude the specified package(s) from the output

[`--exclude-editable`](#fyn-pip-list--exclude-editable) : Exclude any editable packages from output

[`--exclude-newer`](#fyn-pip-list--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--extra-index-url`](#fyn-pip-list--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-pip-list--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--format`](#fyn-pip-list--format) *format* : Select the output format

```
[default: columns]

Possible values:

- `columns`: Display the list of packages in a human-readable table
- `freeze`: Display the list of packages in a `pip freeze`-like format, with one package per line alongside its version
- `json`: Display the list of packages in a machine-readable JSON format
```

[`--help`](#fyn-pip-list--help), `-h` : Display the concise help for this command

[`--index`](#fyn-pip-list--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-pip-list--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-pip-list--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-pip-list--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--managed-python`](#fyn-pip-list--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-list--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-pip-list--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-pip-list--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-index`](#fyn-pip-list--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-pip-list--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-list--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-list--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-pip-list--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--outdated`](#fyn-pip-list--outdated) : List outdated packages.

```
The latest version of each package will be shown alongside the installed version. Up-to-date packages will be omitted from the output.
```

[`--prefix`](#fyn-pip-list--prefix) *prefix* : List packages from the specified `--prefix` directory

[`--project`](#fyn-pip-list--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-list--python), `-p` *python* : The Python interpreter for which packages should be listed.

```
By default, fyn lists packages in a virtual environment but will show packages in a system
Python environment if no virtual environment is found.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-pip-list--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--strict`](#fyn-pip-list--strict) : Validate the Python environment, to detect packages with missing dependencies and other issues

[`--system`](#fyn-pip-list--system) : List packages in the system Python environment.

```
Disables discovery of virtual environments.

See [fyn python](#fyn-python) for details on Python discovery.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--target`](#fyn-pip-list--target), `-t` *target* : List packages from the specified `--target` directory

[`--verbose`](#fyn-pip-list--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn pip show](#fyn-pip-show)

Show information about one or more installed packages

### Usage

```
fyn pip show [OPTIONS] [PACKAGE]...
```

### Arguments

[PACKAGE](#fyn-pip-show--package) : The package(s) to display

### Options

[`--allow-insecure-host`](#fyn-pip-show--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-pip-show--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-show--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-pip-show--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-pip-show--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--files`](#fyn-pip-show--files), `-f` : Show the full list of installed files for each package

[`--help`](#fyn-pip-show--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-pip-show--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-show--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-pip-show--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-pip-show--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-pip-show--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-show--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-show--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-pip-show--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--prefix`](#fyn-pip-show--prefix) *prefix* : Show a package from the specified `--prefix` directory

[`--project`](#fyn-pip-show--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-show--python), `-p` *python* : The Python interpreter to find the package in.

```
By default, fyn looks for packages in a virtual environment but will look for packages in a
system Python environment if no virtual environment is found.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-pip-show--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--strict`](#fyn-pip-show--strict) : Validate the Python environment, to detect packages with missing dependencies and other issues

[`--system`](#fyn-pip-show--system) : Show a package in the system Python environment.

```
Disables discovery of virtual environments.

See [fyn python](#fyn-python) for details on Python discovery.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--target`](#fyn-pip-show--target), `-t` *target* : Show a package from the specified `--target` directory

[`--verbose`](#fyn-pip-show--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn pip tree](#fyn-pip-tree)

Display the dependency tree for an environment

### Usage

```
fyn pip tree [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-pip-tree--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-pip-tree--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-tree--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-pip-tree--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--default-index`](#fyn-pip-tree--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--depth`](#fyn-pip-tree--depth), `-d` *depth* : Maximum display depth of the dependency tree

```
[default: 255]
```

[`--directory`](#fyn-pip-tree--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-pip-tree--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--extra-index-url`](#fyn-pip-tree--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-pip-tree--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--help`](#fyn-pip-tree--help), `-h` : Display the concise help for this command

[`--index`](#fyn-pip-tree--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-pip-tree--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-pip-tree--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--invert`](#fyn-pip-tree--invert), `--reverse` : Show the reverse dependencies for the given package. This flag will invert the tree and display the packages that depend on the given package

[`--keyring-provider`](#fyn-pip-tree--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--managed-python`](#fyn-pip-tree--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-tree--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-pip-tree--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-pip-tree--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-dedupe`](#fyn-pip-tree--no-dedupe) : Do not de-duplicate repeated dependencies. Usually, when a package has already displayed its dependencies, further occurrences will not re-display its dependencies, and will include a (\*) to indicate it has already been shown. This flag will cause those duplicates to be repeated

[`--no-index`](#fyn-pip-tree--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-pip-tree--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-tree--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-tree--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-pip-tree--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--outdated`](#fyn-pip-tree--outdated) : Show the latest available version of each package in the tree

[`--package`](#fyn-pip-tree--package) *package* : Display only the specified packages

[`--project`](#fyn-pip-tree--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--prune`](#fyn-pip-tree--prune) *prune* : Prune the given package from the display of the dependency tree

[`--python`](#fyn-pip-tree--python), `-p` *python* : The Python interpreter for which packages should be listed.

```
By default, fyn lists packages in a virtual environment but will show packages in a system
Python environment if no virtual environment is found.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-pip-tree--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--show-sizes`](#fyn-pip-tree--show-sizes) : Show compressed wheel sizes for packages in the tree

[`--show-version-specifiers`](#fyn-pip-tree--show-version-specifiers) : Show the version constraint(s) imposed on each package

[`--strict`](#fyn-pip-tree--strict) : Validate the Python environment, to detect packages with missing dependencies and other issues

[`--system`](#fyn-pip-tree--system) : List packages in the system Python environment.

```
Disables discovery of virtual environments.

See [fyn python](#fyn-python) for details on Python discovery.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--verbose`](#fyn-pip-tree--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn pip check](#fyn-pip-check)

Verify installed packages have compatible dependencies

### Usage

```
fyn pip check [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-pip-check--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-pip-check--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-pip-check--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-pip-check--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-pip-check--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-pip-check--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-pip-check--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-pip-check--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-pip-check--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-pip-check--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-pip-check--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-pip-check--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-pip-check--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-pip-check--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-pip-check--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-pip-check--python), `-p` *python* : The Python interpreter for which packages should be checked.

```
By default, fyn checks packages in a virtual environment but will check packages in a system
Python environment if no virtual environment is found.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--python-platform`](#fyn-pip-check--python-platform) *python-platform* : The platform for which packages should be checked.

```
By default, the installed packages are checked against the platform of the current interpreter.

Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

When targeting macOS (Darwin), the default minimum version is `13.0`. Use `MACOSX_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting iOS, the default minimum version is `13.0`. Use `IPHONEOS_DEPLOYMENT_TARGET` to specify a different minimum version, e.g., `14.0`.

When targeting Android, the default minimum Android API level is `24`. Use `ANDROID_API_LEVEL` to specify a different minimum version, e.g., `26`.

Possible values:

- `windows`: An alias for `x86_64-pc-windows-msvc`, the default target for Windows
- `linux`: An alias for `x86_64-unknown-linux-gnu`, the default target for Linux
- `macos`: An alias for `aarch64-apple-darwin`, the default target for macOS
- `x86_64-pc-windows-msvc`: A 64-bit x86 Windows target
- `aarch64-pc-windows-msvc`: An ARM64 Windows target
- `i686-pc-windows-msvc`: A 32-bit x86 Windows target
- `x86_64-unknown-linux-gnu`: An x86 Linux target. Equivalent to `x86_64-manylinux_2_28`
- `aarch64-apple-darwin`: An ARM-based macOS target, as seen on Apple Silicon devices
- `x86_64-apple-darwin`: An x86 macOS target
- `aarch64-unknown-linux-gnu`: An ARM64 Linux target. Equivalent to `aarch64-manylinux_2_28`
- `aarch64-unknown-linux-musl`: An ARM64 Linux target
- `x86_64-unknown-linux-musl`: An `x86_64` Linux target
- `riscv64-unknown-linux`: A RISCV64 Linux target
- `x86_64-manylinux2014`: An `x86_64` target for the `manylinux2014` platform. Equivalent to `x86_64-manylinux_2_17`
- `x86_64-manylinux_2_17`: An `x86_64` target for the `manylinux_2_17` platform
- `x86_64-manylinux_2_28`: An `x86_64` target for the `manylinux_2_28` platform
- `x86_64-manylinux_2_31`: An `x86_64` target for the `manylinux_2_31` platform
- `x86_64-manylinux_2_32`: An `x86_64` target for the `manylinux_2_32` platform
- `x86_64-manylinux_2_33`: An `x86_64` target for the `manylinux_2_33` platform
- `x86_64-manylinux_2_34`: An `x86_64` target for the `manylinux_2_34` platform
- `x86_64-manylinux_2_35`: An `x86_64` target for the `manylinux_2_35` platform
- `x86_64-manylinux_2_36`: An `x86_64` target for the `manylinux_2_36` platform
- `x86_64-manylinux_2_37`: An `x86_64` target for the `manylinux_2_37` platform
- `x86_64-manylinux_2_38`: An `x86_64` target for the `manylinux_2_38` platform
- `x86_64-manylinux_2_39`: An `x86_64` target for the `manylinux_2_39` platform
- `x86_64-manylinux_2_40`: An `x86_64` target for the `manylinux_2_40` platform
- `aarch64-manylinux2014`: An ARM64 target for the `manylinux2014` platform. Equivalent to `aarch64-manylinux_2_17`
- `aarch64-manylinux_2_17`: An ARM64 target for the `manylinux_2_17` platform
- `aarch64-manylinux_2_28`: An ARM64 target for the `manylinux_2_28` platform
- `aarch64-manylinux_2_31`: An ARM64 target for the `manylinux_2_31` platform
- `aarch64-manylinux_2_32`: An ARM64 target for the `manylinux_2_32` platform
- `aarch64-manylinux_2_33`: An ARM64 target for the `manylinux_2_33` platform
- `aarch64-manylinux_2_34`: An ARM64 target for the `manylinux_2_34` platform
- `aarch64-manylinux_2_35`: An ARM64 target for the `manylinux_2_35` platform
- `aarch64-manylinux_2_36`: An ARM64 target for the `manylinux_2_36` platform
- `aarch64-manylinux_2_37`: An ARM64 target for the `manylinux_2_37` platform
- `aarch64-manylinux_2_38`: An ARM64 target for the `manylinux_2_38` platform
- `aarch64-manylinux_2_39`: An ARM64 target for the `manylinux_2_39` platform
- `aarch64-manylinux_2_40`: An ARM64 target for the `manylinux_2_40` platform
- `aarch64-linux-android`: An ARM64 Android target
- `x86_64-linux-android`: An `x86_64` Android target
- `wasm32-pyodide2024`: A wasm32 target using the Pyodide 2024 platform. Meant for use with Python 3.12
- `arm64-apple-ios`: An ARM64 target for iOS device
- `arm64-apple-ios-simulator`: An ARM64 target for iOS simulator
- `x86_64-apple-ios-simulator`: An `x86_64` target for iOS simulator
```

[`--python-version`](#fyn-pip-check--python-version) *python-version* : The Python version against which packages should be checked.

```
By default, the installed packages are checked against the version of the current interpreter.
```

[`--quiet`](#fyn-pip-check--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--system`](#fyn-pip-check--system) : Check packages in the system Python environment.

```
Disables discovery of virtual environments.

See [fyn python](#fyn-python) for details on Python discovery.

May also be set with the `UV_SYSTEM_PYTHON` environment variable.
```

[`--verbose`](#fyn-pip-check--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn torch](#fyn-torch)

Diagnose PyTorch backend selection for the current machine

### Usage

```
fyn torch [OPTIONS] <COMMAND>
```

### Commands

[`fyn torch doctor`](#fyn-torch-doctor) : Inspect the current machine and recommend a PyTorch backend

### [fyn torch doctor](#fyn-torch-doctor)

Inspect the current machine and recommend a PyTorch backend.

Detects the current accelerator, recommends the corresponding `--torch-backend` value, and prints the exact `fyn pip install` command to run next.

This command only reports information. It never installs packages or modifies project files.

### Usage

```
fyn torch doctor [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-torch-doctor--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-torch-doctor--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-torch-doctor--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-torch-doctor--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-torch-doctor--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-torch-doctor--help), `-h` : Display the concise help for this command

[`--json`](#fyn-torch-doctor--json) : Display the diagnosis in JSON format

[`--managed-python`](#fyn-torch-doctor--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-torch-doctor--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-torch-doctor--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-torch-doctor--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-torch-doctor--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-torch-doctor--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-torch-doctor--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-torch-doctor--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-torch-doctor--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-torch-doctor--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-torch-doctor--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn venv](#fyn-venv)

Create a virtual environment.

By default, creates a virtual environment named `.venv` in the working directory. An alternative path may be provided positionally.

If in a project, the default environment name can be changed with the `UV_PROJECT_ENVIRONMENT` environment variable; this only applies when run from the project root directory.

If a virtual environment exists at the target path, it will be removed and a new, empty virtual environment will be created.

When using fyn, the virtual environment does not need to be activated. fyn will find a virtual environment (named `.venv`) in the working directory or any parent directories.

### Usage

```
fyn venv [OPTIONS] [PATH]
```

### Arguments

[PATH](#fyn-venv--path) : The path to the virtual environment to create.

```
Default to `.venv` in the working directory.

Relative paths are resolved relative to the working directory.
```

### Options

[`--allow-existing`](#fyn-venv--allow-existing) : Preserve any existing files or directories at the target path.

```
By default, `fyn venv` will exit with an error if the given path is non-empty. The `--allow-existing` option will instead write to the given path, regardless of its contents, and without clearing it beforehand.

WARNING: This option can lead to unexpected behavior if the existing virtual environment and the newly-created virtual environment are linked to different Python interpreters.
```

[`--allow-insecure-host`](#fyn-venv--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-venv--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--clear`](#fyn-venv--clear), `-c` : Remove any existing files or directories at the target path [env: UV_VENV_CLEAR=]

```
By default, `fyn venv` will exit with an error if the given path is non-empty. The `--clear` option will instead clear a non-empty path before creating a new virtual environment.
```

[`--color`](#fyn-venv--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-venv--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--default-index`](#fyn-venv--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-venv--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-venv--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-venv--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for a specific package to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or a ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra-index-url`](#fyn-venv--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-venv--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--help`](#fyn-venv--help), `-h` : Display the concise help for this command

[`--index`](#fyn-venv--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-venv--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-venv--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-venv--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-venv--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
This option is only used for installing seed packages.

Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-venv--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-venv--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-venv--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-venv--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-index`](#fyn-venv--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-venv--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-venv--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-project`](#fyn-venv--no-project), `--no-workspace` : Avoid discovering a project or workspace.

```
By default, fyn searches for projects in the current directory or any parent directory to determine the default path of the virtual environment and check for Python version constraints, if any.
```

[`--no-python-downloads`](#fyn-venv--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-venv--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-venv--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--prompt`](#fyn-venv--prompt) *prompt* : Provide an alternative prompt prefix for the virtual environment.

```
By default, the prompt is dependent on whether a path was provided to `fyn venv`. If provided
(e.g, `fyn venv project`), the prompt is set to the directory name. If not provided
(`fyn venv`), the prompt is set to the current directory's name.

If "." is provided, the current directory name will be used regardless of whether a path was
provided to `fyn venv`.
```

[`--python`](#fyn-venv--python), `-p` *python* : The Python interpreter to use for the virtual environment.

```
During virtual environment creation, fyn will not look for Python interpreters in virtual
environments.

See [fyn python](#fyn-python) for details on Python discovery and supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-venv--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-venv--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-venv--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--relocatable`](#fyn-venv--relocatable) : Make the virtual environment relocatable [env: UV_VENV_RELOCATABLE=]

```
A relocatable virtual environment can be moved around and redistributed without invalidating its associated entrypoint and activation scripts.

Note that this can only be guaranteed for standard `console_scripts` and `gui_scripts`. Other scripts may be adjusted if they ship with a generic `#!python[w]` shebang, and binaries are left as-is.

As a result of making the environment relocatable (by way of writing relative, rather than absolute paths), the entrypoints and scripts themselves will *not* be relocatable. In other words, copying those entrypoints and scripts to a location outside the environment will not work, as they reference paths relative to the environment itself.
```

[`--seed`](#fyn-venv--seed) : Install seed packages (one or more of: `pip`, `setuptools`, and `wheel`) into the virtual environment [env: UV_VENV_SEED=]

```
Note that `setuptools` and `wheel` are not included in Python 3.12+ environments.
```

[`--system-site-packages`](#fyn-venv--system-site-packages) : Give the virtual environment access to the system site packages directory.

```
Unlike `pip`, when a virtual environment is created with `--system-site-packages`, fyn will *not* take system site packages into account when running commands like `fyn pip list` or `fyn pip install`. The `--system-site-packages` flag will provide the virtual environment with access to the system site packages directory at runtime, but will not affect the behavior of fyn commands.
```

[`--verbose`](#fyn-venv--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn shell](#fyn-shell)

Activate the virtual environment in a new shell.

Spawns a new shell process with the project's virtual environment activated. The virtual environment's `bin` directory is prepended to `PATH` and `VIRTUAL_ENV` is set.

Use `exit` or Ctrl-D to leave the activated shell.

### Usage

```
fyn shell [OPTIONS] [PATH]
```

### Arguments

[PATH](#fyn-shell--path) : The path to the virtual environment to activate.

```
If not provided, fyn will look for a `.venv` directory in the current directory or any parent directory. The `VIRTUAL_ENV` environment variable can also be used to specify the virtual environment.
```

### Options

[`--allow-insecure-host`](#fyn-shell--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-shell--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-shell--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-shell--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-shell--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--env-file`](#fyn-shell--env-file) *env-file* : Load environment variables from a `.env` file.

```
Can be provided multiple times, with subsequent files overriding values defined in previous files.

May also be set with the `UV_ENV_FILE` environment variable.
```

[`--help`](#fyn-shell--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-shell--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-shell--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-shell--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-shell--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-env-file`](#fyn-shell--no-env-file) : Avoid reading environment variables from a `.env` file [env: UV_NO_ENV_FILE=]

```
May also be set with the `UV_NO_ENV_FILE` environment variable.
```

[`--no-managed-python`](#fyn-shell--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-shell--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-project`](#fyn-shell--no-project), `--no-workspace` : Avoid discovering a project or workspace.

```
By default, fyn searches for projects in the current directory or any parent directory to determine the virtual environment path.
```

[`--no-python-downloads`](#fyn-shell--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-shell--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-shell--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-shell--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-shell--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn build](#fyn-build)

Build Python packages into source distributions and wheels.

`fyn build` accepts a path to a directory or source distribution, which defaults to the current working directory.

By default, if passed a directory, `fyn build` will build a source distribution ("sdist") from the source directory, and a binary distribution ("wheel") from the source distribution.

`fyn build --sdist` can be used to build only the source distribution, `fyn build --wheel` can be used to build only the binary distribution, and `fyn build --sdist --wheel` can be used to build both distributions from source.

If passed a source distribution, `fyn build --wheel` will build a wheel from the source distribution.

### Usage

```
fyn build [OPTIONS] [SRC]
```

### Arguments

[SRC](#fyn-build--src) : The directory from which distributions should be built, or a source distribution archive to build into a wheel.

```
Defaults to the current working directory.
```

### Options

[`--all-packages`](#fyn-build--all-packages), `--all` : Builds all packages in the workspace.

```
The workspace will be discovered from the provided source directory, or the current directory if no source directory is provided.

If the workspace member does not exist, fyn will exit with an error.
```

[`--allow-insecure-host`](#fyn-build--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--build-constraints`](#fyn-build--build-constraints), `--build-constraint`, `-b` *build-constraints* : Constrain build dependencies using the given requirements files when building distributions.

```
Constraints files are `requirements.txt`-like files that only control the *version* of a build dependency that's installed. However, including a package in a constraints file will *not* trigger the inclusion of that package on its own.

May also be set with the `UV_BUILD_CONSTRAINT` environment variable.
```

[`--cache-dir`](#fyn-build--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--clear`](#fyn-build--clear) : Clear the output directory before the build, removing stale artifacts

[`--color`](#fyn-build--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-build--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--config-setting`](#fyn-build--config-setting), `--config-settings`, `-C` *config-setting* : Settings to pass to the PEP 517 build backend, specified as `KEY=VALUE` pairs

[`--config-settings-package`](#fyn-build--config-settings-package), `--config-settings-package` *config-settings-package* : Settings to pass to the PEP 517 build backend for a specific package, specified as `PACKAGE:KEY=VALUE` pairs

[`--default-index`](#fyn-build--default-index) *default-index* : The URL of the default package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--index` flag.

May also be set with the `UV_DEFAULT_INDEX` environment variable.
```

[`--directory`](#fyn-build--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--exclude-newer`](#fyn-build--exclude-newer) *exclude-newer* : Limit candidate packages to those that were uploaded prior to the given date.

```
Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), local dates in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

May also be set with the `UV_EXCLUDE_NEWER` environment variable.
```

[`--exclude-newer-package`](#fyn-build--exclude-newer-package) *exclude-newer-package* : Limit candidate packages for specific packages to those that were uploaded prior to the given date.

```
Accepts package-date pairs in the format `PACKAGE=DATE`, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a local date in the same format (e.g., `2006-12-02`) resolved based on your system's configured time zone, a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

Can be provided multiple times for different packages.
```

[`--extra-index-url`](#fyn-build--extra-index-url) *extra-index-url* : (Deprecated: use `--index` instead) Extra URLs of package indexes to use, in addition to `--index-url`.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--index-url` (which defaults to PyPI). When multiple `--extra-index-url` flags are provided, earlier values take priority.

May also be set with the `UV_EXTRA_INDEX_URL` environment variable.
```

[`--find-links`](#fyn-build--find-links), `-f` *find-links* : Locations to search for candidate distributions, in addition to those found in the registry indexes.

```
If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

May also be set with the `UV_FIND_LINKS` environment variable.
```

[`--force-pep517`](#fyn-build--force-pep517) : Always build through PEP 517, don't use the fast path for the fyn build backend.

```
By default, fyn won't create a PEP 517 build environment for packages using the fyn build backend, but use a fast path that calls into the build backend directly. This option forces always using PEP 517.
```

[`--fork-strategy`](#fyn-build--fork-strategy) *fork-strategy* : The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

```
By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

May also be set with the `UV_FORK_STRATEGY` environment variable.

Possible values:

- `fewest`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `requires-python`: Optimize for selecting latest supported version of each package, for each supported Python version
```

[`--help`](#fyn-build--help), `-h` : Display the concise help for this command

[`--index`](#fyn-build--index) *index* : The URLs to use when resolving dependencies, in addition to the default index.

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by `--default-index` (which defaults to PyPI). When multiple `--index` flags are provided, earlier values take priority.

Index names are not supported as values. Relative paths must be disambiguated from index names with `./` or `../` on Unix or `.\\`, `..\\`, `./` or `../` on Windows.

May also be set with the `UV_INDEX` environment variable.
```

[`--index-strategy`](#fyn-build--index-strategy) *index-strategy* : The strategy to use when resolving against multiple index URLs.

```
By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

May also be set with the `UV_INDEX_STRATEGY` environment variable.

Possible values:

- `first-index`: Only use results from the first index that returns a match for a given package name
- `unsafe-first-match`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `unsafe-best-match`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index
```

[`--index-url`](#fyn-build--index-url), `-i` *index-url* : (Deprecated: use `--default-index` instead) The URL of the Python package index (by default: <https://pypi.org/simple>).

```
Accepts either a repository compliant with PEP 503 (the simple repository API), or a local directory laid out in the same format.

The index given by this flag is given lower priority than all other indexes specified via the `--extra-index-url` flag.

May also be set with the `UV_INDEX_URL` environment variable.
```

[`--keyring-provider`](#fyn-build--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for index URLs.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--link-mode`](#fyn-build--link-mode) *link-mode* : The method to use when installing packages from the global cache.

```
This option is only used when building source distributions.

Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

May also be set with the `UV_LINK_MODE` environment variable.

Possible values:

- `clone`: Clone (i.e., copy-on-write) packages from the source into the destination
- `copy`: Copy packages from the source into the destination
- `hardlink`: Hard link packages from the source into the destination
- `symlink`: Symbolically link packages from the source into the destination
```

[`--managed-python`](#fyn-build--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-build--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-binary`](#fyn-build--no-binary) : Don't install pre-built wheels.

```
The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

May also be set with the `UV_NO_BINARY` environment variable.
```

[`--no-binary-package`](#fyn-build--no-binary-package) *no-binary-package* : Don't install pre-built wheels for a specific package

```
May also be set with the `UV_NO_BINARY_PACKAGE` environment variable.
```

[`--no-build`](#fyn-build--no-build) : Don't build source distributions.

```
When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

May also be set with the `UV_NO_BUILD` environment variable.
```

[`--no-build-isolation`](#fyn-build--no-build-isolation) : Disable isolation when building source distributions.

```
Assumes that build dependencies specified by PEP 518 are already installed.

May also be set with the `UV_NO_BUILD_ISOLATION` environment variable.
```

[`--no-build-isolation-package`](#fyn-build--no-build-isolation-package) *no-build-isolation-package* : Disable isolation when building source distributions for a specific package.

```
Assumes that the packages' build dependencies specified by PEP 518 are already installed.
```

[`--no-build-logs`](#fyn-build--no-build-logs) : Hide logs from the build backend

[`--no-build-package`](#fyn-build--no-build-package) *no-build-package* : Don't build source distributions for a specific package

```
May also be set with the `UV_NO_BUILD_PACKAGE` environment variable.
```

[`--no-cache`](#fyn-build--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-build--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-create-gitignore`](#fyn-build--no-create-gitignore) : Do not create a `.gitignore` file in the output directory.

```
By default, fyn creates a `.gitignore` file in the output directory to exclude build artifacts from version control. When this flag is used, the file will be omitted.
```

[`--no-index`](#fyn-build--no-index) : Ignore the registry index (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`

[`--no-managed-python`](#fyn-build--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-build--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-build--no-python-downloads) : Disable automatic downloads of Python.

[`--no-sources`](#fyn-build--no-sources) : Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any workspace, Git, URL, or local path sources

```
May also be set with the `UV_NO_SOURCES` environment variable.
```

[`--no-sources-package`](#fyn-build--no-sources-package) *no-sources-package* : Don't use sources from the `tool.fyn.sources` table for the specified packages

```
May also be set with the `UV_NO_SOURCES_PACKAGE` environment variable.
```

[`--no-verify-hashes`](#fyn-build--no-verify-hashes) : Disable validation of hashes in the requirements file.

```
By default, fyn will verify any available hashes in the requirements file, but will not require that all requirements have an associated hash. To enforce hash validation, use `--require-hashes`.

May also be set with the `UV_NO_VERIFY_HASHES` environment variable.
```

[`--offline`](#fyn-build--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--out-dir`](#fyn-build--out-dir), `-o` *out-dir* : The output directory to which distributions should be written.

```
Defaults to the `dist` subdirectory within the source directory, or the directory containing the source distribution archive.
```

[`--package`](#fyn-build--package) *package* : Build a specific package in the workspace.

```
The workspace will be discovered from the provided source directory, or the current directory if no source directory is provided.

If the workspace member does not exist, fyn will exit with an error.
```

[`--prerelease`](#fyn-build--prerelease) *prerelease* : The strategy to use when considering pre-release versions.

```
By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

May also be set with the `UV_PRERELEASE` environment variable.

Possible values:

- `disallow`: Disallow all pre-release versions
- `allow`: Allow all pre-release versions
- `if-necessary`: Allow pre-release versions if all versions of a package are pre-release
- `explicit`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `if-necessary-or-explicit`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements
```

[`--project`](#fyn-build--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--python`](#fyn-build--python), `-p` *python* : The Python interpreter to use for the build environment.

```
By default, builds are executed in isolated virtual environments. The discovered interpreter
will be used to create those environments, and will be symlinked or copied in depending on
the platform.

See [fyn python](#fyn-python) to view supported request formats.

May also be set with the `UV_PYTHON` environment variable.
```

[`--quiet`](#fyn-build--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--refresh`](#fyn-build--refresh) : Refresh all cached data

[`--refresh-package`](#fyn-build--refresh-package) *refresh-package* : Refresh cached data for a specific package

[`--require-hashes`](#fyn-build--require-hashes) : Require a matching hash for each requirement.

```
By default, fyn will verify any available hashes in the requirements file, but will not require that all requirements have an associated hash.

When `--require-hashes` is enabled, *all* requirements must include a hash or set of hashes, and *all* requirements must either be pinned to exact versions (e.g., `==1.0.0`), or be specified via direct URL.

Hash-checking mode introduces a number of additional constraints:

- Git dependencies are not supported. - Editable installations are not supported. - Local dependencies are not supported, unless they point to a specific wheel (`.whl`) or source archive (`.zip`, `.tar.gz`), as opposed to a directory.

May also be set with the `UV_REQUIRE_HASHES` environment variable.
```

[`--resolution`](#fyn-build--resolution) *resolution* : The strategy to use when selecting between the different compatible versions for a given package requirement.

```
By default, fyn will use the latest compatible version of each package (`highest`).

May also be set with the `UV_RESOLUTION` environment variable.

Possible values:

- `highest`: Resolve the highest compatible version of each package
- `lowest`: Resolve the lowest compatible version of each package
- `lowest-direct`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies
```

[`--sdist`](#fyn-build--sdist) : Build a source distribution ("sdist") from the given directory

[`--upgrade`](#fyn-build--upgrade), `-U` : Allow package upgrades, ignoring pinned versions in any existing output file. Implies `--refresh`

[`--upgrade-package`](#fyn-build--upgrade-package), `-P` *upgrade-package* : Allow upgrades for a specific package, ignoring pinned versions in any existing output file. Implies `--refresh-package`

[`--verbose`](#fyn-build--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

[`--wheel`](#fyn-build--wheel) : Build a binary distribution ("wheel") from the given directory

## [fyn publish](#fyn-publish)

Upload distributions to an index

### Usage

```
fyn publish [OPTIONS] [FILES]...
```

### Arguments

[FILES](#fyn-publish--files) : Paths to the files to upload. Accepts glob expressions.

```
Defaults to the `dist` directory. Selects only wheels and source distributions and their attestations, while ignoring other files.
```

### Options

[`--allow-insecure-host`](#fyn-publish--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-publish--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--check-url`](#fyn-publish--check-url) *check-url* : Check an index URL for existing files to skip duplicate uploads.

```
This option allows retrying publishing that failed after only some, but not all files have been uploaded, and handles errors due to parallel uploads of the same file.

Before uploading, the index is checked. If the exact same file already exists in the index, the file will not be uploaded. If an error occurred during the upload, the index is checked again, to handle cases where the identical file was uploaded twice in parallel.

The exact behavior will vary based on the index. When uploading to PyPI, uploading the same file succeeds even without `--check-url`, while most other indexes error. When uploading to pyx, the index URL can be inferred automatically from the publish URL.

The index must provide one of the supported hashes (SHA-256, SHA-384, or SHA-512).

May also be set with the `UV_PUBLISH_CHECK_URL` environment variable.
```

[`--color`](#fyn-publish--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-publish--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-publish--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--dry-run`](#fyn-publish--dry-run) : Perform a dry run without uploading files.

```
When enabled, the command will check for existing files if `--check-url` is provided, and will perform validation against the index if supported, but will not upload any files.
```

[`--help`](#fyn-publish--help), `-h` : Display the concise help for this command

[`--index`](#fyn-publish--index) *index* : The name of an index in the configuration to use for publishing.

````
The index must have a `publish-url` setting, for example:

```
[[tool.fyn.index]]
name = "pypi"
url = "https://pypi.org/simple"
publish-url = "https://upload.pypi.org/legacy/"
```

The index `url` will be used to check for existing files to skip duplicate uploads.

With these settings, the following two calls are equivalent:

```
fyn publish --index pypi
fyn publish --publish-url https://upload.pypi.org/legacy/ --check-url https://pypi.org/simple
```

May also be set with the `UV_PUBLISH_INDEX` environment variable.
````

[`--keyring-provider`](#fyn-publish--keyring-provider) *keyring-provider* : Attempt to use `keyring` for authentication for remote requirements files.

```
At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

Defaults to `disabled`.

May also be set with the `UV_KEYRING_PROVIDER` environment variable.

Possible values:

- `disabled`: Do not use keyring for credential lookup
- `subprocess`: Use the `keyring` command for credential lookup
```

[`--managed-python`](#fyn-publish--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-publish--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-attestations`](#fyn-publish--no-attestations) : Do not upload attestations for the published files.

```
By default, fyn attempts to upload matching PEP 740 attestations with each distribution that is published.

May also be set with the `UV_PUBLISH_NO_ATTESTATIONS` environment variable.
```

[`--no-cache`](#fyn-publish--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-publish--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-publish--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-publish--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-publish--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-publish--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--password`](#fyn-publish--password), `-p` *password* : The password for the upload

```
May also be set with the `UV_PUBLISH_PASSWORD` environment variable.
```

[`--project`](#fyn-publish--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--publish-url`](#fyn-publish--publish-url) *publish-url* : The URL of the upload endpoint (not the index URL).

```
Note that there are typically different URLs for index access (e.g., `https:://.../simple`) and index upload.

Defaults to PyPI's publish URL (<https://upload.pypi.org/legacy/>).

May also be set with the `UV_PUBLISH_URL` environment variable.
```

[`--quiet`](#fyn-publish--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--token`](#fyn-publish--token), `-t` *token* : The token for the upload.

```
Using a token is equivalent to passing `__token__` as `--username` and the token as `--password` password.

May also be set with the `UV_PUBLISH_TOKEN` environment variable.
```

[`--trusted-publishing`](#fyn-publish--trusted-publishing) *trusted-publishing* : Configure trusted publishing.

```
By default, fyn checks for trusted publishing when running in a supported environment, but ignores it if it isn't configured.

fyn's supported environments for trusted publishing include GitHub Actions and GitLab CI/CD.

Possible values:

- `automatic`: Attempt trusted publishing when we're in a supported environment, continue if that fails
- `always`
- `never`
```

[`--username`](#fyn-publish--username), `-u` *username* : The username for the upload

```
May also be set with the `UV_PUBLISH_USERNAME` environment variable.
```

[`--verbose`](#fyn-publish--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn cache](#fyn-cache)

Manage fyn's cache

### Usage

```
fyn cache [OPTIONS] <COMMAND>
```

### Commands

[`fyn cache clean`](#fyn-cache-clean) : Clear the cache, removing all entries or those linked to specific packages

[`fyn cache prune`](#fyn-cache-prune) : Prune all unreachable objects from the cache

[`fyn cache dir`](#fyn-cache-dir) : Show the cache directory

[`fyn cache size`](#fyn-cache-size) : Show the cache size

### [fyn cache clean](#fyn-cache-clean)

Clear the cache, removing all entries or those linked to specific packages

### Usage

```
fyn cache clean [OPTIONS] [PACKAGE]...
```

### Arguments

[PACKAGE](#fyn-cache-clean--package) : The packages to remove from the cache

### Options

[`--allow-insecure-host`](#fyn-cache-clean--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-cache-clean--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-cache-clean--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-cache-clean--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-cache-clean--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--force`](#fyn-cache-clean--force) : Force removal of the cache, ignoring in-use checks.

```
By default, `fyn cache clean` will block until no process is reading the cache. When `--force` is used, `fyn cache clean` will proceed without taking a lock.
```

[`--help`](#fyn-cache-clean--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-cache-clean--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-cache-clean--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-cache-clean--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-cache-clean--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-cache-clean--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-cache-clean--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-cache-clean--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-cache-clean--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-cache-clean--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-cache-clean--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-cache-clean--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn cache prune](#fyn-cache-prune)

Prune all unreachable objects from the cache

### Usage

```
fyn cache prune [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-cache-prune--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-cache-prune--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--ci`](#fyn-cache-prune--ci) : Optimize the cache for persistence in a continuous integration environment, like GitHub Actions.

```
By default, fyn caches both the wheels that it builds from source and the pre-built wheels that it downloads directly, to enable high-performance package installation. In some scenarios, though, persisting pre-built wheels may be undesirable. For example, in GitHub Actions, it's faster to omit pre-built wheels from the cache and instead have re-download them on each run. However, it typically *is* faster to cache wheels that are built from source, since the wheel building process can be expensive, especially for extension modules.

In `--ci` mode, fyn will prune any pre-built wheels from the cache, but retain any wheels that were built from source.
```

[`--color`](#fyn-cache-prune--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-cache-prune--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-cache-prune--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--force`](#fyn-cache-prune--force) : Force removal of the cache, ignoring in-use checks.

```
By default, `fyn cache prune` will block until no process is reading the cache. When `--force` is used, `fyn cache prune` will proceed without taking a lock.
```

[`--help`](#fyn-cache-prune--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-cache-prune--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-cache-prune--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-cache-prune--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-cache-prune--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-cache-prune--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-cache-prune--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-cache-prune--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-cache-prune--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-cache-prune--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-cache-prune--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-cache-prune--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn cache dir](#fyn-cache-dir)

Show the cache directory.

By default, the cache is stored in `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on Unix and `%LOCALAPPDATA%\fyn\cache` on Windows.

When `--no-cache` is used, the cache is stored in a temporary directory and discarded when the process exits.

An alternative cache directory may be specified via the `cache-dir` setting, the `--cache-dir` option, or the `$UV_CACHE_DIR` environment variable.

Note that it is important for performance for the cache directory to be located on the same file system as the Python environment fyn is operating on.

### Usage

```
fyn cache dir [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-cache-dir--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-cache-dir--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-cache-dir--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-cache-dir--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-cache-dir--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-cache-dir--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-cache-dir--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-cache-dir--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-cache-dir--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-cache-dir--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-cache-dir--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-cache-dir--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-cache-dir--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-cache-dir--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-cache-dir--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-cache-dir--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-cache-dir--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn cache size](#fyn-cache-size)

Show the cache size.

Displays the total size of the cache directory. This includes all downloaded and built wheels, source distributions, and other cached data. By default, outputs the size in raw bytes; use `--human` for human-readable output.

### Usage

```
fyn cache size [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-cache-size--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-cache-size--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-cache-size--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-cache-size--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-cache-size--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-cache-size--help), `-h` : Display the concise help for this command

[`--human`](#fyn-cache-size--human), `--human-readable`, `-H` : Display the cache size in human-readable format (e.g., `1.2 GiB` instead of raw bytes)

[`--managed-python`](#fyn-cache-size--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-cache-size--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-cache-size--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-cache-size--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-cache-size--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-cache-size--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-cache-size--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-cache-size--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-cache-size--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-cache-size--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-cache-size--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn status](#fyn-status)

Show the current project and environment status

### Usage

```
fyn status [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-status--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-status--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--check`](#fyn-status--check) : Fail if obvious project status checks do not pass

[`--color`](#fyn-status--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-status--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-status--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-status--help), `-h` : Display the concise help for this command

[`--json`](#fyn-status--json) : Display the status in JSON format

[`--managed-python`](#fyn-status--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-status--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-status--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-status--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-status--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-status--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-status--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-status--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-status--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-status--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-status--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn self](#fyn-self)

Manage the fyn executable

### Usage

```
fyn self [OPTIONS] <COMMAND>
```

### Commands

[`fyn self update`](#fyn-self-update) : Update fyn

[`fyn self version`](#fyn-self-version) : Display fyn's version

### [fyn self update](#fyn-self-update)

Update fyn

### Usage

```
fyn self update [OPTIONS] [TARGET_VERSION]
```

### Arguments

[TARGET_VERSION](#fyn-self-update--target_version) : Update to the specified version. If not provided, fyn will update to the latest version

### Options

[`--allow-insecure-host`](#fyn-self-update--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-self-update--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-self-update--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-self-update--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-self-update--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--dry-run`](#fyn-self-update--dry-run) : Run without performing the update

[`--help`](#fyn-self-update--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-self-update--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-self-update--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-self-update--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-self-update--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-self-update--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-self-update--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-self-update--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-self-update--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-self-update--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-self-update--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--token`](#fyn-self-update--token) *token* : A GitHub token for authentication. A token is not required but can be used to reduce the chance of encountering rate limits

```
May also be set with the `UV_GITHUB_TOKEN` environment variable.
```

[`--verbose`](#fyn-self-update--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

### [fyn self version](#fyn-self-version)

Display fyn's version

### Usage

```
fyn self version [OPTIONS]
```

### Options

[`--allow-insecure-host`](#fyn-self-version--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-self-version--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-self-version--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-self-version--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-self-version--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-self-version--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-self-version--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-self-version--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-self-version--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-self-version--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-self-version--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-progress`](#fyn-self-version--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-self-version--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-self-version--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--output-format`](#fyn-self-version--output-format) *output-format*

[`--project`](#fyn-self-version--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-self-version--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--short`](#fyn-self-version--short) : Only print the version

[`--verbose`](#fyn-self-version--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```

## [fyn generate-shell-completion](#fyn-generate-shell-completion)

Generate shell completion

### Usage

```
fyn generate-shell-completion [OPTIONS] <SHELL>
```

### Arguments

[SHELL](#fyn-generate-shell-completion--shell) : The shell to generate the completion script for

### Options

[`--allow-insecure-host`](#fyn-generate-shell-completion--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--directory`](#fyn-generate-shell-completion--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--managed-python`](#fyn-generate-shell-completion--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--no-managed-python`](#fyn-generate-shell-completion--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--project`](#fyn-generate-shell-completion--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

## [fyn help](#fyn-help)

Display documentation for a command

### Usage

```
fyn help [OPTIONS] [COMMAND]...
```

### Arguments

[COMMAND](#fyn-help--command)

### Options

[`--allow-insecure-host`](#fyn-help--allow-insecure-host), `--trusted-host` *allow-insecure-host* : Allow insecure connections to a host.

```
Can be provided multiple times.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

May also be set with the `UV_INSECURE_HOST` environment variable.
```

[`--cache-dir`](#fyn-help--cache-dir) *cache-dir* : Path to the cache directory.

```
Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on macOS and Linux, and `%LOCALAPPDATA%\fyn\cache` on Windows.

To view the location of the cache directory, run `fyn cache dir`.

May also be set with the `UV_CACHE_DIR` environment variable.
```

[`--color`](#fyn-help--color) *color-choice* : Control the use of color in output.

```
By default, fyn will automatically detect support for colors when writing to a terminal.

Possible values:

- `auto`: Enables colored output only when the output is going to a terminal or TTY with support
- `always`: Enables colored output regardless of the detected environment
- `never`: Disables colored output
```

[`--config-file`](#fyn-help--config-file) *config-file* : The path to a `fyn.toml` file to use for configuration.

```
While fyn configuration can be included in a `pyproject.toml` file, it is not allowed in this context.

May also be set with the `UV_CONFIG_FILE` environment variable.
```

[`--directory`](#fyn-help--directory) *directory* : Change to the given directory prior to running the command.

```
Relative paths are resolved with the given directory as the base.

See `--project` to only change the project root directory.

May also be set with the `UV_WORKING_DIR` environment variable.
```

[`--help`](#fyn-help--help), `-h` : Display the concise help for this command

[`--managed-python`](#fyn-help--managed-python) : Require use of fyn-managed Python versions [env: UV_MANAGED_PYTHON=]

```
By default, fyn prefers using Python versions it manages. However, it will use system Python versions if a fyn-managed Python is not installed. This option disables use of system Python versions.
```

[`--native-tls`](#fyn-help--native-tls), `--system-certs` : Whether to load TLS certificates from the platform's native store [env: UV_NATIVE_TLS=]

```
By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.
```

[`--no-cache`](#fyn-help--no-cache), `--no-cache-dir`, `-n` : Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation

```
May also be set with the `UV_NO_CACHE` environment variable.
```

[`--no-config`](#fyn-help--no-config) : Avoid discovering configuration files (`pyproject.toml`, `fyn.toml`).

```
Normally, configuration files are discovered in the current directory, parent directories, or user configuration directories.

May also be set with the `UV_NO_CONFIG` environment variable.
```

[`--no-managed-python`](#fyn-help--no-managed-python) : Disable use of fyn-managed Python versions [env: UV_NO_MANAGED_PYTHON=]

```
Instead, fyn will search for a suitable Python version on the system.
```

[`--no-pager`](#fyn-help--no-pager) : Disable pager when printing help

[`--no-progress`](#fyn-help--no-progress) : Hide all progress outputs [env: UV_NO_PROGRESS=]

```
For example, spinners or progress bars.
```

[`--no-python-downloads`](#fyn-help--no-python-downloads) : Disable automatic downloads of Python.

[`--offline`](#fyn-help--offline) : Disable network access [env: UV_OFFLINE=]

```
When disabled, fyn will only use locally cached data and locally available files.
```

[`--project`](#fyn-help--project) *project* : Discover a project in the given directory.

```
All `pyproject.toml`, `fyn.toml`, and `.python-version` files will be discovered by walking up the directory tree from the project root, as will the project's virtual environment (`.venv`).

Other command-line arguments (such as relative paths) will be resolved relative to the current working directory.

See `--directory` to change the working directory entirely.

This setting has no effect when used in the `fyn pip` interface.

May also be set with the `UV_PROJECT` environment variable.
```

[`--quiet`](#fyn-help--quiet), `-q` : Use quiet output.

```
Repeating this option, e.g., `-qq`, will enable a silent mode in which fyn will write no output to stdout.
```

[`--verbose`](#fyn-help--verbose), `-v` : Use verbose output.

```
You can configure fine-grained logging using the `RUST_LOG` environment variable. (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
```
