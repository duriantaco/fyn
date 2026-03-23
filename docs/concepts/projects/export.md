---
title: Exporting a lockfile
description: Exporting a lockfile to different formats
---

# Exporting a lockfile

fv can export a lockfile to different formats for integration with other tools and workflows. The
`fv export` command supports multiple output formats, each suited to different use cases.

For more details on lockfiles and how they're created, see the [project layout](./layout.md) and
[locking and syncing](./sync.md) documentation.

## Overview of export formats

fv supports three export formats:

- `requirements.txt`: The traditional pip-compatible
  [requirements file format](https://pip.pypa.io/en/stable/reference/requirements-file-format/).
- `pylock.toml`: The standardized Python lockfile format defined in
  [PEP 751](https://peps.python.org/pep-0751/).
- `CycloneDX`: An industry-standard [Software Bill of Materials (SBOM)](https://cyclonedx.org/)
  format.

The format can be specified with the `--format` flag:

```console
$ fv export --format requirements.txt
$ fv export --format pylock.toml
$ fv export --format cyclonedx1.5
```

!!! tip

    By default, `fv export` prints to stdout. Use `--output-file` to write to a file for any format:

    ```console
    $ fv export --format requirements.txt --output-file requirements.txt
    $ fv export --format pylock.toml --output-file pylock.toml
    $ fv export --format cyclonedx1.5 --output-file sbom.json
    ```

## `requirements.txt` format

The `requirements.txt` format is the most widely supported format for Python dependencies. It can be
used with `pip` and other Python package managers.

### Basic usage

```console
$ fv export --format requirements.txt
```

The generated `requirements.txt` file can then be installed via `fv pip install`, or with other
tools like `pip`.

!!! note

    In general, we recommend against using both a `fv.lock` and a `requirements.txt` file. The
    `fv.lock` format is more powerful and includes features that cannot be expressed in
    `requirements.txt`. If you find yourself exporting a `fv.lock` file, consider opening an issue
    to discuss your use case.

## `pylock.toml` format

[PEP 751](https://peps.python.org/pep-0751/) defines a TOML-based lockfile format for Python
dependencies. fv can export your project's dependency lockfile to this format.

### Basic usage

```console
$ fv export --format pylock.toml
```

## CycloneDX SBOM format

fv can export your project's dependency lockfile as a Software Bill of Materials (SBOM) in CycloneDX
format. SBOMs provide a comprehensive inventory of all software components in your application,
which is useful for security auditing, compliance, and supply chain transparency.

!!! important

    Support for exporting to CycloneDX is in [preview](../preview.md), and may change in any future release.

### What is CycloneDX?

[CycloneDX](https://cyclonedx.org/) is an industry-standard format for creating Software Bill of
Materials. CycloneDX is machine readable and widely supported by security scanning tools,
vulnerability databases, and Software Composition Analysis (SCA) platforms.

### Basic usage

To export your project's lockfile as a CycloneDX SBOM:

```console
$ fv export --format cyclonedx1.5
```

This will generate a JSON-encoded CycloneDX v1.5 document containing your project and all of its
dependencies.

### SBOM Structure

The generated SBOM follows the
[CycloneDX specification](https://cyclonedx.org/specification/overview/). fv also includes the
following custom properties on components:

- `fv:package:marker`: Environment markers (e.g., `python_version >= "3.8"`)
- `fv:workspace:path`: Relative path for workspace members

## Next steps

To learn more about lockfiles and exporting, see the [locking and syncing](./sync.md) documentation
and the [command reference](../../reference/cli.md#fv-export).

Or, read on to learn how to
[build and publish your project to a package index](../../guides/package.md).
