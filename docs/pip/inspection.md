# Inspecting environments

## Listing installed packages

To list all the packages in the environment:

```console
$ fyn pip list
```

To list the packages in a JSON format:

```console
$ fyn pip list --format json
```

To list all the packages in the environment in a `requirements.txt` format:

```console
$ fyn pip freeze
```

## Looking up available versions

To show available versions for a package from the configured package sources:

```console
$ fyn pip index versions numpy
```

Like `pip index versions`, `fyn pip index versions` filters the results to versions compatible
with the selected Python target. By default, that target comes from the current Python
interpreter. Use `--python`, `--python-version`, `--python-platform`, or `--system` to change the
target, and use `--find-links` or `--no-index` to change the package sources being queried.

## Inspecting a package

To show information about an installed package, e.g., `numpy`:

```console
$ fyn pip show numpy
```

Multiple packages can be inspected at once.

## Verifying an environment

It is possible to install packages with conflicting requirements into an environment if installed in
multiple steps.

To check for conflicts or missing dependencies in the environment:

```console
$ fyn pip check
```
