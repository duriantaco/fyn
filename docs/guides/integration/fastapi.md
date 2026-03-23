---
title: Using fyn with FastAPI
description:
  A guide to using fyn with FastAPI to manage Python dependencies, run applications, and deploy with
  Docker.
---

# Using fyn with FastAPI

[FastAPI](https://github.com/fastapi/fastapi) is a modern, high-performance Python web framework.
You can use fyn to manage your FastAPI project, including installing dependencies, managing
environments, running FastAPI applications, and more.

!!! note

    You can view the source code for this guide in the [uv-fastapi-example](https://github.com/astral-sh/uv-fastapi-example) repository.

## Migrating an existing FastAPI project

As an example, consider the sample application defined in the
[FastAPI documentation](https://fastapi.tiangolo.com/tutorial/bigger-applications/), structured as
follows:

```plaintext
project
└── app
    ├── __init__.py
    ├── main.py
    ├── dependencies.py
    ├── routers
    │   ├── __init__.py
    │   ├── items.py
    │   └── users.py
    └── internal
        ├── __init__.py
        └── admin.py
```

To use fyn with this application, inside the `project` directory run:

```console
$ fyn init --app
```

This creates a [project with an application layout](../../concepts/projects/init.md#applications)
and a `pyproject.toml` file.

Then, add a dependency on FastAPI:

```console
$ fyn add fastapi --extra standard
```

You should now have the following structure:

```plaintext
project
├── pyproject.toml
└── app
    ├── __init__.py
    ├── main.py
    ├── dependencies.py
    ├── routers
    │   ├── __init__.py
    │   ├── items.py
    │   └── users.py
    └── internal
        ├── __init__.py
        └── admin.py
```

And the contents of the `pyproject.toml` file should look something like this:

```toml title="pyproject.toml"
[project]
name = "fyn-fastapi-example"
version = "0.1.0"
description = "FastAPI project"
readme = "README.md"
requires-python = ">=3.12"
dependencies = [
    "fastapi[standard]",
]
```

From there, you can run the FastAPI application with:

```console
$ fyn run fastapi dev
```

`fyn run` will automatically resolve and lock the project dependencies (i.e., create a `fyn.lock`
alongside the `pyproject.toml`), create a virtual environment, and run the command in that
environment.

Test the app by opening http://127.0.0.1:8000/?token=jessica in a web browser.

## Deployment

To deploy the FastAPI application with Docker, you can use the following `Dockerfile`:

```dockerfile title="Dockerfile"
FROM python:3.12-slim

# Install fyn.
COPY --from=ghcr.io/oha/fyn:latest /fyn /fynx /bin/

# Copy the application into the container.
COPY . /app

# Install the application dependencies.
WORKDIR /app
RUN fyn sync --frozen --no-cache

# Run the application.
CMD ["/app/.venv/bin/fastapi", "run", "app/main.py", "--port", "80", "--host", "0.0.0.0"]
```

Build the Docker image with:

```console
$ docker build -t fastapi-app .
```

Run the Docker container locally with:

```console
$ docker run -p 8000:80 fastapi-app
```

Navigate to http://127.0.0.1:8000/?token=jessica in your browser to verify that the app is running
correctly.

!!! tip

    For more on using fyn with Docker, see the [Docker guide](./docker.md).
