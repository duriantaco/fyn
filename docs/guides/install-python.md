---
title: Installing and managing Python
description:
  A guide to using fyn to install Python, including requesting specific versions, automatic
  installation, viewing installed versions, and more.
---

# Installing Python

If Python is already installed on your system, fyn will
[detect and use](#using-existing-python-versions) it without configuration. However, fyn can also
install and manage Python versions. fyn [automatically installs](#automatic-python-downloads)
missing Python versions as needed — you don't need to install Python to get started.

## Getting started

To install the latest Python version:

```console
$ fyn python install
```

!!! note

    Python does not publish official distributable binaries. As such, uv uses distributions from the Astral [`python-build-standalone`](https://github.com/astral-sh/python-build-standalone) project. See the [Python distributions](../concepts/python-versions.md#managed-python-distributions) documentation for more details.

Once Python is installed, it will be used by `fyn` commands automatically. fyn also adds the
installed version to your `PATH`:

```console
$ python3.13
```

fyn only installs a _versioned_ executable by default. To install `python` and `python3`
executables, include the `--default` option:

```console
$ fyn python install --default
```

!!! tip

    See the documentation on [installing Python executables](../concepts/python-versions.md#installing-python-executables)
    for more details.

## Installing a specific version

To install a specific Python version:

```console
$ fyn python install 3.12
```

To install multiple Python versions:

```console
$ fyn python install 3.11 3.12
```

To install an alternative Python implementation, e.g., PyPy:

```console
$ fyn python install pypy@3.10
```

See the [`python install`](../concepts/python-versions.md#installing-a-python-version) documentation
for more details.

## Reinstalling Python

To reinstall fyn-managed Python versions, use `--reinstall`, e.g.:

```console
$ fyn python install --reinstall
```

This will reinstall all previously installed Python versions. Improvements are constantly being
added to the Python distributions, so reinstalling may resolve bugs even if the Python version does
not change.

## Viewing Python installations

To view available and installed Python versions:

```console
$ fyn python list
```

See the [`python list`](../concepts/python-versions.md#viewing-available-python-versions)
documentation for more details.

## Automatic Python downloads

Python does not need to be explicitly installed to use fyn. By default, fyn will automatically
download Python versions when they are required. For example, the following would download Python
3.12 if it was not installed:

```console
$ fynx python@3.12 -c "print('hello world')"
```

Even if a specific Python version is not requested, fyn will download the latest version on demand.
For example, if there are no Python versions on your system, the following will install Python
before creating a new virtual environment:

```console
$ fyn venv
```

!!! tip

    Automatic Python downloads can be [easily disabled](../concepts/python-versions.md#disabling-automatic-python-downloads) if you want more control over when Python is downloaded.

<!-- TODO(zanieb): Restore when Python shim management is added
Note that when an automatic Python installation occurs, the `python` command will not be added to the shell. Use `fyn python install-shim` to ensure the `python` shim is installed.
-->

## Using existing Python versions

fyn will use existing Python installations if present on your system. There is no configuration
necessary for this behavior: fyn will use the system Python if it satisfies the requirements of the
command invocation. See the
[Python discovery](../concepts/python-versions.md#discovery-of-python-versions) documentation for
details.

To force fyn to use the system Python, provide the `--no-managed-python` flag. See the
[Python version preference](../concepts/python-versions.md#requiring-or-disabling-managed-python-versions)
documentation for more details.

## Upgrading Python versions

!!! important

    Support for upgrading Python patch versions is in _preview_. This means the behavior is
    experimental and subject to change.

To upgrade a Python version to the latest supported patch release:

```console
$ fyn python upgrade 3.12
```

To upgrade all fyn-managed Python versions:

```console
$ fyn python upgrade
```

See the [`python upgrade`](../concepts/python-versions.md#upgrading-python-versions) documentation
for more details.

## Next steps

To learn more about `fyn python`, see the [Python version concept](../concepts/python-versions.md)
page and the [command reference](../reference/cli.md#fyn-python).

Or, read on to learn how to [run scripts](./scripts.md) and invoke Python with fyn.
