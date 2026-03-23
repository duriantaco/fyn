#!/usr/bin/env bash
set -euxo pipefail

# Install fyn into the Termux prefix
cp /fyn /data/data/com.termux/files/usr/bin/uv
chmod +x /data/data/com.termux/files/usr/bin/uv

# Test uv
fyn --version

# Termux uses Bionic libc (not glibc or musl), so fyn cannot discover
# managed Python installations. Use only-system to skip that check.
export UV_PYTHON_PREFERENCE=only-system
fyn python find
fyn run -- python --version
