# [Managing packages](#managing-packages)

## [Installing a package](#installing-a-package)

To install a package into the virtual environment, e.g., Flask:

```
$ fyn pip install flask
```

To install a package with optional dependencies enabled, e.g., Flask with the "dotenv" extra:

```
$ fyn pip install "flask[dotenv]"
```

To install multiple packages, e.g., Flask and Ruff:

```
$ fyn pip install flask ruff
```

To install a package with a constraint, e.g., Ruff v0.2.0 or newer:

```
$ fyn pip install 'ruff>=0.2.0'
```

To install a package at a specific version, e.g., Ruff v0.3.0:

```
$ fyn pip install 'ruff==0.3.0'
```

To install a package from the disk:

```
$ fyn pip install "ruff @ ./projects/ruff"
```

To install a package from GitHub:

```
$ fyn pip install "git+https://github.com/astral-sh/ruff"
```

To install a package from GitHub at a specific reference:

```
$ # Install a tag
$ fyn pip install "git+https://github.com/astral-sh/ruff@v0.2.0"

$ # Install a commit
$ fyn pip install "git+https://github.com/astral-sh/ruff@1fadefa67b26508cc59cf38e6130bde2243c929d"

$ # Install a branch
$ fyn pip install "git+https://github.com/astral-sh/ruff@main"
```

See the [Git authentication](../../concepts/authentication/git/) documentation for installation from a private repository.

## [Editable packages](#editable-packages)

Editable packages do not need to be reinstalled for changes to their source code to be active.

To install the current project as an editable package

```
$ fyn pip install -e .
```

To install a project in another directory as an editable package:

```
$ fyn pip install -e "ruff @ ./project/ruff"
```

## [Installing packages from files](#installing-packages-from-files)

Multiple packages can be installed at once from standard file formats.

Install from a `requirements.txt` file:

```
$ fyn pip install -r requirements.txt
```

See the [`fyn pip compile`](../compile/) documentation for more information on `requirements.txt` files.

Install from a `pyproject.toml` file:

```
$ fyn pip install -r pyproject.toml
```

Install from a `pyproject.toml` file with optional dependencies enabled, e.g., the "foo" extra:

```
$ fyn pip install -r pyproject.toml --extra foo
```

Install from a `pyproject.toml` file with all optional dependencies enabled:

```
$ fyn pip install -r pyproject.toml --all-extras
```

To install dependency groups in the current project directory's `pyproject.toml`, for example the group `foo`:

```
$ fyn pip install --group foo
```

To specify the project directory where groups should be sourced from:

```
$ fyn pip install --project some/path/ --group foo --group bar
```

Alternatively, you can specify a path to a `pyproject.toml` for each group:

```
$ fyn pip install --group some/path/pyproject.toml:foo --group other/pyproject.toml:bar
```

Note

As in pip, `--group` flags do not apply to other sources specified with flags like `-r` or `-e`. For instance, `fyn pip install -r some/path/pyproject.toml --group foo` sources `foo` from `./pyproject.toml` and **not** `some/path/pyproject.toml`.

## [Downloading distribution archives](#downloading-distribution-archives)

To download a package archive into the current working directory without installing it:

```
$ fyn pip download flask
```

To place downloaded archives in a specific directory:

```
$ fyn pip download flask --dest ./packages
```

To download all archives referenced by a requirements file:

```
$ fyn pip download -r requirements.txt --dest ./packages
```

To prefer source distributions over wheels:

```
$ fyn pip download tqdm --no-binary :all: --dest ./packages
```

The first release of `fyn pip download` supports package specs, requirements files, dependency groups, local wheel and source archive paths, and `--find-links` sources. It does not yet support Git requirements or local source tree directories.

## [Building wheels](#building-wheels)

To build wheels into the current working directory without installing them:

```
$ fyn pip wheel flask
```

To place built wheels in a specific directory:

```
$ fyn pip wheel flask --wheel-dir ./packages
```

To build wheels for the packages referenced by a requirements file:

```
$ fyn pip wheel -r requirements.txt --wheel-dir ./packages
```

To build a wheel from a local source tree:

```
$ fyn pip wheel ./some-package --wheel-dir ./packages
```

The first release of `fyn pip wheel` supports package specs, requirements files, dependency groups, local wheel paths, and local source tree inputs. For source distributions, it builds a wheel into the target directory instead of saving the original archive.

## [Upgrading installed packages](#upgrading-installed-packages)

To upgrade all installed packages in the current environment:

```
$ fyn pip upgrade --all
```

This upgrades the packages already installed in the selected environment. The first release of `fyn pip upgrade` supports `--all` only.

## [Uninstalling a package](#uninstalling-a-package)

To uninstall a package, e.g., Flask:

```
$ fyn pip uninstall flask
```

To uninstall multiple packages, e.g., Flask and Ruff:

```
$ fyn pip uninstall flask ruff
```
