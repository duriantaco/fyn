#!/usr/bin/env bash
set -euxo pipefail

# Install fv into the Termux prefix
cp /fv /data/data/com.termux/files/usr/bin/uv
chmod +x /data/data/com.termux/files/usr/bin/uv

# Test uv
fv --version

# Termux uses Bionic libc (not glibc or musl), so fv cannot discover
# managed Python installations. Use only-system to skip that check.
export UV_PYTHON_PREFERENCE=only-system
fv python find
fv run -- python --version
