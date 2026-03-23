# NOTE this is not a real shell-script, it's parsed by `smoke-test/__main__.py` and executed
# serially via Python for cross-platform support.

# Show the fyn version
fyn --version

# Use any Python 3.13 version
fyn python pin 3.13

# Create a virtual environment and install a package with `fyn pip`
fyn venv -v
fyn pip install ruff -v

# Install a package with extension modules, e.g., `numpy` and make sure it's importable
fyn pip install numpy -v
fyn run --no-project python -c "import numpy; print(numpy.__version__)"

# Show the `fynx` version
fynx --version

# Run a package via `fynx`
fynx -v ruff --version
