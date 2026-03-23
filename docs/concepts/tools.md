# Tools

Tools are Python packages that provide command-line interfaces.

!!! note

    See the [tools guide](../guides/tools.md) for an introduction to working with the tools
    interface — this document discusses details of tool management.

## The `fv tool` interface

fv includes a dedicated interface for interacting with tools. Tools can be invoked without
installation using `fv tool run`, in which case their dependencies are installed in a temporary
virtual environment isolated from the current project.

Because it is very common to run tools without installing them, a `fvx` alias is provided for
`fv tool run` — the two commands are exactly equivalent. For brevity, the documentation will mostly
refer to `fvx` instead of `fv tool run`.

Tools can also be installed with `fv tool install`, in which case their executables are
[available on the `PATH`](#tool-executables) — an isolated virtual environment is still used, but it
is not removed when the command completes.

## Execution vs installation

In most cases, executing a tool with `fvx` is more appropriate than installing the tool. Installing
the tool is useful if you need the tool to be available to other programs on your system, e.g., if
some script you do not control requires the tool, or if you are in a Docker image and want to make
the tool available to users.

## Tool environments

When running a tool with `fvx`, a virtual environment is stored in the fv cache directory and is
treated as disposable, i.e., if you run `fv cache clean` the environment will be deleted. The
environment is only cached to reduce the overhead of repeated invocations. If the environment is
removed, a new one will be created automatically.

When installing a tool with `fv tool install`, a virtual environment is created in the
[fv tools directory](../reference/storage.md#tools). The environment will not be removed unless the
tool is uninstalled. If the environment is manually deleted, the tool will fail to run.

!!! important

    Tool environments are _not_ intended to be mutated directly. It is strongly recommended never to
    mutate a tool environment manually, e.g., with a `pip` operation.

## Tool versions

Unless a specific version is requested, `fv tool install` will install the latest available of the
requested tool. `fvx` will use the latest available version of the requested tool _on the first
invocation_. After that, `fvx` will use the cached version of the tool unless a different version is
requested, the cache is pruned, or the cache is refreshed.

For example, to run a specific version of Ruff:

```console
$ fvx ruff@0.6.0 --version
ruff 0.6.0
```

A subsequent invocation of `fvx` will use the latest, not the cached, version.

```console
$ fvx ruff --version
ruff 0.6.2
```

But, if a new version of Ruff was released, it would not be used unless the cache was refreshed.

To request the latest version of Ruff and refresh the cache, use the `@latest` suffix:

```console
$ fvx ruff@latest --version
0.6.2
```

Once a tool is installed with `fv tool install`, `fvx` will use the installed version by default.

For example, after installing an older version of Ruff:

```console
$ fv tool install ruff==0.5.0
```

The version of `ruff` and `fvx ruff` is the same:

```console
$ ruff --version
ruff 0.5.0
$ fvx ruff --version
ruff 0.5.0
```

However, you can ignore the installed version by requesting the latest version explicitly, e.g.:

```console
$ fvx ruff@latest --version
0.6.2
```

Or, by using the `--isolated` flag, which will avoid refreshing the cache but ignore the installed
version:

```console
$ fvx --isolated ruff --version
0.6.2
```

`fv tool install` will also respect the `{package}@{version}` and `{package}@latest` specifiers, as
in:

```console
$ fv tool install ruff@latest
$ fv tool install ruff@0.6.0
```

## Upgrading tools

Tool environments may be upgraded via `fv tool upgrade`, or re-created entirely via subsequent
`fv tool install` operations.

To upgrade all packages in a tool environment

```console
$ fv tool upgrade black
```

To upgrade a single package in a tool environment:

```console
$ fv tool upgrade black --upgrade-package click
```

Tool upgrades will respect the version constraints provided when installing the tool. For example,
`fv tool install black >=23,<24` followed by `fv tool upgrade black` will upgrade Black to the
latest version in the range `>=23,<24`.

To instead replace the version constraints, reinstall the tool with `fv tool install`:

```console
$ fv tool install black>=24
```

Similarly, tool upgrades will retain the settings provided when installing the tool. For example,
`fv tool install black --prerelease allow` followed by `fv tool upgrade black` will retain the
`--prerelease allow` setting.

!!! note

    Tool upgrades will reinstall the tool executables, even if they have not changed.

To reinstall packages during upgrade, use the `--reinstall` and `--reinstall-package` options.

To reinstall all packages in a tool environment

```console
$ fv tool upgrade black --reinstall
```

To reinstall a single package in a tool environment:

```console
$ fv tool upgrade black --reinstall-package click
```

## Including additional dependencies

Additional packages can be included during tool execution:

```console
$ fvx --with <extra-package> <tool>
```

And, during tool installation:

```console
$ fv tool install --with <extra-package> <tool-package>
```

The `--with` option can be provided multiple times to include additional packages.

The `--with` option supports package specifications, so a specific version can be requested:

```console
$ fvx --with <extra-package>==<version> <tool-package>
```

The `-w` shorthand can be used in place of the `--with` option:

```console
$ fvx -w <extra-package> <tool-package>
```

If the requested version conflicts with the requirements of the tool package, package resolution
will fail and the command will error.

## Installing executables from additional packages

When installing a tool, you may want to include executables from additional packages in the same
tool environment. This is useful when you have related tools that work together or when you want to
install multiple executables that share dependencies.

The `--with-executables-from` option allows you to specify additional packages whose executables
should be installed alongside the main tool:

```console
$ fv tool install --with-executables-from <package1>,<package2> <tool-package>
```

For example, to install Ansible along with executables from `ansible-core` and `ansible-lint`:

```console
$ fv tool install --with-executables-from ansible-core,ansible-lint ansible
```

This will install all executables from the `ansible`, `ansible-core`, and `ansible-lint` packages
into the same tool environment, making them all available on the `PATH`.

The `--with-executables-from` option can be combined with other installation options:

```console
$ fv tool install --with-executables-from ansible-core --with mkdocs-material ansible
```

Note that `--with-executables-from` differs from `--with` in that:

- `--with` includes additional packages as dependencies but does not install their executables
- `--with-executables-from` includes both the packages as dependencies and installs their
  executables

## Python versions

Each tool environment is linked to a specific Python version. This uses the same Python version
[discovery logic](./python-versions.md#discovery-of-python-versions) as other virtual environments
created by fv, but will ignore non-global Python version requests like `.python-version` files and
the `requires-python` value from a `pyproject.toml`.

The `--python` option can be used to request a specific version. See the
[Python version](./python-versions.md) documentation for more details.

If the Python version used by a tool is _uninstalled_, the tool environment will be broken and the
tool may be unusable.

## Tool executables

Tool executables include all console entry points, script entry points, and binary scripts provided
by a Python package. Tool executables are symlinked into the
[executable directory](../reference/storage.md#tool-executables) on Unix and copied on Windows.

!!! note

    Executables provided by dependencies of tool packages are not installed.

The [executable directory](../reference/storage.md#executable-directory) must be in the `PATH`
variable for tool executables to be available from the shell. If it is not in the `PATH`, a warning
will be displayed. The `fv tool update-shell` command can be used to add the executable directory to
the `PATH` in common shell configuration files.

### Overwriting executables

Installation of tools will not overwrite executables in the executable directory that were not
previously installed by fv. For example, if `pipx` has been used to install a tool,
`fv tool install` will fail. The `--force` flag can be used to override this behavior.

## Relationship to `fv run`

The invocation `fv tool run <name>` (or `fvx <name>`) is nearly equivalent to:

```console
$ fv run --no-project --with <name> -- <name>
```

However, there are a couple notable differences when using fv's tool interface:

- The `--with` option is not needed — the required package is inferred from the command name.
- The temporary environment is cached in a dedicated location.
- The `--no-project` flag is not needed — tools are always run isolated from the project.
- If a tool is already installed, `fv tool run` will use the installed version but `fv run` will
  not.

If the tool should not be isolated from the project, e.g., when running `pytest` or `mypy`, then
`fv run` should be used instead of `fv tool run`.
