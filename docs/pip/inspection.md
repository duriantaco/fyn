# Inspecting environments

## Listing installed packages

To list all the packages in the environment:

```console
$ fv pip list
```

To list the packages in a JSON format:

```console
$ fv pip list --format json
```

To list all the packages in the environment in a `requirements.txt` format:

```console
$ fv pip freeze
```

## Inspecting a package

To show information about an installed package, e.g., `numpy`:

```console
$ fv pip show numpy
```

Multiple packages can be inspected at once.

## Verifying an environment

It is possible to install packages with conflicting requirements into an environment if installed in
multiple steps.

To check for conflicts or missing dependencies in the environment:

```console
$ fv pip check
```
