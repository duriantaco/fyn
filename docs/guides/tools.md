---
title: Using tools
description:
  A guide to using fv to run tools published as Python packages, including one-off invocations with
  fvx, requesting specific tool versions, installing tools, upgrading tools, and more.
---

# Using tools

Many Python packages provide applications that can be used as tools. fv has specialized support for
easily invoking and installing tools.

## Running tools

The `fvx` command invokes a tool without installing it.

For example, to run `ruff`:

```console
$ fvx ruff
```

!!! note

    This is exactly equivalent to:

    ```console
    $ fv tool run ruff
    ```

    `fvx` is provided as an alias for convenience.

Arguments can be provided after the tool name:

```console
$ fvx pycowsay hello from fv

  -------------
< hello from fv >
  -------------
   \   ^__^
    \  (oo)\_______
       (__)\       )\/\
           ||----w |
           ||     ||

```

Tools are installed into temporary, isolated environments when using `fvx`.

!!! note

    If you are running a tool in a [_project_](../concepts/projects/index.md) and the tool requires that
    your project is installed, e.g., when using `pytest` or `mypy`, you'll want to use
    [`fv run`](./projects.md#running-commands) instead of `fvx`. Otherwise, the tool will be run in
    a virtual environment that is isolated from your project.

    If your project has a flat structure, e.g., instead of using a `src` directory for modules,
    the project itself does not need to be installed and `fvx` is fine. In this case, using
    `fv run` is only beneficial if you want to pin the version of the tool in the project's
    dependencies.

## Commands with different package names

When `fvx ruff` is invoked, fv installs the `ruff` package which provides the `ruff` command.
However, sometimes the package and command names differ.

The `--from` option can be used to invoke a command from a specific package, e.g., `http` which is
provided by `httpie`:

```console
$ fvx --from httpie http
```

## Requesting specific versions

To run a tool at a specific version, use `command@<version>`:

```console
$ fvx ruff@0.3.0 check
```

To run a tool at the latest version, use `command@latest`:

```console
$ fvx ruff@latest check
```

The `--from` option can also be used to specify package versions, as above:

```console
$ fvx --from 'ruff==0.3.0' ruff check
```

Or, to constrain to a range of versions:

```console
$ fvx --from 'ruff>0.2.0,<0.3.0' ruff check
```

Note the `@` syntax cannot be used for anything other than an exact version.

## Requesting extras

The `--from` option can be used to run a tool with extras:

```console
$ fvx --from 'mypy[faster-cache,reports]' mypy --xml-report mypy_report
```

This can also be combined with version selection:

```console
$ fvx --from 'mypy[faster-cache,reports]==1.13.0' mypy --xml-report mypy_report
```

## Requesting different sources

The `--from` option can also be used to install from alternative sources.

For example, to pull from git:

```console
$ fvx --from git+https://github.com/httpie/cli httpie
```

You can also pull the latest commit from a specific named branch:

```console
$ fvx --from git+https://github.com/httpie/cli@master httpie
```

Or pull a specific tag:

```console
$ fvx --from git+https://github.com/httpie/cli@3.2.4 httpie
```

Or even a specific commit:

```console
$ fvx --from git+https://github.com/httpie/cli@2843b87 httpie
```

Or with [Git LFS](https://git-lfs.com) support:

```console
$ uvx --lfs --from git+https://github.com/astral-sh/lfs-cowsay lfs-cowsay
```

## Commands with plugins

Additional dependencies can be included, e.g., to include `mkdocs-material` when running `mkdocs`:

```console
$ fvx --with mkdocs-material mkdocs --help
```

## Installing tools

If a tool is used often, it is useful to install it to a persistent environment and add it to the
`PATH` instead of invoking `fvx` repeatedly.

!!! tip

    `fvx` is a convenient alias for `fv tool run`. All of the other commands for interacting with
    tools require the full `fv tool` prefix.

To install `ruff`:

```console
$ fv tool install ruff
```

When a tool is installed, its executables are placed in a `bin` directory in the `PATH` which allows
the tool to be run without fv. If it's not on the `PATH`, a warning will be displayed and
`fv tool update-shell` can be used to add it to the `PATH`.

After installing `ruff`, it should be available:

```console
$ ruff --version
```

Unlike `fv pip install`, installing a tool does not make its modules available in the current
environment. For example, the following command will fail:

```console
$ python -c "import ruff"
```

This isolation is important for reducing interactions and conflicts between dependencies of tools,
scripts, and projects.

Unlike `fvx`, `fv tool install` operates on a _package_ and will install all executables provided by
the tool.

For example, the following will install the `http`, `https`, and `httpie` executables:

```console
$ fv tool install httpie
```

Additionally, package versions can be included without `--from`:

```console
$ fv tool install 'httpie>0.1.0'
```

And, similarly, for package sources:

```console
$ fv tool install git+https://github.com/httpie/cli
```

Or package sources with [Git LFS](https://git-lfs.com):

```console
$ fv tool install --lfs git+https://github.com/astral-sh/lfs-cowsay
```

As with `fvx`, installations can include additional packages:

```console
$ fv tool install mkdocs --with mkdocs-material
```

Multiple related executables can be installed together in the same tool environment, using the
`--with-executables-from` flag. For example, the following will install the executables from
`ansible`, plus those ones provided by `ansible-core` and `ansible-lint`:

```console
$ fv tool install --with-executables-from ansible-core,ansible-lint ansible
```

## Upgrading tools

To upgrade a tool, use `fv tool upgrade`:

```console
$ fv tool upgrade ruff
```

Tool upgrades will respect the version constraints provided when installing the tool. For example,
`fv tool install ruff >=0.3,<0.4` followed by `fv tool upgrade ruff` will upgrade Ruff to the latest
version in the range `>=0.3,<0.4`.

To instead replace the version constraints, re-install the tool with `fv tool install`:

```console
$ fv tool install ruff>=0.4
```

To instead upgrade all tools:

```console
$ fv tool upgrade --all
```

## Requesting Python versions

By default, fv will use your default Python interpreter (the first it finds) when running,
installing, or upgrading tools. You can specify the Python interpreter to use with the `--python`
option.

For example, to request a specific Python version when running a tool:

```console
$ fvx --python 3.10 ruff
```

Or, when installing a tool:

```console
$ fv tool install --python 3.10 ruff
```

Or, when upgrading a tool:

```console
$ fv tool upgrade --python 3.10 ruff
```

For more details on requesting Python versions, see the
[Python version](../concepts/python-versions.md#requesting-a-version) concept page.

## Legacy Windows Scripts

Tools also support running
[legacy setuptools scripts](https://packaging.python.org/en/latest/guides/distributing-packages-using-setuptools/#scripts).
These scripts are available via `$(fv tool dir)\<tool-name>\Scripts` when installed.

Currently only legacy scripts with the `.ps1`, `.cmd`, and `.bat` extensions are supported.

For example, below is an example running a Command Prompt script.

```console
$ fv tool run --from nuitka==2.6.7 nuitka.cmd --version
```

In addition, you don't need to specify the extension. `fvx` will automatically look for files ending
in `.ps1`, `.cmd`, and `.bat` in that order of execution on your behalf.

```console
$ fv tool run --from nuitka==2.6.7 nuitka --version
```

## Next steps

To learn more about managing tools with fv, see the [Tools concept](../concepts/tools.md) page and
the [command reference](../reference/cli.md#fv-tool).

Or, read on to learn how to [work on projects](./projects.md).
