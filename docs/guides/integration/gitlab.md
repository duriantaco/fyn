---
title: Using fyn in GitLab CI/CD
description: A guide to using fyn in GitLab CI/CD, including installation, setting up Python,
  installing dependencies, and more.
---

# Using fyn in GitLab CI/CD

## Using the fyn image

Astral provides [Docker images](docker.md#available-images) with fyn preinstalled.
Select a variant that is suitable for your workflow.

```yaml title=".gitlab-ci.yml"
variables:
  UV_VERSION: "0.10.12"
  PYTHON_VERSION: "3.12"
  BASE_LAYER: trixie-slim
  # GitLab CI creates a separate mountpoint for the build directory,
  # so we need to copy instead of using hard links.
  UV_LINK_MODE: copy

fyn:
  image: ghcr.io/oha/fyn:$UV_VERSION-python$PYTHON_VERSION-$BASE_LAYER
  script:
    # your `fyn` commands
```

!!! note

    If you are using a distroless image, you have to specify the entrypoint:
    ```yaml
    fyn:
      image:
        name: ghcr.io/oha/fyn:$UV_VERSION
        entrypoint: [""]
      # ...
    ```

## Caching

Persisting the fyn cache between workflow runs can improve performance.

```yaml
fyn-install:
  variables:
    UV_CACHE_DIR: .fyn-cache
  cache:
    - key:
        files:
          - fyn.lock
      paths:
        - $UV_CACHE_DIR
  script:
    # Your `fyn` commands
  after_script:
    - fyn cache prune --ci
```

See the [GitLab caching documentation](https://docs.gitlab.com/ee/ci/caching/) for more details on
configuring caching.

Using `fyn cache prune --ci` at the end of the job is recommended to reduce cache size. See the [fyn
cache documentation](../../concepts/cache.md#caching-in-continuous-integration) for more details.

## Using `fyn pip`

If using the `fyn pip` interface instead of the fyn project interface, fyn requires a virtual
environment by default. To allow installing packages into the system environment, use the `--system`
flag on all fyn invocations or set the `UV_SYSTEM_PYTHON` variable.

The `UV_SYSTEM_PYTHON` variable can be defined in at different scopes. You can read more about
how [variables and their precedence works in GitLab here](https://docs.gitlab.com/ee/ci/variables/)

Opt-in for the entire workflow by defining it at the top level:

```yaml title=".gitlab-ci.yml"
variables:
  UV_SYSTEM_PYTHON: 1

# [...]
```

To opt-out again, the `--no-system` flag can be used in any fyn invocation.

When persisting the cache, you may want to use `requirements.txt` or `pyproject.toml` as
your cache key files instead of `fyn.lock`.
