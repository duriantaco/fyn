# NOTE this is not a real shell-script, it's parsed by `smoke-test/__main__.py` and executed
# serially via Python for cross-platform support.

# Show the fv version
fv --version

# Use any Python 3.13 version
fv python pin 3.13

# Create a virtual environment and install a package with `fv pip`
fv venv -v
fv pip install ruff -v

# Install a package with extension modules, e.g., `numpy` and make sure it's importable
fv pip install numpy -v
fv run --no-project python -c "import numpy; print(numpy.__version__)"

# Show the `fvx` version
fvx --version

# Run a package via `fvx`
fvx -v ruff --version
