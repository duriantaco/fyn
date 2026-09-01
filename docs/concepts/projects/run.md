# Running commands in projects

When working on a project, it is installed into the virtual environment at `.venv`. This environment
is isolated from the current shell by default, so invocations that require the project, e.g.,
`python -c "import example"`, will fail. Instead, use `fyn run` to run commands in the project
environment:

```console
$ fyn run python -c "import example"
```

When using `run`, fyn will ensure that the project environment is up-to-date before running the
given command.

The given command can be provided by the project environment or exist outside of it, e.g.:

```console
$ # Presuming the project provides `example-cli`
$ fyn run example-cli foo

$ # Running a `bash` script that requires the project to be available
$ fyn run bash scripts/foo.sh
```

## Running project tasks

Projects can define named tasks in `pyproject.toml` under `[tool.fyn.tasks]` and invoke them with
`fyn run <name>`:

```toml title="pyproject.toml"
[tool.fyn.tasks]
test = { cmd = "pytest -q", env = { PYTHONWARNINGS = "error" } }
lint = "ruff check ."
check = ["ruff check .", "pytest -q"]
ci = { chain = ["lint", "test"], description = "Run lint and tests" }
```

```console
$ fyn run test
$ fyn run check
$ fyn run ci
$ fyn run --list-tasks
```

Tasks support three forms:

- a command string, such as `test = "pytest -q"`
- a direct sequence of command strings, such as `check = ["ruff check .", "pytest -q"]`
- a table with `cmd`, `chain`, `description`, and `env`

Direct command sequences run each command in the listed order and stop on the first failure. Chained
tasks resolve other named tasks, run them in sequence, and also stop on the first failure. Task
`env` values are applied to the spawned command. If a chained task defines `env`, those values are
inherited by its child tasks, and any child task values override the parent values.

Additional CLI arguments are supported for `cmd` tasks:

```console
$ fyn run test -- -k my_test
```

Additional CLI arguments are not supported for direct command sequences or chained tasks. Define a
`cmd` task, or run the child task directly, when you need to pass extra arguments.

## Running tasks across a workspace

Use `--workspace` to run the same named task in every child workspace member that defines it:

```console
$ fyn run --workspace test
```

Unless `--no-sync` is supplied, fyn synchronizes the shared workspace environment once with all
packages installed before starting any tasks. It then schedules the selected child members according
to active workspace dependencies declared in the current manifests: a dependency's task finishes
before the tasks of members that depend on it. Markers, selected extras and dependency groups,
transitively activated extras, source overrides, and `{ workspace = true }` are taken into account.
Independent members run in parallel by default.

The workspace root is intentionally excluded. It can therefore define an aggregate task that invokes
workspace mode without recursively selecting itself:

```toml title="pyproject.toml"
[tool.fyn.tasks]
test = "fyn run --workspace test"
```

Use `--sequential` to run only one member at a time:

```console
$ fyn run --workspace --sequential test
```

Use the repeatable `--filter` option to select exact package names. A single option can also contain
a comma-separated list:

```console
$ fyn run --workspace --filter api --filter worker test
$ fyn run --workspace --filter api,worker test
```

Filters must name child workspace members, and each filtered member must define the requested task.
Without filters, child members that do not define the task are skipped.

List tasks by child member with:

```console
$ fyn run --workspace --list-tasks
```

See [Using workspaces](./workspaces.md#running-tasks-across-workspace-members) for the full
workspace execution model.

## Missing commands

If task resolution succeeds but the external command still cannot be spawned, fyn augments the error
with the most likely next step instead of only showing the raw OS error.

- For projects with tasks, a missing bare command suggests `fyn run --list-tasks`.
- For bare executables that may come from Python packages, it suggests `fyn tool run <command>`.
- For path-like commands such as `./script`, it reminds you to check that the path exists relative
  to the current directory.

For example:

```console
$ fyn run tesst
error: Failed to spawn: `tesst`
  Caused by: No such file or directory (os error 2)

hint: If you meant to run a task, use `fyn run --list-tasks` to inspect available tasks.

hint: If `tesst` is provided by a Python package, try `fyn tool run tesst`.
```

## Requesting additional dependencies

Additional dependencies or different versions of dependencies can be requested per invocation.

The `--with` option is used to include a dependency for the invocation, e.g., to request a different
version of `httpx`:

```console
$ fyn run --with httpx==0.26.0 python -c "import httpx; print(httpx.__version__)"
0.26.0
$ fyn run --with httpx==0.25.0 python -c "import httpx; print(httpx.__version__)"
0.25.0
```

The requested version will be respected regardless of the project's requirements. For example, even
if the project requires `httpx==0.24.0`, the output above would be the same.

## Running scripts

Scripts that declare inline metadata are automatically executed in environments isolated from the
project. See the [scripts guide](../../guides/scripts.md#declaring-script-dependencies) for more
details.

For example, given a script:

```python title="example.py"
# /// script
# dependencies = [
#   "httpx",
# ]
# ///

import httpx

resp = httpx.get("https://peps.python.org/api/peps.json")
data = resp.json()
print([(k, v["title"]) for k, v in data.items()][:10])
```

The invocation `fyn run example.py` would run _isolated_ from the project with only the given
dependencies listed.

## Legacy scripts on Windows

Support is provided for
[legacy setuptools scripts](https://packaging.python.org/en/latest/guides/distributing-packages-using-setuptools/#scripts).
These types of scripts are additional files installed by setuptools in `.venv\Scripts`.

Currently only legacy scripts with the `.ps1`, `.cmd`, and `.bat` extensions are supported.

For example, below is an example running a Command Prompt script.

```console
$ fyn run --with nuitka==2.6.7 -- nuitka.cmd --version
```

In addition, you don't need to specify the extension. `fyn` will automatically look for files ending
in `.ps1`, `.cmd`, and `.bat` in that order of execution on your behalf.

```console
$ fyn run --with nuitka==2.6.7 -- nuitka --version
```

## Signal handling

fyn does not cede control of the process to the spawned command in order to provide better error
messages on failure. Consequently, fyn is responsible for forwarding some signals to the child
process the requested command runs in.

On Unix systems, fyn will forward most signals (with the exception of SIGKILL, SIGCHLD, SIGIO, and
SIGPOLL) to the child process. Since terminals send SIGINT to the foreground process group on
Ctrl-C, fyn will only forward a SIGINT to the child process if it is sent more than once or the
child process group differs from fyn's.

On Windows, these concepts do not apply and fyn ignores Ctrl-C events, deferring handling to the
child process so it can exit cleanly.
