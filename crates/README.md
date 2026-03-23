# Crates

## [fyn-bench](./uv-bench)

Functionality for benchmarking fyn.

## [fyn-cache-key](./uv-cache-key)

Generic functionality for caching paths, URLs, and other resources across platforms.

## [fyn-distribution-filename](./uv-distribution-filename)

Parse built distribution (wheel) and source distribution (sdist) filenames to extract structured
metadata.

## [fyn-distribution-types](./uv-distribution-types)

Abstractions for representing built distributions (wheels) and source distributions (sdists), and
the sources from which they can be downloaded.

## [fyn-install-wheel-rs](./uv-install-wheel)

Install built distributions (wheels) into a virtual environment.

## [fyn-once-map](./uv-once-map)

A [`waitmap`](https://github.com/withoutboats/waitmap)-like concurrent hash map for executing tasks
exactly once.

## [fyn-pep440-rs](./uv-pep440)

Utilities for interacting with Python version numbers and specifiers.

## [fyn-pep508-rs](./uv-pep508)

Utilities for parsing and evaluating
[dependency specifiers](https://packaging.python.org/en/latest/specifications/dependency-specifiers/),
previously known as [PEP 508](https://peps.python.org/pep-0508/).

## [fyn-platform-tags](./uv-platform-tags)

Functionality for parsing and inferring Python platform tags as per
[PEP 425](https://peps.python.org/pep-0425/).

## [fyn-cli](./uv-cli)

Command-line interface for the fyn package manager.

## [fyn-build-frontend](./uv-build-frontend)

A [PEP 517](https://www.python.org/dev/peps/pep-0517/)-compatible build frontend for fyn.

## [fyn-cache](./uv-cache)

Functionality for caching Python packages and associated metadata.

## [fyn-client](./uv-client)

Client for interacting with PyPI-compatible HTTP APIs.

## [fyn-dev](./uv-dev)

Development utilities for fyn.

## [fyn-dispatch](./uv-dispatch)

A centralized `struct` for resolving and building source distributions in isolated environments.
Implements the traits defined in `uv-types`.

## [fyn-distribution](./uv-distribution)

Client for interacting with built distributions (wheels) and source distributions (sdists). Capable
of fetching metadata, distribution contents, etc.

## [fyn-extract](./uv-extract)

Utilities for extracting files from archives.

## [fyn-fs](./uv-fs)

Utilities for interacting with the filesystem.

## [fyn-git](./uv-git)

Functionality for interacting with Git repositories.

## [fyn-installer](./uv-installer)

Functionality for installing Python packages into a virtual environment.

## [fyn-python](./uv-python)

Functionality for detecting and leveraging the current Python interpreter.

## [fyn-normalize](./uv-normalize)

Normalize package and extra names as per Python specifications.

## [fyn-requirements](./uv-requirements)

Utilities for reading package requirements from `pyproject.toml` and `requirements.txt` files.

## [fyn-resolver](./uv-resolver)

Functionality for resolving Python packages and their dependencies.

## [fyn-shell](./uv-shell)

Utilities for detecting and manipulating shell environments.

## [fyn-types](./uv-types)

Shared traits for fyn, to avoid circular dependencies.

## [fyn-pypi-types](./uv-pypi-types)

General-purpose type definitions for types used in PyPI-compatible APIs.

## [fyn-virtualenv](./uv-virtualenv)

A `venv` replacement to create virtual environments in Rust.

## [fyn-warnings](./uv-warnings)

User-facing warnings for fyn.

## [fyn-workspace](./uv-workspace)

Workspace abstractions for fyn.

## [fyn-requirements-txt](./uv-requirements-txt)

Functionality for parsing `requirements.txt` files.
