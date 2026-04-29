# [Running commands in projects](#running-commands-in-projects)

When working on a project, it is installed into the virtual environment at `.venv`. This environment is isolated from the current shell by default, so invocations that require the project, e.g., `python -c "import example"`, will fail. Instead, use `fyn run` to run commands in the project environment:

```
$ fyn run python -c "import example"
```

When using `run`, fyn will ensure that the project environment is up-to-date before running the given command.

The given command can be provided by the project environment or exist outside of it, e.g.:

```
$ # Presuming the project provides `example-cli`
$ fyn run example-cli foo

$ # Running a `bash` script that requires the project to be available
$ fyn run bash scripts/foo.sh
```

## [Running project tasks](#running-project-tasks)

Projects can define named tasks in `pyproject.toml` under `[tool.fyn.tasks]` and invoke them with `fyn run <name>`:

pyproject.toml

```
[tool.fyn.tasks]
test = { cmd = "pytest -q", env = { PYTHONWARNINGS = "error" } }
lint = "ruff check ."
check = { chain = ["lint", "test"], description = "Run lint and tests" }
```

```
$ fyn run test
$ fyn run check
$ fyn run --list-tasks
```

Tasks support two forms:

- a command string, such as `test = "pytest -q"`
- a table with `cmd`, `chain`, `description`, and `env`

Chained tasks run their child tasks in sequence and stop on the first failure. Task `env` values are applied to the spawned command. If a chained task defines `env`, those values are inherited by its child tasks, and any child task values override the parent values.

Additional CLI arguments are supported for `cmd` tasks:

```
$ fyn run test -- -k my_test
```

Additional CLI arguments are not supported for chained tasks; run the child task directly when you need to pass extra arguments.

## [Missing commands](#missing-commands)

If task resolution succeeds but the external command still cannot be spawned, fyn augments the error with the most likely next step instead of only showing the raw OS error.

- For projects with tasks, a missing bare command suggests `fyn run --list-tasks`.
- For bare executables that may come from Python packages, it suggests `fyn tool run <command>`.
- For path-like commands such as `./script`, it reminds you to check that the path exists relative to the current directory.

For example:

```
$ fyn run tesst
error: Failed to spawn: `tesst`
  Caused by: No such file or directory (os error 2)

hint: If you meant to run a task, use `fyn run --list-tasks` to inspect available tasks.

hint: If `tesst` is provided by a Python package, try `fyn tool run tesst`.
```

## [Requesting additional dependencies](#requesting-additional-dependencies)

Additional dependencies or different versions of dependencies can be requested per invocation.

The `--with` option is used to include a dependency for the invocation, e.g., to request a different version of `httpx`:

```
$ fyn run --with httpx==0.26.0 python -c "import httpx; print(httpx.__version__)"
0.26.0
$ fyn run --with httpx==0.25.0 python -c "import httpx; print(httpx.__version__)"
0.25.0
```

The requested version will be respected regardless of the project's requirements. For example, even if the project requires `httpx==0.24.0`, the output above would be the same.

## [Running scripts](#running-scripts)

Scripts that declare inline metadata are automatically executed in environments isolated from the project. See the [scripts guide](../../../guides/scripts/#declaring-script-dependencies) for more details.

For example, given a script:

example.py

```
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

The invocation `fyn run example.py` would run *isolated* from the project with only the given dependencies listed.

## [Legacy scripts on Windows](#legacy-scripts-on-windows)

Support is provided for [legacy setuptools scripts](https://packaging.python.org/en/latest/guides/distributing-packages-using-setuptools/#scripts). These types of scripts are additional files installed by setuptools in `.venv\Scripts`.

Currently only legacy scripts with the `.ps1`, `.cmd`, and `.bat` extensions are supported.

For example, below is an example running a Command Prompt script.

```
$ fyn run --with nuitka==2.6.7 -- nuitka.cmd --version
```

In addition, you don't need to specify the extension. `fyn` will automatically look for files ending in `.ps1`, `.cmd`, and `.bat` in that order of execution on your behalf.

```
$ fyn run --with nuitka==2.6.7 -- nuitka --version
```

## [Signal handling](#signal-handling)

fyn does not cede control of the process to the spawned command in order to provide better error messages on failure. Consequently, fyn is responsible for forwarding some signals to the child process the requested command runs in.

On Unix systems, fyn will forward most signals (with the exception of SIGKILL, SIGCHLD, SIGIO, and SIGPOLL) to the child process. Since terminals send SIGINT to the foreground process group on Ctrl-C, fyn will only forward a SIGINT to the child process if it is sent more than once or the child process group differs from fyn's.

On Windows, these concepts do not apply and fyn ignores Ctrl-C events, deferring handling to the child process so it can exit cleanly.
