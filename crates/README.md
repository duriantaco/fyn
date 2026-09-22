# Crates

## [fyn-bench](./fyn-bench)

Functionality for benchmarking fyn.

## [fyn-cache-key](./fyn-cache-key)

Generic functionality for caching paths, URLs, and other resources across platforms.

## [fyn-distribution-filename](./fyn-distribution-filename)

Parse built distribution (wheel) and source distribution (sdist) filenames to extract structured
metadata.

## [fyn-distribution-types](./fyn-distribution-types)

Abstractions for representing built distributions (wheels) and source distributions (sdists), and
the sources from which they can be downloaded.

## [fyn-install-wheel-rs](./fyn-install-wheel)

Install built distributions (wheels) into a virtual environment.

## [fyn-once-map](./fyn-once-map)

A [`waitmap`](https://github.com/withoutboats/waitmap)-like concurrent hash map for executing tasks
exactly once.

## [fyn-pep440-rs](./fyn-pep440)

Utilities for interacting with Python version numbers and specifiers.

## [fyn-pep508-rs](./fyn-pep508)

Utilities for parsing and evaluating
[dependency specifiers](https://packaging.python.org/en/latest/specifications/dependency-specifiers/),
previously known as [PEP 508](https://peps.python.org/pep-0508/).

## [fyn-platform-tags](./fyn-platform-tags)

Functionality for parsing and inferring Python platform tags as per
[PEP 425](https://peps.python.org/pep-0425/).

## [fyn-cli](./fyn-cli)

Command-line interface for the fyn package manager.

## [fyn-build-frontend](./fyn-build-frontend)

A [PEP 517](https://www.python.org/dev/peps/pep-0517/)-compatible build frontend for fyn.

## [fyn-cache](./fyn-cache)

Functionality for caching Python packages and associated metadata.

## [fyn-client](./fyn-client)

Client for interacting with PyPI-compatible HTTP APIs.

## [fyn-dev](./fyn-dev)

Development utilities for fyn.

## [fyn-dispatch](./fyn-dispatch)

A centralized `struct` for resolving and building source distributions in isolated environments.
Implements the traits defined in `uv-types`.

## [fyn-distribution](./fyn-distribution)

Client for interacting with built distributions (wheels) and source distributions (sdists). Capable
of fetching metadata, distribution contents, etc.

## [fyn-extract](./fyn-extract)

Utilities for extracting files from archives.

## [fyn-fs](./fyn-fs)

Utilities for interacting with the filesystem.

## [fyn-git](./fyn-git)

Functionality for interacting with Git repositories.

## [fyn-installer](./fyn-installer)

Functionality for installing Python packages into a virtual environment.

## [fyn-python](./fyn-python)

Functionality for detecting and leveraging the current Python interpreter.

## [fyn-normalize](./fyn-normalize)

Normalize package and extra names as per Python specifications.

## [fyn-requirements](./fyn-requirements)

Utilities for reading package requirements from `pyproject.toml` and `requirements.txt` files.

## [fyn-resolver](./fyn-resolver)

Functionality for resolving Python packages and their dependencies.

## [fyn-shell](./fyn-shell)

Utilities for detecting and manipulating shell environments.

## [fyn-types](./fyn-types)

Shared traits for fyn, to avoid circular dependencies.

## [fyn-pypi-types](./fyn-pypi-types)

General-purpose type definitions for types used in PyPI-compatible APIs.

## [fyn-virtualenv](./fyn-virtualenv)

A `venv` replacement to create virtual environments in Rust.

## [fyn-warnings](./fyn-warnings)

User-facing warnings for fyn.

## [fyn-workspace](./fyn-workspace)

Workspace abstractions for fyn.

## [fyn-requirements-txt](./fyn-requirements-txt)

Functionality for parsing `requirements.txt` files.
