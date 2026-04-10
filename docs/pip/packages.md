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

## Guarding dependency installs

Dependency guards are disabled by default. When enabled, fyn evaluates the planned install set
before any host-side prepare or install work begins.

The CLI examples below use the shorter `--guard-*` aliases. Configuration continues to use the full
`dependency-guard-*` setting names.

Use the built-in Socket provider to reject low-reputation registry packages:

```console
$ fyn pip install flask --guard-provider socket --guard-socket-min-score 90
```

Use the external command provider to delegate the decision to another program:

```console
$ fyn pip install flask --guard-provider command --guard-command install-guard
```

The command receives a JSON manifest on stdin describing the planned install. Exit with code `0` to
allow the operation, or any non-zero code to block it.

The top-level payload includes `schema_version`, `socket_min_score`, `remote`, `cached`,
`reinstalls`, and `extraneous`. External commands should treat unknown schema versions as
incompatible and block the install.

You can combine both providers:

```console
$ fyn pip install flask \
    --guard-provider socket \
    --guard-provider command \
    --guard-command install-guard \
    --guard-socket-min-score 90
```

If the guard allows the install, fyn continues with its normal output. If it blocks, fyn exits
before modifying the environment and reports the reason. For example:

```console
$ fyn pip install flask --guard-provider socket --guard-socket-min-score 90
error: Socket dependency guard blocked `pkg:pypi/<name>@<version>` because the score 75 is below the minimum 90
```

```console
$ fyn pip install flask --guard-provider command --guard-command install-guard
error: Dependency guard command `install-guard` blocked installation: suspicious network activity
```

To enable guards by default for the pip interface, configure them in `pyproject.toml`:

```toml title="pyproject.toml"
[tool.fyn.pip]
dependency-guard-provider = ["socket", "command"]
dependency-guard-command = ["install-guard"]
dependency-guard-socket-min-score = 90
```

Or in `fyn.toml`:

```toml title="fyn.toml"
[pip]
dependency-guard-provider = ["socket", "command"]
dependency-guard-command = ["install-guard"]
dependency-guard-socket-min-score = 90
```

To apply the same settings to project-level commands such as `fyn sync`, set them under `[tool.fyn]`
instead of `[tool.fyn.pip]`.

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

## Downloading distribution archives

To download a package archive into the current working directory without installing it:

```console
$ fyn pip download flask
```

To place downloaded archives in a specific directory:

```console
$ fyn pip download flask --dest ./packages
```

To download all archives referenced by a requirements file:

```console
$ fyn pip download -r requirements.txt --dest ./packages
```

To prefer source distributions over wheels:

```console
$ fyn pip download tqdm --no-binary :all: --dest ./packages
```

The first release of `fyn pip download` supports package specs, requirements files, dependency
groups, local wheel and source archive paths, and `--find-links` sources. It does not yet support
Git requirements or local source tree directories.

## Building wheels

To build wheels into the current working directory without installing them:

```console
$ fyn pip wheel flask
```

To place built wheels in a specific directory:

```console
$ fyn pip wheel flask --wheel-dir ./packages
```

To build wheels for the packages referenced by a requirements file:

```console
$ fyn pip wheel -r requirements.txt --wheel-dir ./packages
```

To build a wheel from a local source tree:

```console
$ fyn pip wheel ./some-package --wheel-dir ./packages
```

The first release of `fyn pip wheel` supports package specs, requirements files, dependency groups,
local wheel paths, and local source tree inputs. For source distributions, it builds a wheel into
the target directory instead of saving the original archive.

## Upgrading installed packages

To upgrade all installed packages in the current environment:

```console
$ fyn pip upgrade --all
```

This upgrades the packages already installed in the selected environment. The first release of
`fyn pip upgrade` supports `--all` only.

## Uninstalling a package

To uninstall a package, e.g., Flask:

```console
$ fyn pip uninstall flask
```

To uninstall multiple packages, e.g., Flask and Ruff:

```console
$ fyn pip uninstall flask ruff
```
