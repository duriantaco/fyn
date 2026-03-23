This fake fyn package symlinks the Python module of fyn in-tree and has a fake `uv` binary, allowing
testing of the Python module behaviors. Consumers can replace the `uv` binary with a debug binary or
similar if they need it to actually work.
