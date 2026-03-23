---
title: Using fyn with marimo
description:
  A complete guide to using fyn with marimo notebooks for interactive computing, script execution,
  and data apps.
---

# Using fyn with marimo

[marimo](https://github.com/marimo-team/marimo) is an open-source Python notebook that blends
interactive computing with the reproducibility and reusability of traditional software, letting you
version with Git, run as scripts, and share as apps. Because marimo notebooks are stored as pure
Python scripts, they are able to integrate tightly with fyn.

You can readily use marimo as a standalone tool, as self-contained scripts, in projects, and in
non-project environments.

## Using marimo as a standalone tool

For ad-hoc access to marimo notebooks, start a marimo server at any time in an isolated environment
with:

```console
$ fynx marimo edit
```

Start a specific notebook with:

```console
$ fynx marimo edit my_notebook.py
```

## Using marimo with inline script metadata

Because marimo notebooks are stored as Python scripts, they can encapsulate their own dependencies
using inline script metadata, via fyn's [support for scripts](../../guides/scripts.md). For example,
to add `numpy` as a dependency to your notebook, use this command:

```console
$ fyn add --script my_notebook.py numpy
```

To interactively edit a notebook containing inline script metadata, use:

```console
$ fynx marimo edit --sandbox my_notebook.py
```

marimo will automatically use fyn to start your notebook in an isolated virtual environment with
your script's dependencies. Packages installed from the marimo UI will automatically be added to the
notebook's script metadata.

You can optionally run these notebooks as Python scripts, without opening an interactive session:

```console
$ fyn run my_notebook.py
```

## Using marimo within a project

If you're working within a [project](../../concepts/projects/index.md), you can start a marimo
notebook with access to the project's virtual environment via the following command (assuming marimo
is a project dependency):

```console
$ fyn run marimo edit my_notebook.py
```

To make additional packages available to your notebook, either add them to your project with
`fyn add`, or use marimo's built-in package installation UI, which will invoke `fyn add` on your
behalf.

If marimo is not a project dependency, you can still run a notebook with the following command:

```console
$ fyn run --with marimo marimo edit my_notebook.py
```

This will let you import your project's modules while editing your notebook. However, packages
installed via marimo's UI when running in this way will not be added to your project, and may
disappear on subsequent marimo invocations.

## Using marimo in a non-project environment

To run marimo in a virtual environment that isn't associated with a
[project](../../concepts/projects/index.md), add marimo to the environment directly:

```console
$ fyn venv
$ fyn pip install numpy
$ fyn pip install marimo
$ fyn run marimo edit
```

From here, `import numpy` will work within the notebook, and marimo's UI installer will add packages
to the environment with `fyn pip install` on your behalf.

## Running marimo notebooks as scripts

Regardless of how your dependencies are managed (with inline script metadata, within a project, or
with a non-project environment), you can run marimo notebooks as scripts with:

```console
$ fyn run my_notebook.py
```

This executes your notebook as a Python script, without opening an interactive session in your
browser.
