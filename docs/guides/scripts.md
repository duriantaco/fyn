---
title: Running scripts
description:
  A guide to using fyn to run Python scripts, including support for inline dependency metadata,
  reproducible scripts, and more.
---

# Running scripts

A Python script is a file intended for standalone execution, e.g., with `python <script>.py`. Using
fyn to execute scripts ensures that script dependencies are managed without manually managing
environments.

!!! note

    If you are not familiar with Python environments: every Python installation has an environment
    that packages can be installed in. Typically, creating [_virtual_ environments](https://docs.python.org/3/library/venv.html) is recommended to
    isolate packages required by each script. fyn automatically manages virtual environments for you
    and prefers a [declarative](#declaring-script-dependencies) approach to dependencies.

## Running a script without dependencies

If your script has no dependencies, you can execute it with `fyn run`:

```python title="example.py"
print("Hello world")
```

```console
$ fyn run example.py
Hello world
```

If you want a plain `python` command that follows fyn's interpreter resolution rules, install the
shim with `fyn python install-shim`.

Similarly, if your script depends on a module in the standard library, there's nothing more to do:

```python title="example.py"
import os

print(os.path.expanduser("~"))
```

```console
$ fyn run example.py
/Users/astral
```

Arguments may be provided to the script:

```python title="example.py"
import sys

print(" ".join(sys.argv[1:]))
```

```console
$ fyn run example.py test
test

$ fyn run example.py hello world!
hello world!
```

Additionally, your script can be read directly from stdin:

```console
$ echo 'print("hello world!")' | fyn run -
```

Or, if your shell supports [here-documents](https://en.wikipedia.org/wiki/Here_document):

```bash
fyn run - <<EOF
print("hello world!")
EOF
```

Note that if you use `fyn run` in a _project_, i.e., a directory with a `pyproject.toml`, it will
install the current project before running the script. If your script does not depend on the
project, use the `--no-project` flag to skip this:

```console
$ # Note: the `--no-project` flag must be provided _before_ the script name.
$ fyn run --no-project example.py
```

See the [projects guide](./projects.md) for more details on working in projects.

## Running a script with dependencies

When your script requires other packages, they must be installed into the environment that the
script runs in. fyn prefers to create these environments on-demand instead of using a long-lived
virtual environment with manually managed dependencies. This requires explicit declaration of
dependencies that are required for the script. Generally, it's recommended to use a
[project](./projects.md) or [inline metadata](#declaring-script-dependencies) to declare
dependencies, but fyn supports requesting dependencies per invocation as well.

For example, the following script requires `rich`.

```python title="example.py"
import time
from rich.progress import track

for i in track(range(20), description="For example:"):
    time.sleep(0.05)
```

If executed without specifying a dependency, this script will fail:

```console
$ fyn run --no-project example.py
Traceback (most recent call last):
  File "/Users/astral/example.py", line 2, in <module>
    from rich.progress import track
ModuleNotFoundError: No module named 'rich'
```

Request the dependency using the `--with` option:

```console
$ fyn run --with rich example.py
For example: ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% 0:00:01
```

Constraints can be added to the requested dependency if specific versions are needed:

```console
$ fyn run --with 'rich>12,<13' example.py
```

Multiple dependencies can be requested by repeating with `--with` option.

Note that if `fyn run` is used in a _project_, these dependencies will be included _in addition_ to
the project's dependencies. To opt-out of this behavior, use the `--no-project` flag.

## Creating a Python script

Python recently added a standard format for
[inline script metadata](https://packaging.python.org/en/latest/specifications/inline-script-metadata/#inline-script-metadata).
It allows for selecting Python versions and defining dependencies. Use `fyn init --script` to
initialize scripts with the inline metadata:

```console
$ fyn init --script example.py --python 3.12
```

## Declaring script dependencies

The inline metadata format allows the dependencies for a script to be declared in the script itself.

fyn supports adding and updating inline script metadata for you. Use `fyn add --script` to declare
the dependencies for the script:

```console
$ fyn add --script example.py 'requests<3' 'rich'
```

This will add a `script` section at the top of the script declaring the dependencies using TOML:

```python title="example.py"
# /// script
# dependencies = [
#   "requests<3",
#   "rich",
# ]
# ///

import requests
from rich.pretty import pprint

resp = requests.get("https://peps.python.org/api/peps.json")
data = resp.json()
pprint([(k, v["title"]) for k, v in data.items()][:10])
```

fyn will automatically create an environment with the dependencies necessary to run the script,
e.g.:

```console
$ fyn run example.py
[
│   ('1', 'PEP Purpose and Guidelines'),
│   ('2', 'Procedure for Adding New Modules'),
│   ('3', 'Guidelines for Handling Bug Reports'),
│   ('4', 'Deprecation of Standard Modules'),
│   ('5', 'Guidelines for Language Evolution'),
│   ('6', 'Bug Fix Releases'),
│   ('7', 'Style Guide for C Code'),
│   ('8', 'Style Guide for Python Code'),
│   ('9', 'Sample Plaintext PEP Template'),
│   ('10', 'Voting Guidelines')
]
```

!!! important

    When using inline script metadata, even if `fyn run` is [used in a _project_](../concepts/projects/run.md), the project's dependencies will be ignored. The `--no-project` flag is not required.

fyn also respects Python version requirements:

```python title="example.py"
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///

# Use some syntax added in Python 3.12
type Point = tuple[float, float]
print(Point)
```

!!! note

    The `dependencies` field must be provided even if empty.

`fyn run` will search for and use the required Python version. The Python version will download if
it is not installed — see the documentation on [Python versions](../concepts/python-versions.md) for
more details.

## Using a shebang to create an executable file

A shebang can be added to make a script executable without using `fyn run` — this makes it easy to
run scripts that are on your `PATH` or in the current folder.

For example, create a file called `greet` with the following contents

```python title="greet"
#!/usr/bin/env -S fyn run --script

print("Hello, world!")
```

Ensure that your script is executable, e.g., with `chmod +x greet`, then run the script:

```console
$ ./greet
Hello, world!
```

Declaration of dependencies is also supported in this context, for example:

```python title="example"
#!/usr/bin/env -S fyn run --script
#
# /// script
# requires-python = ">=3.12"
# dependencies = ["httpx"]
# ///

import httpx

print(httpx.get("https://example.com"))
```

## Using alternative package indexes

If you wish to use an alternative [package index](../concepts/indexes.md) to resolve dependencies,
you can provide the index with the `--index` option:

```console
$ fyn add --index "https://example.com/simple" --script example.py 'requests<3' 'rich'
```

This will include the package data in the inline metadata:

```python
# [[tool.fyn.index]]
# url = "https://example.com/simple"
```

If you require authentication to access the package index, then please refer to the
[package index](../concepts/indexes.md) documentation.

## Locking dependencies

fyn supports locking dependencies for PEP 723 scripts using the `fyn.lock` file format. Unlike with
projects, scripts must be explicitly locked using `fyn lock`:

```console
$ fyn lock --script example.py
```

Running `fyn lock --script` will create a `.lock` file adjacent to the script (e.g.,
`example.py.lock`).

Once locked, subsequent operations like `fyn run --script`, `fyn add --script`,
`fyn export --script`, and `fyn tree --script` will reuse the locked dependencies, updating the
lockfile if necessary.

If no such lockfile is present, commands like `fyn export --script` will still function as expected,
but will not create a lockfile.

## Improving reproducibility

In addition to locking dependencies, fyn supports an `exclude-newer` field in the `tool.fyn` section
of inline script metadata to limit fyn to only considering distributions released before a specific
date. This is useful for improving the reproducibility of your script when run at a later point in
time.

The date should be specified as an [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) timestamp
(e.g., `2006-12-02T02:07:43Z`).

```python title="example.py"
# /// script
# dependencies = [
#   "requests",
# ]
# [tool.fyn]
# exclude-newer = "2023-10-16T00:00:00Z"
# ///

import requests

print(requests.__version__)
```

## Using different Python versions

fyn allows arbitrary Python versions to be requested on each script invocation, for example:

```python title="example.py"
import sys

print(".".join(map(str, sys.version_info[:3])))
```

```console
$ # Use the default Python version, may differ on your machine
$ fyn run example.py
3.12.6
```

```console
$ # Use a specific Python version
$ fyn run --python 3.10 example.py
3.10.15
```

See the [Python version request](../concepts/python-versions.md#requesting-a-version) documentation
for more details on requesting Python versions.

## Using GUI scripts

On Windows `fyn` will run your script ending with `.pyw` extension using `pythonw`:

```python title="example.pyw"
from tkinter import Tk, ttk

root = Tk()
root.title("fyn")
frm = ttk.Frame(root, padding=10)
frm.grid()
ttk.Label(frm, text="Hello World").grid(column=0, row=0)
root.mainloop()
```

```console
PS> fyn run example.pyw
```

![Run Result](../assets/uv_gui_script_hello_world.png){: style="height:50px;width:150px"}

Similarly, it works with dependencies as well:

```python title="example_pyqt.pyw"
import sys
from PyQt5.QtWidgets import QApplication, QWidget, QLabel, QGridLayout

app = QApplication(sys.argv)
widget = QWidget()
grid = QGridLayout()

text_label = QLabel()
text_label.setText("Hello World!")
grid.addWidget(text_label)

widget.setLayout(grid)
widget.setGeometry(100, 100, 200, 50)
widget.setWindowTitle("fyn")
widget.show()
sys.exit(app.exec_())
```

```console
PS> fyn run --with PyQt5 example_pyqt.pyw
```

![Run Result](../assets/uv_gui_script_hello_world_pyqt.png){: style="height:50px;width:150px"}

## Next steps

To learn more about `fyn run`, see the [command reference](../reference/cli.md#fyn-run).

Or, read on to learn how to [run and install tools](./tools.md) with fyn.
