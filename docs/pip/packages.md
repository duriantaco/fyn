# Managing packages

## Installing a package

To install a package into the virtual environment, e.g., Flask:

```console
$ fyn pip install flask
```

To install a package with optional dependencies enabled, e.g., Flask with the "dotenv" extra:

```console
$ fyn pip install "flask[dotenv]"
```

To install multiple packages, e.g., Flask and Ruff:

```console
$ fyn pip install flask ruff
```

To install a package with a constraint, e.g., Ruff v0.2.0 or newer:

```console
$ fyn pip install 'ruff>=0.2.0'
```

To install a package at a specific version, e.g., Ruff v0.3.0:

```console
$ fyn pip install 'ruff==0.3.0'
```

To install a package from the disk:

```console
$ fyn pip install "ruff @ ./projects/ruff"
```

To install a package from GitHub:

```console
$ fyn pip install "git+https://github.com/astral-sh/ruff"
```

To install a package from GitHub at a specific reference:

```console
$ # Install a tag
$ fyn pip install "git+https://github.com/astral-sh/ruff@v0.2.0"

$ # Install a commit
$ fyn pip install "git+https://github.com/astral-sh/ruff@1fadefa67b26508cc59cf38e6130bde2243c929d"

$ # Install a branch
$ fyn pip install "git+https://github.com/astral-sh/ruff@main"
```

See the [Git authentication](../concepts/authentication/git.md) documentation for installation from
a private repository.

## Editable packages

Editable packages do not need to be reinstalled for changes to their source code to be active.

To install the current project as an editable package

```console
$ fyn pip install -e .
```

To install a project in another directory as an editable package:

```console
$ fyn pip install -e "ruff @ ./project/ruff"
```

## Installing packages from files

Multiple packages can be installed at once from standard file formats.

Install from a `requirements.txt` file:

```console
$ fyn pip install -r requirements.txt
```

See the [`fyn pip compile`](./compile.md) documentation for more information on `requirements.txt`
files.

Install from a `pyproject.toml` file:

```console
$ fyn pip install -r pyproject.toml
```

Install from a `pyproject.toml` file with optional dependencies enabled, e.g., the "foo" extra:

```console
$ fyn pip install -r pyproject.toml --extra foo
```

Install from a `pyproject.toml` file with all optional dependencies enabled:

```console
$ fyn pip install -r pyproject.toml --all-extras
```

To install dependency groups in the current project directory's `pyproject.toml`, for example the
group `foo`:

```console
$ fyn pip install --group foo
```

To specify the project directory where groups should be sourced from:

```console
$ fyn pip install --project some/path/ --group foo --group bar
```

Alternatively, you can specify a path to a `pyproject.toml` for each group:

```console
$ fyn pip install --group some/path/pyproject.toml:foo --group other/pyproject.toml:bar
```

!!! note

    As in pip, `--group` flags do not apply to other sources specified with flags like `-r` or `-e`.
    For instance, `fyn pip install -r some/path/pyproject.toml --group foo` sources `foo`
    from `./pyproject.toml` and **not** `some/path/pyproject.toml`.

## Uninstalling a package

To uninstall a package, e.g., Flask:

```console
$ fyn pip uninstall flask
```

To uninstall multiple packages, e.g., Flask and Ruff:

```console
$ fyn pip uninstall flask ruff
```
