# Crates

## [fv-bench](./uv-bench)

Functionality for benchmarking fv.

## [fv-cache-key](./uv-cache-key)

Generic functionality for caching paths, URLs, and other resources across platforms.

## [fv-distribution-filename](./uv-distribution-filename)

Parse built distribution (wheel) and source distribution (sdist) filenames to extract structured
metadata.

## [fv-distribution-types](./uv-distribution-types)

Abstractions for representing built distributions (wheels) and source distributions (sdists), and
the sources from which they can be downloaded.

## [fv-install-wheel-rs](./uv-install-wheel)

Install built distributions (wheels) into a virtual environment.

## [fv-once-map](./uv-once-map)

A [`waitmap`](https://github.com/withoutboats/waitmap)-like concurrent hash map for executing tasks
exactly once.

## [fv-pep440-rs](./uv-pep440)

Utilities for interacting with Python version numbers and specifiers.

## [fv-pep508-rs](./uv-pep508)

Utilities for parsing and evaluating
[dependency specifiers](https://packaging.python.org/en/latest/specifications/dependency-specifiers/),
previously known as [PEP 508](https://peps.python.org/pep-0508/).

## [fv-platform-tags](./uv-platform-tags)

Functionality for parsing and inferring Python platform tags as per
[PEP 425](https://peps.python.org/pep-0425/).

## [fv-cli](./uv-cli)

Command-line interface for the fv package manager.

## [fv-build-frontend](./uv-build-frontend)

A [PEP 517](https://www.python.org/dev/peps/pep-0517/)-compatible build frontend for fv.

## [fv-cache](./uv-cache)

Functionality for caching Python packages and associated metadata.

## [fv-client](./uv-client)

Client for interacting with PyPI-compatible HTTP APIs.

## [fv-dev](./uv-dev)

Development utilities for fv.

## [fv-dispatch](./uv-dispatch)

A centralized `struct` for resolving and building source distributions in isolated environments.
Implements the traits defined in `uv-types`.

## [fv-distribution](./uv-distribution)

Client for interacting with built distributions (wheels) and source distributions (sdists). Capable
of fetching metadata, distribution contents, etc.

## [fv-extract](./uv-extract)

Utilities for extracting files from archives.

## [fv-fs](./uv-fs)

Utilities for interacting with the filesystem.

## [fv-git](./uv-git)

Functionality for interacting with Git repositories.

## [fv-installer](./uv-installer)

Functionality for installing Python packages into a virtual environment.

## [fv-python](./uv-python)

Functionality for detecting and leveraging the current Python interpreter.

## [fv-normalize](./uv-normalize)

Normalize package and extra names as per Python specifications.

## [fv-requirements](./uv-requirements)

Utilities for reading package requirements from `pyproject.toml` and `requirements.txt` files.

## [fv-resolver](./uv-resolver)

Functionality for resolving Python packages and their dependencies.

## [fv-shell](./uv-shell)

Utilities for detecting and manipulating shell environments.

## [fv-types](./uv-types)

Shared traits for fv, to avoid circular dependencies.

## [fv-pypi-types](./uv-pypi-types)

General-purpose type definitions for types used in PyPI-compatible APIs.

## [fv-virtualenv](./uv-virtualenv)

A `venv` replacement to create virtual environments in Rust.

## [fv-warnings](./uv-warnings)

User-facing warnings for fv.

## [fv-workspace](./uv-workspace)

Workspace abstractions for fv.

## [fv-requirements-txt](./uv-requirements-txt)

Functionality for parsing `requirements.txt` files.
