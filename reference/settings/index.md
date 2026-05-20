## [Project metadata](#project-metadata)

### \[[`build-constraint-dependencies`](#build-constraint-dependencies)\](#build-constraint-dependencies)

Constraints to apply when solving build dependencies.

Build constraints are used to restrict the versions of build dependencies that are selected when building a package during resolution or installation.

Including a package as a constraint will *not* trigger installation of the package during a build; instead, the package must be requested elsewhere in the project's build dependency graph.

Note

In `fyn lock`, `fyn sync`, and `fyn run`, fyn will only read `build-constraint-dependencies` from the `pyproject.toml` at the workspace root, and will ignore any declarations in other workspace members or `fyn.toml` files.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn]
# Ensure that the setuptools v60.0.0 is used whenever a package has a build dependency
# on setuptools.
build-constraint-dependencies = ["setuptools==60.0.0"]
```

______________________________________________________________________

### \[[`conflicts`](#conflicts)\](#conflicts)

Declare collections of extras or dependency groups that are conflicting (i.e., mutually exclusive).

It's useful to declare conflicts when two or more extras have mutually incompatible dependencies. For example, extra `foo` might depend on `numpy==2.0.0` while extra `bar` depends on `numpy==2.1.0`. While these dependencies conflict, it may be the case that users are not expected to activate both `foo` and `bar` at the same time, making it possible to generate a universal resolution for the project despite the incompatibility.

By making such conflicts explicit, fyn can generate a universal resolution for a project, taking into account that certain combinations of extras and groups are mutually exclusive. In exchange, installation will fail if a user attempts to activate both conflicting extras.

**Default value**: `[]`

**Type**: `list[list[dict]]`

**Example usage**:

pyproject.toml

```
[tool.fyn]
# Require that `package[extra1]` and `package[extra2]` are resolved
# in different forks so that they cannot conflict with one another.
conflicts = [
    [
        { extra = "extra1" },
        { extra = "extra2" },
    ]
]

# Require that the dependency groups `group1` and `group2`
# are resolved in different forks so that they cannot conflict
# with one another.
conflicts = [
    [
        { group = "group1" },
        { group = "group2" },
    ]
]
```

______________________________________________________________________

### \[[`constraint-dependencies`](#constraint-dependencies)\](#constraint-dependencies)

Constraints to apply when resolving the project's dependencies.

Constraints are used to restrict the versions of dependencies that are selected during resolution.

Including a package as a constraint will *not* trigger installation of the package on its own; instead, the package must be requested elsewhere in the project's first-party or transitive dependencies.

Note

In `fyn lock`, `fyn sync`, and `fyn run`, fyn will only read `constraint-dependencies` from the `pyproject.toml` at the workspace root, and will ignore any declarations in other workspace members or `fyn.toml` files.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn]
# Ensure that the grpcio version is always less than 1.65, if it's requested by a
# direct or transitive dependency.
constraint-dependencies = ["grpcio<1.65"]
```

______________________________________________________________________

### \[[`default-groups`](#default-groups)\](#default-groups)

The list of `dependency-groups` to install by default.

Can also be the literal `"all"` to default enable all groups.

**Default value**: `["dev"]`

**Type**: `str | list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn]
default-groups = ["docs"]
```

______________________________________________________________________

### \[[`dependency-groups`](#dependency-groups)\](#dependency-groups)

Additional settings for `dependency-groups`.

Currently this can only be used to add `requires-python` constraints to dependency groups (typically to inform fyn that your dev tooling has a higher python requirement than your actual project).

This cannot be used to define dependency groups, use the top-level `[dependency-groups]` table for that.

**Default value**: `[]`

**Type**: `dict`

**Example usage**:

pyproject.toml

```
[tool.fyn.dependency-groups]
my-group = {requires-python = ">=3.12"}
```

______________________________________________________________________

### \[[`dev-dependencies`](#dev-dependencies)\](#dev-dependencies)

The project's development dependencies.

Development dependencies will be installed by default in `fyn run` and `fyn sync`, but will not appear in the project's published metadata.

Use of this field is not recommend anymore. Instead, use the `dependency-groups.dev` field which is a standardized way to declare development dependencies. The contents of `tool.fyn.dev-dependencies` and `dependency-groups.dev` are combined to determine the final requirements of the `dev` dependency group.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn]
dev-dependencies = ["ruff==0.5.0"]
```

______________________________________________________________________

### \[[`environments`](#environments)\](#environments)

A list of supported environments against which to resolve dependencies.

By default, fyn will resolve for all possible environments during a `fyn lock` operation. However, you can restrict the set of supported environments to improve performance and avoid unsatisfiable branches in the solution space.

These environments will also be respected when `fyn pip compile` is invoked with the `--universal` flag.

**Default value**: `[]`

**Type**: `str | list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn]
# Resolve for macOS, but not for Linux or Windows.
environments = ["sys_platform == 'darwin'"]
```

______________________________________________________________________

### \[[`exclude-dependencies`](#exclude-dependencies)\](#exclude-dependencies)

Dependencies to exclude when resolving the project's dependencies.

Excludes are used to prevent a package from being selected during resolution, regardless of whether it's requested by any other package. When a package is excluded, it will be omitted from the dependency list entirely.

Including a package as an exclusion will prevent it from being installed, even if it's requested by transitive dependencies. This can be useful for removing optional dependencies or working around packages with broken dependencies.

Note

In `fyn lock`, `fyn sync`, and `fyn run`, fyn will only read `exclude-dependencies` from the `pyproject.toml` at the workspace root, and will ignore any declarations in other workspace members or `fyn.toml` files.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn]
# Exclude Werkzeug from being installed, even if transitive dependencies request it.
exclude-dependencies = ["werkzeug"]
```

______________________________________________________________________

### \[[`index`](#index)\](#index)

The indexes to use when resolving dependencies.

Accepts either a repository compliant with [PEP 503](https://peps.python.org/pep-0503/) (the simple repository API), or a local directory laid out in the same format.

Indexes are considered in the order in which they're defined, such that the first-defined index has the highest priority. Further, the indexes provided by this setting are given higher priority than any indexes specified via [`index_url`](#index-url) or [`extra_index_url`](#extra-index-url). fyn will only consider the first index that contains a given package, unless an alternative [index strategy](#index-strategy) is specified.

If an index is marked as `explicit = true`, it will be used exclusively for the dependencies that select it explicitly via `[tool.fyn.sources]`, as in:

```
[[tool.fyn.index]]
name = "pytorch"
url = "https://download.pytorch.org/whl/cu121"
explicit = true

[tool.fyn.sources]
torch = { index = "pytorch" }
```

If an index is marked as `default = true`, it will be moved to the end of the prioritized list, such that it is given the lowest priority when resolving packages. Additionally, marking an index as default will disable the PyPI default index.

**Default value**: `[]`

**Type**: `dict`

**Example usage**:

pyproject.toml

```
[[tool.fyn.index]]
name = "pytorch"
url = "https://download.pytorch.org/whl/cu121"
```

______________________________________________________________________

### \[[`managed`](#managed)\](#managed)

Whether the project is managed by fyn. If `false`, fyn will ignore the project when `fyn run` is invoked.

**Default value**: `true`

**Type**: `bool`

**Example usage**:

pyproject.toml

```
[tool.fyn]
managed = false
```

______________________________________________________________________

### \[[`override-dependencies`](#override-dependencies)\](#override-dependencies)

Overrides to apply when resolving the project's dependencies.

Overrides are used to force selection of a specific version of a package, regardless of the version requested by any other package, and regardless of whether choosing that version would typically constitute an invalid resolution.

While constraints are *additive*, in that they're combined with the requirements of the constituent packages, overrides are *absolute*, in that they completely replace the requirements of any constituent packages.

Including a package as an override will *not* trigger installation of the package on its own; instead, the package must be requested elsewhere in the project's first-party or transitive dependencies.

Note

In `fyn lock`, `fyn sync`, and `fyn run`, fyn will only read `override-dependencies` from the `pyproject.toml` at the workspace root, and will ignore any declarations in other workspace members or `fyn.toml` files.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn]
# Always install Werkzeug 2.3.0, regardless of whether transitive dependencies request
# a different version.
override-dependencies = ["werkzeug==2.3.0"]
```

______________________________________________________________________

### \[[`package`](#package)\](#package)

Whether the project should be considered a Python package, or a non-package ("virtual") project.

Packages are built and installed into the virtual environment in editable mode and thus require a build backend, while virtual projects are *not* built or installed; instead, only their dependencies are included in the virtual environment.

Creating a package requires that a `build-system` is present in the `pyproject.toml`, and that the project adheres to a structure that adheres to the build backend's expectations (e.g., a `src` layout).

**Default value**: `true`

**Type**: `bool`

**Example usage**:

pyproject.toml

```
[tool.fyn]
package = false
```

______________________________________________________________________

### \[[`required-environments`](#required-environments)\](#required-environments)

A list of required platforms, for packages that lack source distributions.

When a package does not have a source distribution, it's availability will be limited to the platforms supported by its built distributions (wheels). For example, if a package only publishes wheels for Linux, then it won't be installable on macOS or Windows.

By default, fyn requires each package to include at least one wheel that is compatible with the designated Python version. The `required-environments` setting can be used to ensure that the resulting resolution contains wheels for specific platforms, or fails if no such wheels are available.

While the `environments` setting *limits* the set of environments that fyn will consider when resolving dependencies, `required-environments` *expands* the set of platforms that fyn *must* support when resolving dependencies.

For example, `environments = ["sys_platform == 'darwin'"]` would limit fyn to solving for macOS (and ignoring Linux and Windows). On the other hand, `required-environments = ["sys_platform == 'darwin'"]` would *require* that any package without a source distribution include a wheel for macOS in order to be installable.

**Default value**: `[]`

**Type**: `str | list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn]
# Require that the package is available on the following platforms:
required-environments = [
    # macOS on Apple Silicon (ARM)
    "sys_platform == 'darwin' and platform_machine == 'arm64'",
    # Linux on x86_64 (Intel/AMD)
    "sys_platform == 'linux' and platform_machine == 'x86_64'",
    # Windows on x86_64 (Intel/AMD)
    "sys_platform == 'win32' and platform_machine == 'AMD64'",
]
```

______________________________________________________________________

### \[[`sources`](#sources)\](#sources)

The sources to use when resolving dependencies.

`tool.fyn.sources` enriches the dependency metadata with additional sources, incorporated during development. A dependency source can be a Git repository, a URL, a local path, or an alternative registry.

See [Dependencies](../../concepts/projects/dependencies/) for more.

**Default value**: `{}`

**Type**: `dict`

**Example usage**:

pyproject.toml

```
[tool.fyn.sources]
httpx = { git = "https://github.com/encode/httpx", tag = "0.27.0" }
pytest = { url = "https://files.pythonhosted.org/packages/6b/77/7440a06a8ead44c7757a64362dd22df5760f9b12dc5f11b6188cd2fc27a0/pytest-8.3.3-py3-none-any.whl" }
pydantic = { path = "/path/to/pydantic", editable = true }
```

______________________________________________________________________

### \[[`tasks`](#tasks)\](#tasks)

Project tasks that can be run via `fyn run <name>`.

Each task is either a command string or a table with `cmd`, `chain`, `description`, and `env` fields.

**Default value**: `{}`

**Type**: `dict`

**Example usage**:

pyproject.toml

```
[tool.fyn.tasks]
test = "pytest -xvs"
lint = "ruff check ."
format = { cmd = "ruff format .", description = "Format code" }
check = { chain = ["lint", "test"], description = "Lint then test" }
```

______________________________________________________________________

### [`build-backend`](#build-backend)

Settings for the fyn build backend (`fyn_build`).

Note that those settings only apply when using the `fyn_build` backend, other build backends (such as hatchling) have their own configuration.

All options that accept globs use the portable glob patterns from [PEP 639](https://packaging.python.org/en/latest/specifications/glob-patterns/).

#### \[[`data`](#build-backend_data)\](#build-backend_data)

Data includes for wheels.

Each entry is a directory, whose contents are copied to the matching directory in the wheel in `<name>-<version>.data/(purelib|platlib|headers|scripts|data)`. Upon installation, this data is moved to its target location, as defined by <https://docs.python.org/3.12/library/sysconfig.html#installation-paths>. Usually, small data files are included by placing them in the Python module instead of using data includes.

- `scripts`: Installed to the directory for executables, `<venv>/bin` on Unix or `<venv>\Scripts` on Windows. This directory is added to `PATH` when the virtual environment is activated or when using `fyn run`, so this data type can be used to install additional binaries. Consider using `project.scripts` instead for Python entrypoints.

- `data`: Installed over the virtualenv environment root.

  Warning: This may override existing files!

- `headers`: Installed to the include directory. Compilers building Python packages with this package as build requirement use the include directory to find additional header files.

- `purelib` and `platlib`: Installed to the `site-packages` directory. It is not recommended to use these two options.

**Default value**: `{}`

**Type**: `dict[str, str]`

**Example usage**:

pyproject.toml

```
[tool.fyn.build-backend]
data = { headers = "include/headers", scripts = "bin" }
```

______________________________________________________________________

#### \[[`default-excludes`](#build-backend_default-excludes)\](#build-backend_default-excludes)

If set to `false`, the default excludes aren't applied.

Default excludes: `__pycache__`, `*.pyc`, and `*.pyo`.

**Default value**: `true`

**Type**: `bool`

**Example usage**:

pyproject.toml

```
[tool.fyn.build-backend]
default-excludes = false
```

______________________________________________________________________

#### \[[`module-name`](#build-backend_module-name)\](#build-backend_module-name)

The name of the module directory inside `module-root`.

The default module name is the package name with dots and dashes replaced by underscores.

Package names need to be valid Python identifiers, and the directory needs to contain a `__init__.py`. An exception are stubs packages, whose name ends with `-stubs`, with the stem being the module name, and which contain a `__init__.pyi` file.

For namespace packages with a single module, the path can be dotted, e.g., `foo.bar` or `foo-stubs.bar`.

For namespace packages with multiple modules, the path can be a list, e.g., `["foo", "bar"]`. We recommend using a single module per package, splitting multiple packages into a workspace.

Note that using this option runs the risk of creating two packages with different names but the same module names. Installing such packages together leads to unspecified behavior, often with corrupted files or directory trees.

**Default value**: `None`

**Type**: `str | list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn.build-backend]
module-name = "sklearn"
```

______________________________________________________________________

#### \[[`module-root`](#build-backend_module-root)\](#build-backend_module-root)

The directory that contains the module directory.

Common values are `src` (src layout, the default) or an empty path (flat layout).

**Default value**: `"src"`

**Type**: `str`

**Example usage**:

pyproject.toml

```
[tool.fyn.build-backend]
module-root = ""
```

______________________________________________________________________

#### \[[`namespace`](#build-backend_namespace)\](#build-backend_namespace)

Build a namespace package.

Build a PEP 420 implicit namespace package, allowing more than one root `__init__.py`.

Use this option when the namespace package contains multiple root `__init__.py`, for namespace packages with a single root `__init__.py` use a dotted `module-name` instead.

To compare dotted `module-name` and `namespace = true`, the first example below can be expressed with `module-name = "cloud.database"`: There is one root `__init__.py` `database`. In the second example, we have three roots (`cloud.database`, `cloud.database_pro`, `billing.modules.database_pro`), so `namespace = true` is required.

```
src
└── cloud
    └── database
        ├── __init__.py
        ├── query_builder
        │   └── __init__.py
        └── sql
            ├── parser.py
            └── __init__.py
```

```
src
├── cloud
│   ├── database
│   │   ├── __init__.py
│   │   ├── query_builder
│   │   │   └── __init__.py
│   │   └── sql
│   │       ├── __init__.py
│   │       └── parser.py
│   └── database_pro
│       ├── __init__.py
│       └── query_builder.py
└── billing
    └── modules
        └── database_pro
            ├── __init__.py
            └── sql.py
```

**Default value**: `false`

**Type**: `bool`

**Example usage**:

pyproject.toml

```
[tool.fyn.build-backend]
namespace = true
```

______________________________________________________________________

#### \[[`source-exclude`](#build-backend_source-exclude)\](#build-backend_source-exclude)

Glob expressions which files and directories to exclude from the source distribution.

These exclusions are also applied to wheels to ensure that a wheel built from a source tree is consistent with a wheel built from a source distribution.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn.build-backend]
source-exclude = ["*.bin"]
```

______________________________________________________________________

#### \[[`source-include`](#build-backend_source-include)\](#build-backend_source-include)

Glob expressions which files and directories to additionally include in the source distribution.

`pyproject.toml` and the contents of the module directory are always included.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn.build-backend]
source-include = ["tests/**"]
```

______________________________________________________________________

#### \[[`wheel-exclude`](#build-backend_wheel-exclude)\](#build-backend_wheel-exclude)

Glob expressions which files and directories to exclude from the wheel.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn.build-backend]
wheel-exclude = ["*.bin"]
```

______________________________________________________________________

### [`workspace`](#workspace)

#### \[[`exclude`](#workspace_exclude)\](#workspace_exclude)

Packages to exclude as workspace members. If a package matches both `members` and `exclude`, it will be excluded.

Supports both globs and explicit paths.

For more information on the glob syntax, refer to the [`glob` documentation](https://docs.rs/glob/latest/glob/struct.Pattern.html).

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn.workspace]
exclude = ["member1", "path/to/member2", "libs/*"]
```

______________________________________________________________________

#### \[[`members`](#workspace_members)\](#workspace_members)

Packages to include as workspace members.

Supports both globs and explicit paths.

For more information on the glob syntax, refer to the [`glob` documentation](https://docs.rs/glob/latest/glob/struct.Pattern.html).

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

pyproject.toml

```
[tool.fyn.workspace]
members = ["member1", "path/to/member2", "libs/*"]
```

______________________________________________________________________

## [Configuration](#configuration)

### \[[`add-bounds`](#add-bounds)\](#add-bounds)

The default version specifier when adding a dependency.

When adding a dependency to the project, if no constraint or URL is provided, a constraint is added based on the latest compatible version of the package. By default, a lower bound constraint is used, e.g., `>=1.2.3`.

When `--frozen` is provided, no resolution is performed, and dependencies are always added without constraints.

This option is in preview and may change in any future release.

**Default value**: `"lower"`

**Possible values**:

- `"lower"`: Only a lower bound, e.g., `>=1.2.3`
- `"major"`: Allow the same major version, similar to the semver caret, e.g., `>=1.2.3, <2.0.0`
- `"minor"`: Allow the same minor version, similar to the semver tilde, e.g., `>=1.2.3, <1.3.0`
- `"exact"`: Pin the exact version, e.g., `==1.2.3`

**Example usage**:

```
[tool.fyn]
add-bounds = "major"
```

```
add-bounds = "major"
```

______________________________________________________________________

### \[[`allow-insecure-host`](#allow-insecure-host)\](#allow-insecure-host)

Allow insecure connections to host.

Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g., `localhost:8080`), or a URL (e.g., `https://localhost`).

WARNING: Hosts included in this list will not be verified against the system's certificate store. Only use `--allow-insecure-host` in a secure network with verified sources, as it bypasses SSL verification and could expose you to MITM attacks.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn]
allow-insecure-host = ["localhost:8080"]
```

```
allow-insecure-host = ["localhost:8080"]
```

______________________________________________________________________

### \[[`cache-dir`](#cache-dir)\](#cache-dir)

Path to the cache directory.

Defaults to `$XDG_CACHE_HOME/fyn` or `$HOME/.cache/fyn` on Linux and macOS, and `%LOCALAPPDATA%\fyn\cache` on Windows.

**Default value**: `None`

**Type**: `str`

**Example usage**:

fyn.toml

```
cache-dir = "./.fyn_cache"
```

______________________________________________________________________

### \[[`cache-keys`](#cache-keys)\](#cache-keys)

The keys to consider when caching builds for the project.

Cache keys enable you to specify the files or directories that should trigger a rebuild when modified. By default, fyn will rebuild a project whenever the `pyproject.toml`, `setup.py`, or `setup.cfg` files in the project directory are modified, or if a `src` directory is added or removed, i.e.:

```
cache-keys = [{ file = "pyproject.toml" }, { file = "setup.py" }, { file = "setup.cfg" }, { dir = "src" }]
```

As an example: if a project uses dynamic metadata to read its dependencies from a `requirements.txt` file, you can specify `cache-keys = [{ file = "requirements.txt" }, { file = "pyproject.toml" }]` to ensure that the project is rebuilt whenever the `requirements.txt` file is modified (in addition to watching the `pyproject.toml`).

Globs are supported, following the syntax of the [`glob`](https://docs.rs/glob/0.3.1/glob/struct.Pattern.html) crate. For example, to invalidate the cache whenever a `.toml` file in the project directory or any of its subdirectories is modified, you can specify `cache-keys = [{ file = "**/*.toml" }]`. Note that the use of globs can be expensive, as fyn may need to walk the filesystem to determine whether any files have changed.

Cache keys can also include version control information. For example, if a project uses `setuptools_scm` to read its version from a Git commit, you can specify `cache-keys = [{ git = { commit = true }, { file = "pyproject.toml" }]` to include the current Git commit hash in the cache key (in addition to the `pyproject.toml`). Git tags are also supported via `cache-keys = [{ git = { commit = true, tags = true } }]`.

Cache keys can also include environment variables. For example, if a project relies on `MACOSX_DEPLOYMENT_TARGET` or other environment variables to determine its behavior, you can specify `cache-keys = [{ env = "MACOSX_DEPLOYMENT_TARGET" }]` to invalidate the cache whenever the environment variable changes.

Cache keys only affect the project defined by the `pyproject.toml` in which they're specified (as opposed to, e.g., affecting all members in a workspace), and all paths and globs are interpreted as relative to the project directory.

**Default value**: `[{ file = "pyproject.toml" }, { file = "setup.py" }, { file = "setup.cfg" }]`

**Type**: `list[dict]`

**Example usage**:

```
[tool.fyn]
cache-keys = [{ file = "pyproject.toml" }, { file = "requirements.txt" }, { git = { commit = true } }]
```

```
cache-keys = [{ file = "pyproject.toml" }, { file = "requirements.txt" }, { git = { commit = true } }]
```

______________________________________________________________________

### \[[`check-url`](#check-url)\](#check-url)

Check an index URL for existing files to skip duplicate uploads.

This option allows retrying publishing that failed after only some, but not all files have been uploaded, and handles error due to parallel uploads of the same file.

Before uploading, the index is checked. If the exact same file already exists in the index, the file will not be uploaded. If an error occurred during the upload, the index is checked again, to handle cases where the identical file was uploaded twice in parallel.

The exact behavior will vary based on the index. When uploading to PyPI, uploading the same file succeeds even without `--check-url`, while most other indexes error.

The index must provide one of the supported hashes (SHA-256, SHA-384, or SHA-512).

**Default value**: `None`

**Type**: `str`

**Example usage**:

```
[tool.fyn]
check-url = "https://test.pypi.org/simple"
```

```
check-url = "https://test.pypi.org/simple"
```

______________________________________________________________________

### \[[`compile-bytecode`](#compile-bytecode)\](#compile-bytecode)

Compile Python files to bytecode after installation.

By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn]
compile-bytecode = true
```

```
compile-bytecode = true
```

______________________________________________________________________

### \[[`concurrent-builds`](#concurrent-builds)\](#concurrent-builds)

The maximum number of source distributions that fyn will build concurrently at any given time.

Defaults to the number of available CPU cores.

**Default value**: `None`

**Type**: `int`

**Example usage**:

```
[tool.fyn]
concurrent-builds = 4
```

```
concurrent-builds = 4
```

______________________________________________________________________

### \[[`concurrent-downloads`](#concurrent-downloads)\](#concurrent-downloads)

The maximum number of in-flight concurrent downloads that fyn will perform at any given time.

**Default value**: `50`

**Type**: `int`

**Example usage**:

```
[tool.fyn]
concurrent-downloads = 4
```

```
concurrent-downloads = 4
```

______________________________________________________________________

### \[[`concurrent-installs`](#concurrent-installs)\](#concurrent-installs)

The number of threads used when installing and unzipping packages.

Defaults to the number of available CPU cores.

**Default value**: `None`

**Type**: `int`

**Example usage**:

```
[tool.fyn]
concurrent-installs = 4
```

```
concurrent-installs = 4
```

______________________________________________________________________

### \[[`config-settings`](#config-settings)\](#config-settings)

Settings to pass to the [PEP 517](https://peps.python.org/pep-0517/) build backend, specified as `KEY=VALUE` pairs.

**Default value**: `{}`

**Type**: `dict`

**Example usage**:

```
[tool.fyn]
config-settings = { editable_mode = "compat" }
```

```
config-settings = { editable_mode = "compat" }
```

______________________________________________________________________

### \[[`config-settings-package`](#config-settings-package)\](#config-settings-package)

Settings to pass to the [PEP 517](https://peps.python.org/pep-0517/) build backend for specific packages, specified as `KEY=VALUE` pairs.

Accepts a map from package names to string key-value pairs.

**Default value**: `{}`

**Type**: `dict`

**Example usage**:

```
[tool.fyn]
config-settings-package = { numpy = { editable_mode = "compat" } }
```

```
config-settings-package = { numpy = { editable_mode = "compat" } }
```

______________________________________________________________________

### \[[`dependency-metadata`](#dependency-metadata)\](#dependency-metadata)

Pre-defined static metadata for dependencies of the project (direct or transitive). When provided, enables the resolver to use the specified metadata instead of querying the registry or building the relevant package from source.

Metadata should be provided in adherence with the [Metadata 2.3](https://packaging.python.org/en/latest/specifications/core-metadata/) standard, though only the following fields are respected:

- `name`: The name of the package.
- (Optional) `version`: The version of the package. If omitted, the metadata will be applied to all versions of the package.
- (Optional) `requires-dist`: The dependencies of the package (e.g., `werkzeug>=0.14`).
- (Optional) `requires-python`: The Python version required by the package (e.g., `>=3.10`).
- (Optional) `provides-extra`: The extras provided by the package.

**Default value**: `[]`

**Type**: `list[dict]`

**Example usage**:

```
[tool.fyn]
dependency-metadata = [
    { name = "flask", version = "1.0.0", requires-dist = ["werkzeug"], requires-python = ">=3.6" },
]
```

```
dependency-metadata = [
    { name = "flask", version = "1.0.0", requires-dist = ["werkzeug"], requires-python = ">=3.6" },
]
```

______________________________________________________________________

### \[[`exclude-newer`](#exclude-newer)\](#exclude-newer)

Limit candidate packages to those that were uploaded prior to the given date.

Accepts RFC 3339 timestamps (e.g., `2006-12-02T02:07:43Z`), a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or an ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

**Default value**: `None`

**Type**: `str`

**Example usage**:

```
[tool.fyn]
exclude-newer = "2006-12-02T02:07:43Z"
```

```
exclude-newer = "2006-12-02T02:07:43Z"
```

______________________________________________________________________

### \[[`exclude-newer-package`](#exclude-newer-package)\](#exclude-newer-package)

Limit candidate packages for specific packages to those that were uploaded prior to the given date.

Accepts a dictionary format of `PACKAGE = "DATE"` pairs, where `DATE` is an RFC 3339 timestamp (e.g., `2006-12-02T02:07:43Z`), a "friendly" duration (e.g., `24 hours`, `1 week`, `30 days`), or a ISO 8601 duration (e.g., `PT24H`, `P7D`, `P30D`).

Durations do not respect semantics of the local time zone and are always resolved to a fixed number of seconds assuming that a day is 24 hours (e.g., DST transitions are ignored). Calendar units such as months and years are not allowed.

**Default value**: `None`

**Type**: `dict`

**Example usage**:

```
[tool.fyn]
exclude-newer-package = { tqdm = "2022-04-04T00:00:00Z" }
```

```
exclude-newer-package = { tqdm = "2022-04-04T00:00:00Z" }
```

______________________________________________________________________

### \[[`extra-build-dependencies`](#extra-build-dependencies)\](#extra-build-dependencies)

Additional build dependencies for packages.

This allows extending the PEP 517 build environment for the project's dependencies with additional packages. This is useful for packages that assume the presence of packages like `pip`, and do not declare them as build dependencies.

**Default value**: `[]`

**Type**: `dict`

**Example usage**:

```
[tool.fyn]
extra-build-dependencies = { pytest = ["setuptools"] }
```

```
extra-build-dependencies = { pytest = ["setuptools"] }
```

______________________________________________________________________

### \[[`extra-build-variables`](#extra-build-variables)\](#extra-build-variables)

Extra environment variables to set when building certain packages.

Environment variables will be added to the environment when building the specified packages.

**Default value**: `{}`

**Type**: `dict[str, dict[str, str]]`

**Example usage**:

```
[tool.fyn]
extra-build-variables = { flash-attn = { FLASH_ATTENTION_SKIP_CUDA_BUILD = "TRUE" } }
```

```
extra-build-variables = { flash-attn = { FLASH_ATTENTION_SKIP_CUDA_BUILD = "TRUE" } }
```

______________________________________________________________________

### \[[`extra-index-url`](#extra-index-url)\](#extra-index-url)

Extra URLs of package indexes to use, in addition to `--index-url`.

Accepts either a repository compliant with [PEP 503](https://peps.python.org/pep-0503/) (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by [`index_url`](#index-url) or [`index`](#index) with `default = true`. When multiple indexes are provided, earlier values take priority.

To control fyn's resolution strategy when multiple indexes are present, see [`index_strategy`](#index-strategy).

(Deprecated: use `index` instead.)

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn]
extra-index-url = ["https://download.pytorch.org/whl/cpu"]
```

```
extra-index-url = ["https://download.pytorch.org/whl/cpu"]
```

______________________________________________________________________

### \[[`find-links`](#find-links)\](#find-links)

Locations to search for candidate distributions, in addition to those found in the registry indexes.

If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn]
find-links = ["https://download.pytorch.org/whl/torch_stable.html"]
```

```
find-links = ["https://download.pytorch.org/whl/torch_stable.html"]
```

______________________________________________________________________

### \[[`fork-strategy`](#fork-strategy)\](#fork-strategy)

The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

**Default value**: `"requires-python"`

**Possible values**:

- `"fewest"`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `"requires-python"`: Optimize for selecting latest supported version of each package, for each supported Python version

**Example usage**:

```
[tool.fyn]
fork-strategy = "fewest"
```

```
fork-strategy = "fewest"
```

______________________________________________________________________

### \[[`http-proxy`](#http-proxy)\](#http-proxy)

The URL of the HTTP proxy to use.

**Default value**: `None`

**Type**: `str`

**Example usage**:

fyn.toml

```
http-proxy = "http://proxy.example.com"
```

______________________________________________________________________

### \[[`https-proxy`](#https-proxy)\](#https-proxy)

The URL of the HTTPS proxy to use.

**Default value**: `None`

**Type**: `str`

**Example usage**:

fyn.toml

```
https-proxy = "https://proxy.example.com"
```

______________________________________________________________________

### \[[`index`](#index)\](#index)

The package indexes to use when resolving dependencies.

Accepts either a repository compliant with [PEP 503](https://peps.python.org/pep-0503/) (the simple repository API), or a local directory laid out in the same format.

Indexes are considered in the order in which they're defined, such that the first-defined index has the highest priority. Further, the indexes provided by this setting are given higher priority than any indexes specified via [`index_url`](#index-url) or [`extra_index_url`](#extra-index-url). fyn will only consider the first index that contains a given package, unless an alternative [index strategy](#index-strategy) is specified.

If an index is marked as `explicit = true`, it will be used exclusively for those dependencies that select it explicitly via `[tool.fyn.sources]`, as in:

```
[[tool.fyn.index]]
name = "pytorch"
url = "https://download.pytorch.org/whl/cu121"
explicit = true

[tool.fyn.sources]
torch = { index = "pytorch" }
```

If an index is marked as `default = true`, it will be moved to the end of the prioritized list, such that it is given the lowest priority when resolving packages. Additionally, marking an index as default will disable the PyPI default index.

**Default value**: `"[]"`

**Type**: `dict`

**Example usage**:

```
[[tool.fyn.index]]
name = "pytorch"
url = "https://download.pytorch.org/whl/cu121"
```

```
[[tool.fyn.index]]
name = "pytorch"
url = "https://download.pytorch.org/whl/cu121"
```

______________________________________________________________________

### \[[`index-strategy`](#index-strategy)\](#index-strategy)

The strategy to use when resolving against multiple index URLs.

By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

**Default value**: `"first-index"`

**Possible values**:

- `"first-index"`: Only use results from the first index that returns a match for a given package name
- `"unsafe-first-match"`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `"unsafe-best-match"`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index

**Example usage**:

```
[tool.fyn]
index-strategy = "unsafe-best-match"
```

```
index-strategy = "unsafe-best-match"
```

______________________________________________________________________

### \[[`index-url`](#index-url)\](#index-url)

The URL of the Python package index (by default: <https://pypi.org/simple>).

Accepts either a repository compliant with [PEP 503](https://peps.python.org/pep-0503/) (the simple repository API), or a local directory laid out in the same format.

The index provided by this setting is given lower priority than any indexes specified via [`extra_index_url`](#extra-index-url) or [`index`](#index).

(Deprecated: use `index` instead.)

**Default value**: `"https://pypi.org/simple"`

**Type**: `str`

**Example usage**:

```
[tool.fyn]
index-url = "https://test.pypi.org/simple"
```

```
index-url = "https://test.pypi.org/simple"
```

______________________________________________________________________

### \[[`keyring-provider`](#keyring-provider)\](#keyring-provider)

Attempt to use `keyring` for authentication for index URLs.

At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

**Default value**: `"disabled"`

**Type**: `str`

**Example usage**:

```
[tool.fyn]
keyring-provider = "subprocess"
```

```
keyring-provider = "subprocess"
```

______________________________________________________________________

### \[[`link-mode`](#link-mode)\](#link-mode)

The method to use when installing packages from the global cache.

Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

**Default value**: `"clone" (macOS, Linux) or "hardlink" (Windows)`

**Possible values**:

- `"clone"`: Clone (i.e., copy-on-write) packages from the source into the destination
- `"copy"`: Copy packages from the source into the destination
- `"hardlink"`: Hard link packages from the source into the destination
- `"symlink"`: Symbolically link packages from the source into the destination

**Example usage**:

```
[tool.fyn]
link-mode = "copy"
```

```
link-mode = "copy"
```

______________________________________________________________________

### \[[`native-tls`](#native-tls)\](#native-tls)

Whether to load TLS certificates from the platform's native certificate store.

By default, fyn loads certificates from the bundled `webpki-roots` crate. The `webpki-roots` are a reliable set of trust roots from Mozilla, and including them in fyn improves portability and performance (especially on macOS).

However, in some cases, you may want to use the platform's native certificate store, especially if you're relying on a corporate trust root (e.g., for a mandatory proxy) that's included in your system's certificate store.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

fyn.toml

```
native-tls = true
```

______________________________________________________________________

### \[[`no-binary`](#no-binary)\](#no-binary)

Don't install pre-built wheels.

The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn]
no-binary = true
```

```
no-binary = true
```

______________________________________________________________________

### \[[`no-binary-package`](#no-binary-package)\](#no-binary-package)

Don't install pre-built wheels for a specific package.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn]
no-binary-package = ["ruff"]
```

```
no-binary-package = ["ruff"]
```

______________________________________________________________________

### \[[`no-build`](#no-build)\](#no-build)

Don't build source distributions.

When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn]
no-build = true
```

```
no-build = true
```

______________________________________________________________________

### \[[`no-build-isolation`](#no-build-isolation)\](#no-build-isolation)

Disable isolation when building source distributions.

Assumes that build dependencies specified by [PEP 518](https://peps.python.org/pep-0518/) are already installed.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn]
no-build-isolation = true
```

```
no-build-isolation = true
```

______________________________________________________________________

### \[[`no-build-isolation-package`](#no-build-isolation-package)\](#no-build-isolation-package)

Disable isolation when building source distributions for a specific package.

Assumes that the packages' build dependencies specified by [PEP 518](https://peps.python.org/pep-0518/) are already installed.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn]
no-build-isolation-package = ["package1", "package2"]
```

```
no-build-isolation-package = ["package1", "package2"]
```

______________________________________________________________________

### \[[`no-build-package`](#no-build-package)\](#no-build-package)

Don't build source distributions for a specific package.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn]
no-build-package = ["ruff"]
```

```
no-build-package = ["ruff"]
```

______________________________________________________________________

### \[[`no-cache`](#no-cache)\](#no-cache)

Avoid reading from or writing to the cache, instead using a temporary directory for the duration of the operation.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn]
no-cache = true
```

```
no-cache = true
```

______________________________________________________________________

### \[[`no-index`](#no-index)\](#no-index)

Ignore all registry indexes (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn]
no-index = true
```

```
no-index = true
```

______________________________________________________________________

### \[[`no-proxy`](#no-proxy)\](#no-proxy)

A list of hosts to exclude from proxying.

**Default value**: `None`

**Type**: `list[str]`

**Example usage**:

fyn.toml

```
no-proxy = ["localhost", "127.0.0.1"]
```

______________________________________________________________________

### \[[`no-sources`](#no-sources)\](#no-sources)

Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any local or Git sources.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn]
no-sources = true
```

```
no-sources = true
```

______________________________________________________________________

### \[[`no-sources-package`](#no-sources-package)\](#no-sources-package)

Ignore `tool.fyn.sources` for the specified packages.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn]
no-sources-package = ["ruff"]
```

```
no-sources-package = ["ruff"]
```

______________________________________________________________________

### \[[`offline`](#offline)\](#offline)

Disable network access, relying only on locally cached data and locally available files.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn]
offline = true
```

```
offline = true
```

______________________________________________________________________

### \[[`prerelease`](#prerelease)\](#prerelease)

The strategy to use when considering pre-release versions.

By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

**Default value**: `"if-necessary-or-explicit"`

**Possible values**:

- `"disallow"`: Disallow all pre-release versions
- `"allow"`: Allow all pre-release versions
- `"if-necessary"`: Allow pre-release versions if all versions of a package are pre-release
- `"explicit"`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `"if-necessary-or-explicit"`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements

**Example usage**:

```
[tool.fyn]
prerelease = "allow"
```

```
prerelease = "allow"
```

______________________________________________________________________

### \[[`preview`](#preview)\](#preview)

Whether to enable experimental, preview features.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn]
preview = true
```

```
preview = true
```

______________________________________________________________________

### \[[`publish-url`](#publish-url)\](#publish-url)

The URL for publishing packages to the Python package index (by default: <https://upload.pypi.org/legacy/>).

**Default value**: `"https://upload.pypi.org/legacy/"`

**Type**: `str`

**Example usage**:

```
[tool.fyn]
publish-url = "https://test.pypi.org/legacy/"
```

```
publish-url = "https://test.pypi.org/legacy/"
```

______________________________________________________________________

### \[[`pypy-install-mirror`](#pypy-install-mirror)\](#pypy-install-mirror)

Mirror URL to use for downloading managed PyPy installations.

By default, managed PyPy installations are downloaded from [downloads.python.org](https://downloads.python.org/). This variable can be set to a mirror URL to use a different source for PyPy installations. The provided URL will replace `https://downloads.python.org/pypy` in, e.g., `https://downloads.python.org/pypy/pypy3.8-v7.3.7-osx64.tar.bz2`.

Distributions can be read from a local directory by using the `file://` URL scheme.

**Default value**: `None`

**Type**: `str`

**Example usage**:

fyn.toml

```
pypy-install-mirror = "https://downloads.python.org/pypy"
```

______________________________________________________________________

### \[[`python-downloads`](#python-downloads)\](#python-downloads)

Whether to allow Python downloads.

**Default value**: `"automatic"`

**Possible values**:

- `"automatic"`: Automatically download managed Python installations when needed
- `"manual"`: Do not automatically download managed Python installations; require explicit installation
- `"never"`: Do not ever allow Python downloads

**Example usage**:

```
[tool.fyn]
python-downloads = "manual"
```

```
python-downloads = "manual"
```

______________________________________________________________________

### \[[`python-downloads-json-url`](#python-downloads-json-url)\](#python-downloads-json-url)

URL pointing to JSON of custom Python installations.

**Default value**: `None`

**Type**: `str`

**Example usage**:

fyn.toml

```
python-downloads-json-url = "/etc/fyn/python-downloads.json"
```

______________________________________________________________________

### \[[`python-install-mirror`](#python-install-mirror)\](#python-install-mirror)

Mirror URL for downloading managed Python installations.

By default, managed Python installations are downloaded from [`python-build-standalone`](https://github.com/astral-sh/python-build-standalone). This variable can be set to a mirror URL to use a different source for Python installations. The provided URL will replace `https://github.com/astral-sh/python-build-standalone/releases/download` in, e.g., `https://github.com/astral-sh/python-build-standalone/releases/download/20240713/cpython-3.12.4%2B20240713-aarch64-apple-darwin-install_only.tar.gz`.

Distributions can be read from a local directory by using the `file://` URL scheme.

**Default value**: `None`

**Type**: `str`

**Example usage**:

fyn.toml

```
python-install-mirror = "https://github.com/astral-sh/python-build-standalone/releases/download"
```

______________________________________________________________________

### \[[`python-preference`](#python-preference)\](#python-preference)

Whether to prefer using Python installations that are already present on the system, or those that are downloaded and installed by fyn.

**Default value**: `"managed"`

**Possible values**:

- `"only-managed"`: Only use managed Python installations; never use system Python installations
- `"managed"`: Prefer managed Python installations over system Python installations
- `"system"`: Prefer system Python installations over managed Python installations
- `"only-system"`: Only use system Python installations; never use managed Python installations

**Example usage**:

```
[tool.fyn]
python-preference = "managed"
```

```
python-preference = "managed"
```

______________________________________________________________________

### \[[`reinstall`](#reinstall)\](#reinstall)

Reinstall all packages, regardless of whether they're already installed. Implies `refresh`.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn]
reinstall = true
```

```
reinstall = true
```

______________________________________________________________________

### \[[`reinstall-package`](#reinstall-package)\](#reinstall-package)

Reinstall a specific package, regardless of whether it's already installed. Implies `refresh-package`.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn]
reinstall-package = ["ruff"]
```

```
reinstall-package = ["ruff"]
```

______________________________________________________________________

### \[[`required-version`](#required-version)\](#required-version)

Enforce a requirement on the version of fyn.

If the version of fyn does not meet the requirement at runtime, fyn will exit with an error.

Accepts a [PEP 440](https://peps.python.org/pep-0440/) specifier, like `==0.5.0` or `>=0.5.0`.

**Default value**: `null`

**Type**: `str`

**Example usage**:

```
[tool.fyn]
required-version = ">=0.5.0"
```

```
required-version = ">=0.5.0"
```

______________________________________________________________________

### \[[`resolution`](#resolution)\](#resolution)

The strategy to use when selecting between the different compatible versions for a given package requirement.

By default, fyn will use the latest compatible version of each package (`highest`).

**Default value**: `"highest"`

**Possible values**:

- `"highest"`: Resolve the highest compatible version of each package
- `"lowest"`: Resolve the lowest compatible version of each package
- `"lowest-direct"`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies

**Example usage**:

```
[tool.fyn]
resolution = "lowest-direct"
```

```
resolution = "lowest-direct"
```

______________________________________________________________________

### \[[`torch-backend`](#torch-backend)\](#torch-backend)

The backend to use when fetching packages in the PyTorch ecosystem.

When set, fyn will ignore the configured index URLs for packages in the PyTorch ecosystem, and will instead use the defined backend.

For example, when set to `cpu`, fyn will use the CPU-only PyTorch index; when set to `cu126`, fyn will use the PyTorch index for CUDA 12.6.

The `auto` mode will attempt to detect the appropriate PyTorch index based on the currently installed CUDA drivers.

This setting is only respected by `fyn pip` commands.

This option is in preview and may change in any future release.

**Default value**: `null`

**Type**: `str`

**Example usage**:

```
[tool.fyn]
torch-backend = "auto"
```

```
torch-backend = "auto"
```

______________________________________________________________________

### \[[`trusted-publishing`](#trusted-publishing)\](#trusted-publishing)

Configure trusted publishing.

By default, fyn checks for trusted publishing when running in a supported environment, but ignores it if it isn't configured.

fyn's supported environments for trusted publishing include GitHub Actions and GitLab CI/CD.

**Default value**: `automatic`

**Type**: `str`

**Example usage**:

```
[tool.fyn]
trusted-publishing = "always"
```

```
trusted-publishing = "always"
```

______________________________________________________________________

### \[[`upgrade`](#upgrade)\](#upgrade)

Allow package upgrades, ignoring pinned versions in any existing output file.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn]
upgrade = true
```

```
upgrade = true
```

______________________________________________________________________

### \[[`upgrade-package`](#upgrade-package)\](#upgrade-package)

Allow upgrades for a specific package, ignoring pinned versions in any existing output file.

Accepts both standalone package names (`ruff`) and version specifiers (`ruff<0.5.0`).

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn]
upgrade-package = ["ruff"]
```

```
upgrade-package = ["ruff"]
```

______________________________________________________________________

### [`audit`](#audit)

#### \[[`ignore`](#audit_ignore)\](#audit_ignore)

A list of vulnerability IDs to ignore during auditing.

Vulnerabilities matching any of the provided IDs (including aliases) will be excluded from the audit results.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.audit]
ignore = ["PYSEC-2022-43017", "GHSA-5239-wwwm-4pmq"]
```

```
[audit]
ignore = ["PYSEC-2022-43017", "GHSA-5239-wwwm-4pmq"]
```

______________________________________________________________________

#### \[[`ignore-until-fixed`](#audit_ignore-until-fixed)\](#audit_ignore-until-fixed)

A list of vulnerability IDs to ignore during auditing, but only while no fix is available.

Vulnerabilities matching any of the provided IDs (including aliases) will be excluded from the audit results as long as they have no known fix versions. Once a fix version becomes available, the vulnerability will be reported again.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.audit]
ignore-until-fixed = ["PYSEC-2022-43017"]
```

```
[audit]
ignore-until-fixed = ["PYSEC-2022-43017"]
```

______________________________________________________________________

### [`pip`](#pip)

Settings that are specific to the `fyn pip` command-line interface.

These values will be ignored when running commands outside the `fyn pip` namespace (e.g., `fyn lock`, `fynx`).

#### \[[`all-extras`](#pip_all-extras)\](#pip_all-extras)

Include all optional dependencies.

Only applies to `pyproject.toml`, `setup.py`, and `setup.cfg` sources.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
all-extras = true
```

```
[pip]
all-extras = true
```

______________________________________________________________________

#### \[[`allow-empty-requirements`](#pip_allow-empty-requirements)\](#pip_allow-empty-requirements)

Allow `fyn pip sync` with empty requirements, which will clear the environment of all packages.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
allow-empty-requirements = true
```

```
[pip]
allow-empty-requirements = true
```

______________________________________________________________________

#### \[[`annotation-style`](#pip_annotation-style)\](#pip_annotation-style)

The style of the annotation comments included in the output file, used to indicate the source of each package.

**Default value**: `"split"`

**Possible values**:

- `"line"`: Render the annotations on a single, comma-separated line
- `"split"`: Render each annotation on its own line

**Example usage**:

```
[tool.fyn.pip]
annotation-style = "line"
```

```
[pip]
annotation-style = "line"
```

______________________________________________________________________

#### \[[`break-system-packages`](#pip_break-system-packages)\](#pip_break-system-packages)

Allow fyn to modify an `EXTERNALLY-MANAGED` Python installation.

WARNING: `--break-system-packages` is intended for use in continuous integration (CI) environments, when installing into Python installations that are managed by an external package manager, like `apt`. It should be used with caution, as such Python installations explicitly recommend against modifications by other package managers (like fyn or pip).

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
break-system-packages = true
```

```
[pip]
break-system-packages = true
```

______________________________________________________________________

#### \[[`compile-bytecode`](#pip_compile-bytecode)\](#pip_compile-bytecode)

Compile Python files to bytecode after installation.

By default, fyn does not compile Python (`.py`) files to bytecode (`__pycache__/*.pyc`); instead, compilation is performed lazily the first time a module is imported. For use-cases in which start time is critical, such as CLI applications and Docker containers, this option can be enabled to trade longer installation times for faster start times.

When enabled, fyn will process the entire site-packages directory (including packages that are not being modified by the current operation) for consistency. Like pip, it will also ignore errors.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
compile-bytecode = true
```

```
[pip]
compile-bytecode = true
```

______________________________________________________________________

#### \[[`config-settings`](#pip_config-settings)\](#pip_config-settings)

Settings to pass to the [PEP 517](https://peps.python.org/pep-0517/) build backend, specified as `KEY=VALUE` pairs.

**Default value**: `{}`

**Type**: `dict`

**Example usage**:

```
[tool.fyn.pip]
config-settings = { editable_mode = "compat" }
```

```
[pip]
config-settings = { editable_mode = "compat" }
```

______________________________________________________________________

#### \[[`config-settings-package`](#pip_config-settings-package)\](#pip_config-settings-package)

Settings to pass to the [PEP 517](https://peps.python.org/pep-0517/) build backend for specific packages, specified as `KEY=VALUE` pairs.

**Default value**: `{}`

**Type**: `dict`

**Example usage**:

```
[tool.fyn.pip]
config-settings-package = { numpy = { editable_mode = "compat" } }
```

```
[pip]
config-settings-package = { numpy = { editable_mode = "compat" } }
```

______________________________________________________________________

#### \[[`custom-compile-command`](#pip_custom-compile-command)\](#pip_custom-compile-command)

The header comment to include at the top of the output file generated by `fyn pip compile`.

Used to reflect custom build scripts and commands that wrap `fyn pip compile`.

**Default value**: `None`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
custom-compile-command = "./custom-fyn-compile.sh"
```

```
[pip]
custom-compile-command = "./custom-fyn-compile.sh"
```

______________________________________________________________________

#### \[[`dependency-metadata`](#pip_dependency-metadata)\](#pip_dependency-metadata)

Pre-defined static metadata for dependencies of the project (direct or transitive). When provided, enables the resolver to use the specified metadata instead of querying the registry or building the relevant package from source.

Metadata should be provided in adherence with the [Metadata 2.3](https://packaging.python.org/en/latest/specifications/core-metadata/) standard, though only the following fields are respected:

- `name`: The name of the package.
- (Optional) `version`: The version of the package. If omitted, the metadata will be applied to all versions of the package.
- (Optional) `requires-dist`: The dependencies of the package (e.g., `werkzeug>=0.14`).
- (Optional) `requires-python`: The Python version required by the package (e.g., `>=3.10`).
- (Optional) `provides-extra`: The extras provided by the package.

**Default value**: `[]`

**Type**: `list[dict]`

**Example usage**:

```
[tool.fyn.pip]
dependency-metadata = [
    { name = "flask", version = "1.0.0", requires-dist = ["werkzeug"], requires-python = ">=3.6" },
]
```

```
[pip]
dependency-metadata = [
    { name = "flask", version = "1.0.0", requires-dist = ["werkzeug"], requires-python = ">=3.6" },
]
```

______________________________________________________________________

#### \[[`emit-build-options`](#pip_emit-build-options)\](#pip_emit-build-options)

Include `--no-binary` and `--only-binary` entries in the output file generated by `fyn pip compile`.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
emit-build-options = true
```

```
[pip]
emit-build-options = true
```

______________________________________________________________________

#### \[[`emit-find-links`](#pip_emit-find-links)\](#pip_emit-find-links)

Include `--find-links` entries in the output file generated by `fyn pip compile`.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
emit-find-links = true
```

```
[pip]
emit-find-links = true
```

______________________________________________________________________

#### \[[`emit-index-annotation`](#pip_emit-index-annotation)\](#pip_emit-index-annotation)

Include comment annotations indicating the index used to resolve each package (e.g., `# from https://pypi.org/simple`).

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
emit-index-annotation = true
```

```
[pip]
emit-index-annotation = true
```

______________________________________________________________________

#### \[[`emit-index-url`](#pip_emit-index-url)\](#pip_emit-index-url)

Include `--index-url` and `--extra-index-url` entries in the output file generated by `fyn pip compile`.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
emit-index-url = true
```

```
[pip]
emit-index-url = true
```

______________________________________________________________________

#### \[[`emit-marker-expression`](#pip_emit-marker-expression)\](#pip_emit-marker-expression)

Whether to emit a marker string indicating the conditions under which the set of pinned dependencies is valid.

The pinned dependencies may be valid even when the marker expression is false, but when the expression is true, the requirements are known to be correct.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
emit-marker-expression = true
```

```
[pip]
emit-marker-expression = true
```

______________________________________________________________________

#### \[[`exclude-newer`](#pip_exclude-newer)\](#pip_exclude-newer)

Limit candidate packages to those that were uploaded prior to a given point in time.

Accepts a superset of [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (e.g., `2006-12-02T02:07:43Z`). A full timestamp is required to ensure that the resolver will behave consistently across timezones.

**Default value**: `None`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
exclude-newer = "2006-12-02T02:07:43Z"
```

```
[pip]
exclude-newer = "2006-12-02T02:07:43Z"
```

______________________________________________________________________

#### \[[`exclude-newer-package`](#pip_exclude-newer-package)\](#pip_exclude-newer-package)

Limit candidate packages for specific packages to those that were uploaded prior to the given date.

Accepts package-date pairs in a dictionary format.

**Default value**: `None`

**Type**: `dict`

**Example usage**:

```
[tool.fyn.pip]
exclude-newer-package = { tqdm = "2022-04-04T00:00:00Z" }
```

```
[pip]
exclude-newer-package = { tqdm = "2022-04-04T00:00:00Z" }
```

______________________________________________________________________

#### \[[`extra`](#pip_extra)\](#pip_extra)

Include optional dependencies from the specified extra; may be provided more than once.

Only applies to `pyproject.toml`, `setup.py`, and `setup.cfg` sources.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
extra = ["dev", "docs"]
```

```
[pip]
extra = ["dev", "docs"]
```

______________________________________________________________________

#### \[[`extra-build-dependencies`](#pip_extra-build-dependencies)\](#pip_extra-build-dependencies)

Additional build dependencies for packages.

This allows extending the PEP 517 build environment for the project's dependencies with additional packages. This is useful for packages that assume the presence of packages like `pip`, and do not declare them as build dependencies.

**Default value**: `[]`

**Type**: `dict`

**Example usage**:

```
[tool.fyn.pip]
extra-build-dependencies = { pytest = ["setuptools"] }
```

```
[pip]
extra-build-dependencies = { pytest = ["setuptools"] }
```

______________________________________________________________________

#### \[[`extra-build-variables`](#pip_extra-build-variables)\](#pip_extra-build-variables)

Extra environment variables to set when building certain packages.

Environment variables will be added to the environment when building the specified packages.

**Default value**: `{}`

**Type**: `dict[str, dict[str, str]]`

**Example usage**:

```
[tool.fyn.pip]
extra-build-variables = { flash-attn = { FLASH_ATTENTION_SKIP_CUDA_BUILD = "TRUE" } }
```

```
[pip]
extra-build-variables = { flash-attn = { FLASH_ATTENTION_SKIP_CUDA_BUILD = "TRUE" } }
```

______________________________________________________________________

#### \[[`extra-index-url`](#pip_extra-index-url)\](#pip_extra-index-url)

Extra URLs of package indexes to use, in addition to `--index-url`.

Accepts either a repository compliant with [PEP 503](https://peps.python.org/pep-0503/) (the simple repository API), or a local directory laid out in the same format.

All indexes provided via this flag take priority over the index specified by [`index_url`](#index-url). When multiple indexes are provided, earlier values take priority.

To control fyn's resolution strategy when multiple indexes are present, see [`index_strategy`](#index-strategy).

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
extra-index-url = ["https://download.pytorch.org/whl/cpu"]
```

```
[pip]
extra-index-url = ["https://download.pytorch.org/whl/cpu"]
```

______________________________________________________________________

#### \[[`find-links`](#pip_find-links)\](#pip_find-links)

Locations to search for candidate distributions, in addition to those found in the registry indexes.

If a path, the target must be a directory that contains packages as wheel files (`.whl`) or source distributions (e.g., `.tar.gz` or `.zip`) at the top level.

If a URL, the page must contain a flat list of links to package files adhering to the formats described above.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
find-links = ["https://download.pytorch.org/whl/torch_stable.html"]
```

```
[pip]
find-links = ["https://download.pytorch.org/whl/torch_stable.html"]
```

______________________________________________________________________

#### \[[`fork-strategy`](#pip_fork-strategy)\](#pip_fork-strategy)

The strategy to use when selecting multiple versions of a given package across Python versions and platforms.

By default, fyn will optimize for selecting the latest version of each package for each supported Python version (`requires-python`), while minimizing the number of selected versions across platforms.

Under `fewest`, fyn will minimize the number of selected versions for each package, preferring older versions that are compatible with a wider range of supported Python versions or platforms.

**Default value**: `"requires-python"`

**Possible values**:

- `"fewest"`: Optimize for selecting the fewest number of versions for each package. Older versions may be preferred if they are compatible with a wider range of supported Python versions or platforms
- `"requires-python"`: Optimize for selecting latest supported version of each package, for each supported Python version

**Example usage**:

```
[tool.fyn.pip]
fork-strategy = "fewest"
```

```
[pip]
fork-strategy = "fewest"
```

______________________________________________________________________

#### \[[`generate-hashes`](#pip_generate-hashes)\](#pip_generate-hashes)

Include distribution hashes in the output file.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
generate-hashes = true
```

```
[pip]
generate-hashes = true
```

______________________________________________________________________

#### \[[`group`](#pip_group)\](#pip_group)

Include the following dependency groups.

**Default value**: `None`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
group = ["dev", "docs"]
```

```
[pip]
group = ["dev", "docs"]
```

______________________________________________________________________

#### \[[`index-strategy`](#pip_index-strategy)\](#pip_index-strategy)

The strategy to use when resolving against multiple index URLs.

By default, fyn will stop at the first index on which a given package is available, and limit resolutions to those present on that first index (`first-index`). This prevents "dependency confusion" attacks, whereby an attacker can upload a malicious package under the same name to an alternate index.

**Default value**: `"first-index"`

**Possible values**:

- `"first-index"`: Only use results from the first index that returns a match for a given package name
- `"unsafe-first-match"`: Search for every package name across all indexes, exhausting the versions from the first index before moving on to the next
- `"unsafe-best-match"`: Search for every package name across all indexes, preferring the "best" version found. If a package version is in multiple indexes, only look at the entry for the first index

**Example usage**:

```
[tool.fyn.pip]
index-strategy = "unsafe-best-match"
```

```
[pip]
index-strategy = "unsafe-best-match"
```

______________________________________________________________________

#### \[[`index-url`](#pip_index-url)\](#pip_index-url)

The URL of the Python package index (by default: <https://pypi.org/simple>).

Accepts either a repository compliant with [PEP 503](https://peps.python.org/pep-0503/) (the simple repository API), or a local directory laid out in the same format.

The index provided by this setting is given lower priority than any indexes specified via [`extra_index_url`](#extra-index-url).

**Default value**: `"https://pypi.org/simple"`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
index-url = "https://test.pypi.org/simple"
```

```
[pip]
index-url = "https://test.pypi.org/simple"
```

______________________________________________________________________

#### \[[`keyring-provider`](#pip_keyring-provider)\](#pip_keyring-provider)

Attempt to use `keyring` for authentication for index URLs.

At present, only `--keyring-provider subprocess` is supported, which configures fyn to use the `keyring` CLI to handle authentication.

**Default value**: `disabled`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
keyring-provider = "subprocess"
```

```
[pip]
keyring-provider = "subprocess"
```

______________________________________________________________________

#### \[[`link-mode`](#pip_link-mode)\](#pip_link-mode)

The method to use when installing packages from the global cache.

Defaults to `clone` (also known as Copy-on-Write) on macOS and Linux, and `hardlink` on Windows.

WARNING: The use of symlink link mode is discouraged, as they create tight coupling between the cache and the target environment. For example, clearing the cache (`fyn cache clean`) will break all installed packages by way of removing the underlying source files. Use symlinks with caution.

**Default value**: `"clone" (macOS, Linux) or "hardlink" (Windows)`

**Possible values**:

- `"clone"`: Clone (i.e., copy-on-write) packages from the source into the destination
- `"copy"`: Copy packages from the source into the destination
- `"hardlink"`: Hard link packages from the source into the destination
- `"symlink"`: Symbolically link packages from the source into the destination

**Example usage**:

```
[tool.fyn.pip]
link-mode = "copy"
```

```
[pip]
link-mode = "copy"
```

______________________________________________________________________

#### \[[`no-annotate`](#pip_no-annotate)\](#pip_no-annotate)

Exclude comment annotations indicating the source of each package from the output file generated by `fyn pip compile`.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
no-annotate = true
```

```
[pip]
no-annotate = true
```

______________________________________________________________________

#### \[[`no-binary`](#pip_no-binary)\](#pip_no-binary)

Don't install pre-built wheels.

The given packages will be built and installed from source. The resolver will still use pre-built wheels to extract package metadata, if available.

Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
no-binary = ["ruff"]
```

```
[pip]
no-binary = ["ruff"]
```

______________________________________________________________________

#### \[[`no-build`](#pip_no-build)\](#pip_no-build)

Don't build source distributions.

When enabled, resolving will not run arbitrary Python code. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

Alias for `--only-binary :all:`.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
no-build = true
```

```
[pip]
no-build = true
```

______________________________________________________________________

#### \[[`no-build-isolation`](#pip_no-build-isolation)\](#pip_no-build-isolation)

Disable isolation when building source distributions.

Assumes that build dependencies specified by [PEP 518](https://peps.python.org/pep-0518/) are already installed.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
no-build-isolation = true
```

```
[pip]
no-build-isolation = true
```

______________________________________________________________________

#### \[[`no-build-isolation-package`](#pip_no-build-isolation-package)\](#pip_no-build-isolation-package)

Disable isolation when building source distributions for a specific package.

Assumes that the packages' build dependencies specified by [PEP 518](https://peps.python.org/pep-0518/) are already installed.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
no-build-isolation-package = ["package1", "package2"]
```

```
[pip]
no-build-isolation-package = ["package1", "package2"]
```

______________________________________________________________________

#### \[[`no-deps`](#pip_no-deps)\](#pip_no-deps)

Ignore package dependencies, instead only add those packages explicitly listed on the command line to the resulting requirements file.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
no-deps = true
```

```
[pip]
no-deps = true
```

______________________________________________________________________

#### \[[`no-emit-package`](#pip_no-emit-package)\](#pip_no-emit-package)

Specify a package to omit from the output resolution. Its dependencies will still be included in the resolution. Equivalent to pip-compile's `--unsafe-package` option.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
no-emit-package = ["ruff"]
```

```
[pip]
no-emit-package = ["ruff"]
```

______________________________________________________________________

#### \[[`no-extra`](#pip_no-extra)\](#pip_no-extra)

Exclude the specified optional dependencies if `all-extras` is supplied.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
all-extras = true
no-extra = ["dev", "docs"]
```

```
[pip]
all-extras = true
no-extra = ["dev", "docs"]
```

______________________________________________________________________

#### \[[`no-header`](#pip_no-header)\](#pip_no-header)

Exclude the comment header at the top of output file generated by `fyn pip compile`.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
no-header = true
```

```
[pip]
no-header = true
```

______________________________________________________________________

#### \[[`no-index`](#pip_no-index)\](#pip_no-index)

Ignore all registry indexes (e.g., PyPI), instead relying on direct URL dependencies and those provided via `--find-links`.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
no-index = true
```

```
[pip]
no-index = true
```

______________________________________________________________________

#### \[[`no-sources`](#pip_no-sources)\](#pip_no-sources)

Ignore the `tool.fyn.sources` table when resolving dependencies. Used to lock against the standards-compliant, publishable package metadata, as opposed to using any local or Git sources.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
no-sources = true
```

```
[pip]
no-sources = true
```

______________________________________________________________________

#### \[[`no-sources-package`](#pip_no-sources-package)\](#pip_no-sources-package)

Ignore `tool.fyn.sources` for the specified packages.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
no-sources-package = ["ruff"]
```

```
[pip]
no-sources-package = ["ruff"]
```

______________________________________________________________________

#### \[[`no-strip-extras`](#pip_no-strip-extras)\](#pip_no-strip-extras)

Include extras in the output file.

By default, fyn strips extras, as any packages pulled in by the extras are already included as dependencies in the output file directly. Further, output files generated with `--no-strip-extras` cannot be used as constraints files in `install` and `sync` invocations.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
no-strip-extras = true
```

```
[pip]
no-strip-extras = true
```

______________________________________________________________________

#### \[[`no-strip-markers`](#pip_no-strip-markers)\](#pip_no-strip-markers)

Include environment markers in the output file generated by `fyn pip compile`.

By default, fyn strips environment markers, as the resolution generated by `compile` is only guaranteed to be correct for the target environment.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
no-strip-markers = true
```

```
[pip]
no-strip-markers = true
```

______________________________________________________________________

#### \[[`only-binary`](#pip_only-binary)\](#pip_only-binary)

Only use pre-built wheels; don't build source distributions.

When enabled, resolving will not run code from the given packages. The cached wheels of already-built source distributions will be reused, but operations that require building distributions will exit with an error.

Multiple packages may be provided. Disable binaries for all packages with `:all:`. Clear previously specified packages with `:none:`.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
only-binary = ["ruff"]
```

```
[pip]
only-binary = ["ruff"]
```

______________________________________________________________________

#### \[[`output-file`](#pip_output-file)\](#pip_output-file)

Write the requirements generated by `fyn pip compile` to the given `requirements.txt` file.

If the file already exists, the existing versions will be preferred when resolving dependencies, unless `--upgrade` is also specified.

**Default value**: `None`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
output-file = "requirements.txt"
```

```
[pip]
output-file = "requirements.txt"
```

______________________________________________________________________

#### \[[`pip-in-project`](#pip_pip-in-project)\](#pip_pip-in-project)

Whether mutating `fyn pip` commands are allowed inside managed projects.

When set to `warn` (the default), `fyn pip install`, `fyn pip sync`, `fyn pip upgrade`, and `fyn pip uninstall` will emit a warning before modifying the active environment inside a managed project. When set to `error`, those commands will fail instead. When set to `allow`, they will proceed without an additional warning.

**Default value**: `"warn"`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
pip-in-project = "error"
```

```
[pip]
pip-in-project = "error"
```

______________________________________________________________________

#### \[[`prefix`](#pip_prefix)\](#pip_prefix)

Install packages into `lib`, `bin`, and other top-level folders under the specified directory, as if a virtual environment were present at that location.

In general, prefer the use of `--python` to install into an alternate environment, as scripts and other artifacts installed via `--prefix` will reference the installing interpreter, rather than any interpreter added to the `--prefix` directory, rendering them non-portable.

**Default value**: `None`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
prefix = "./prefix"
```

```
[pip]
prefix = "./prefix"
```

______________________________________________________________________

#### \[[`prerelease`](#pip_prerelease)\](#pip_prerelease)

The strategy to use when considering pre-release versions.

By default, fyn will accept pre-releases for packages that *only* publish pre-releases, along with first-party requirements that contain an explicit pre-release marker in the declared specifiers (`if-necessary-or-explicit`).

**Default value**: `"if-necessary-or-explicit"`

**Possible values**:

- `"disallow"`: Disallow all pre-release versions
- `"allow"`: Allow all pre-release versions
- `"if-necessary"`: Allow pre-release versions if all versions of a package are pre-release
- `"explicit"`: Allow pre-release versions for first-party packages with explicit pre-release markers in their version requirements
- `"if-necessary-or-explicit"`: Allow pre-release versions if all versions of a package are pre-release, or if the package has an explicit pre-release marker in its version requirements

**Example usage**:

```
[tool.fyn.pip]
prerelease = "allow"
```

```
[pip]
prerelease = "allow"
```

______________________________________________________________________

#### \[[`python`](#pip_python)\](#pip_python)

The Python interpreter into which packages should be installed.

By default, fyn installs into the virtual environment in the current working directory or any parent directory. The `--python` option allows you to specify a different interpreter, which is intended for use in continuous integration (CI) environments or other automated workflows.

Supported formats:

- `3.10` looks for an installed Python 3.10 in the registry on Windows (see `py --list-paths`), or `python3.10` on Linux and macOS.
- `python3.10` or `python.exe` looks for a binary with the given name in `PATH`.
- `/home/ferris/.local/bin/python3.10` uses the exact Python at the given path.

**Default value**: `None`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
python = "3.10"
```

```
[pip]
python = "3.10"
```

______________________________________________________________________

#### \[[`python-platform`](#pip_python-platform)\](#pip_python-platform)

The platform for which requirements should be resolved.

Represented as a "target triple", a string that describes the target platform in terms of its CPU, vendor, and operating system name, like `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`.

**Default value**: `None`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
python-platform = "x86_64-unknown-linux-gnu"
```

```
[pip]
python-platform = "x86_64-unknown-linux-gnu"
```

______________________________________________________________________

#### \[[`python-version`](#pip_python-version)\](#pip_python-version)

The minimum Python version that should be supported by the resolved requirements (e.g., `3.8` or `3.8.17`).

If a patch version is omitted, the minimum patch version is assumed. For example, `3.8` is mapped to `3.8.0`.

**Default value**: `None`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
python-version = "3.8"
```

```
[pip]
python-version = "3.8"
```

______________________________________________________________________

#### \[[`reinstall`](#pip_reinstall)\](#pip_reinstall)

Reinstall all packages, regardless of whether they're already installed. Implies `refresh`.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
reinstall = true
```

```
[pip]
reinstall = true
```

______________________________________________________________________

#### \[[`reinstall-package`](#pip_reinstall-package)\](#pip_reinstall-package)

Reinstall a specific package, regardless of whether it's already installed. Implies `refresh-package`.

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
reinstall-package = ["ruff"]
```

```
[pip]
reinstall-package = ["ruff"]
```

______________________________________________________________________

#### \[[`require-hashes`](#pip_require-hashes)\](#pip_require-hashes)

Require a matching hash for each requirement.

Hash-checking mode is all or nothing. If enabled, *all* requirements must be provided with a corresponding hash or set of hashes. Additionally, if enabled, *all* requirements must either be pinned to exact versions (e.g., `==1.0.0`), or be specified via direct URL.

Hash-checking mode introduces a number of additional constraints:

- Git dependencies are not supported.
- Editable installations are not supported.
- Local dependencies are not supported, unless they point to a specific wheel (`.whl`) or source archive (`.zip`, `.tar.gz`), as opposed to a directory.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
require-hashes = true
```

```
[pip]
require-hashes = true
```

______________________________________________________________________

#### \[[`resolution`](#pip_resolution)\](#pip_resolution)

The strategy to use when selecting between the different compatible versions for a given package requirement.

By default, fyn will use the latest compatible version of each package (`highest`).

**Default value**: `"highest"`

**Possible values**:

- `"highest"`: Resolve the highest compatible version of each package
- `"lowest"`: Resolve the lowest compatible version of each package
- `"lowest-direct"`: Resolve the lowest compatible version of any direct dependencies, and the highest compatible version of any transitive dependencies

**Example usage**:

```
[tool.fyn.pip]
resolution = "lowest-direct"
```

```
[pip]
resolution = "lowest-direct"
```

______________________________________________________________________

#### \[[`strict`](#pip_strict)\](#pip_strict)

Validate the Python environment, to detect packages with missing dependencies and other issues.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
strict = true
```

```
[pip]
strict = true
```

______________________________________________________________________

#### \[[`system`](#pip_system)\](#pip_system)

Install packages into the system Python environment.

By default, fyn installs into the virtual environment in the current working directory or any parent directory. The `--system` option instructs fyn to instead use the first Python found in the system `PATH`.

WARNING: `--system` is intended for use in continuous integration (CI) environments and should be used with caution, as it can modify the system Python installation.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
system = true
```

```
[pip]
system = true
```

______________________________________________________________________

#### \[[`target`](#pip_target)\](#pip_target)

Install packages into the specified directory, rather than into the virtual or system Python environment. The packages will be installed at the top-level of the directory.

**Default value**: `None`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
target = "./target"
```

```
[pip]
target = "./target"
```

______________________________________________________________________

#### \[[`torch-backend`](#pip_torch-backend)\](#pip_torch-backend)

The backend to use when fetching packages in the PyTorch ecosystem.

When set, fyn will ignore the configured index URLs for packages in the PyTorch ecosystem, and will instead use the defined backend.

For example, when set to `cpu`, fyn will use the CPU-only PyTorch index; when set to `cu126`, fyn will use the PyTorch index for CUDA 12.6.

The `auto` mode will attempt to detect the appropriate PyTorch index based on the currently installed CUDA drivers.

This setting is only respected by `fyn pip` commands.

This option is in preview and may change in any future release.

**Default value**: `null`

**Type**: `str`

**Example usage**:

```
[tool.fyn.pip]
torch-backend = "auto"
```

```
[pip]
torch-backend = "auto"
```

______________________________________________________________________

#### \[[`universal`](#pip_universal)\](#pip_universal)

Perform a universal resolution, attempting to generate a single `requirements.txt` output file that is compatible with all operating systems, architectures, and Python implementations.

In universal mode, the current Python version (or user-provided `--python-version`) will be treated as a lower bound. For example, `--universal --python-version 3.7` would produce a universal resolution for Python 3.7 and later.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
universal = true
```

```
[pip]
universal = true
```

______________________________________________________________________

#### \[[`upgrade`](#pip_upgrade)\](#pip_upgrade)

Allow package upgrades, ignoring pinned versions in any existing output file.

**Default value**: `false`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
upgrade = true
```

```
[pip]
upgrade = true
```

______________________________________________________________________

#### \[[`upgrade-package`](#pip_upgrade-package)\](#pip_upgrade-package)

Allow upgrades for a specific package, ignoring pinned versions in any existing output file.

Accepts both standalone package names (`ruff`) and version specifiers (`ruff<0.5.0`).

**Default value**: `[]`

**Type**: `list[str]`

**Example usage**:

```
[tool.fyn.pip]
upgrade-package = ["ruff"]
```

```
[pip]
upgrade-package = ["ruff"]
```

______________________________________________________________________

#### \[[`verify-hashes`](#pip_verify-hashes)\](#pip_verify-hashes)

Validate any hashes provided in the requirements file.

Unlike `--require-hashes`, `--verify-hashes` does not require that all requirements have hashes; instead, it will limit itself to verifying the hashes of those requirements that do include them.

**Default value**: `true`

**Type**: `bool`

**Example usage**:

```
[tool.fyn.pip]
verify-hashes = true
```

```
[pip]
verify-hashes = true
```

______________________________________________________________________
