# [Installing Python](#installing-python)

If Python is already installed on your system, fyn will [detect and use](#using-existing-python-versions) it without configuration. However, fyn can also install and manage Python versions. fyn [automatically installs](#automatic-python-downloads) missing Python versions as needed — you don't need to install Python to get started.

## [Getting started](#getting-started)

To install the latest Python version:

```
$ fyn python install
```

Note

Python does not publish official distributable binaries. As such, fyn uses distributions from the Astral [`python-build-standalone`](https://github.com/astral-sh/python-build-standalone) project. See the [Python distributions](../../concepts/python-versions/#managed-python-distributions) documentation for more details.

Once Python is installed, it will be used by `fyn` commands automatically. fyn also adds the installed version to your `PATH`:

```
$ python3.13
```

fyn only installs a *versioned* executable by default. To install `python` and `python3` executables, include the `--default` option:

```
$ fyn python install --default
```

Tip

See the documentation on [installing Python executables](../../concepts/python-versions/#installing-python-executables) for more details.

## [Installing a specific version](#installing-a-specific-version)

To install a specific Python version:

```
$ fyn python install 3.12
```

To install multiple Python versions:

```
$ fyn python install 3.11 3.12
```

To install an alternative Python implementation, e.g., PyPy:

```
$ fyn python install pypy@3.10
```

See the [`python install`](../../concepts/python-versions/#installing-a-python-version) documentation for more details.

## [Reinstalling Python](#reinstalling-python)

To reinstall fyn-managed Python versions, use `--reinstall`, e.g.:

```
$ fyn python install --reinstall
```

This will reinstall all previously installed Python versions. Improvements are constantly being added to the Python distributions, so reinstalling may resolve bugs even if the Python version does not change.

## [Viewing Python installations](#viewing-python-installations)

To view available and installed Python versions:

```
$ fyn python list
```

See the [`python list`](../../concepts/python-versions/#viewing-available-python-versions) documentation for more details.

If you maintain your own catalog of downloadable Python builds, `fyn python install`, `find`, `list`, `upgrade`, and `pin` accept `--python-downloads-json-url <source>`.

## [Automatic Python downloads](#automatic-python-downloads)

Python does not need to be explicitly installed to use fyn. By default, fyn will automatically download Python versions when they are required. For example, the following would download Python 3.12 if it was not installed:

```
$ fynx python@3.12 -c "print('hello world')"
```

Even if a specific Python version is not requested, fyn will download the latest version on demand. For example, if there are no Python versions on your system, the following will install Python before creating a new virtual environment:

```
$ fyn venv
```

Tip

Automatic Python downloads can be [easily disabled](../../concepts/python-versions/#disabling-automatic-python-downloads) if you want more control over when Python is downloaded.

Note that when an automatic Python installation occurs, the `python` command will not be added to the shell. Use `fyn python install-shim` to ensure the `python` shim is installed.

## [Using existing Python versions](#using-existing-python-versions)

fyn will use existing Python installations if present on your system. There is no configuration necessary for this behavior: fyn will use the system Python if it satisfies the requirements of the command invocation. See the [Python discovery](../../concepts/python-versions/#discovery-of-python-versions) documentation for details.

To force fyn to use the system Python, provide the `--no-managed-python` flag. See the [Python version preference](../../concepts/python-versions/#requiring-or-disabling-managed-python-versions) documentation for more details.

## [Upgrading Python versions](#upgrading-python-versions)

Important

Support for upgrading Python patch versions is in *preview*. This means the behavior is experimental and subject to change.

To upgrade a Python version to the latest supported patch release:

```
$ fyn python upgrade 3.12
```

To upgrade all fyn-managed Python versions:

```
$ fyn python upgrade
```

See the [`python upgrade`](../../concepts/python-versions/#upgrading-python-versions) documentation for more details.

## [Next steps](#next-steps)

To learn more about `fyn python`, see the [Python version concept](../../concepts/python-versions/) page and the [command reference](../../reference/cli/#fyn-python).

Or, read on to learn how to [run scripts](../scripts/) and invoke Python with fyn.
