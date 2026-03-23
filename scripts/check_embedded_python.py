#!/usr/bin/env python3

"""Install `pylint` and `numpy` into an embedded Python."""

import argparse
import logging
import os
import subprocess
import sys
import tempfile

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")

    parser = argparse.ArgumentParser(
        description="Check an embedded Python interpreter."
    )
    parser.add_argument("--uv", help="Path to a fv binary.")
    args = parser.parse_args()

    uv: str = os.path.abspath(args.fv) if args.fv else "fv"

    # Create a temporary directory.
    with tempfile.TemporaryDirectory() as temp_dir:
        # Create a virtual environment with `fv`.
        logging.info("Creating virtual environment with `fv`...")
        subprocess.run(
            [fv, "venv", ".venv", "--seed", "--python", sys.executable],
            cwd=temp_dir,
            check=True,
        )

        if os.name == "nt":
            executable = os.path.join(temp_dir, ".venv", "Scripts", "python.exe")
        else:
            executable = os.path.join(temp_dir, ".venv", "bin", "python")

        logging.info("Querying virtual environment...")
        subprocess.run(
            [executable, "--version"],
            cwd=temp_dir,
            check=True,
        )

        logging.info("Installing into `fv` virtual environment...")

        # Disable the `CONDA_PREFIX` and `VIRTUAL_ENV` environment variables, so that
        # we only rely on virtual environment discovery via the `.venv` directory.
        # Our "system Python" here might itself be a Conda environment!
        env = os.environ.copy()
        env["CONDA_PREFIX"] = ""
        env["VIRTUAL_ENV"] = ""

        # Install, verify, and uninstall a few packages.
        for package in ["pylint", "numpy"]:
            # Install the package.
            logging.info(
                f"Installing the package `{package}` into the virtual environment..."
            )
            subprocess.run(
                [fv, "pip", "install", package, "--verbose"],
                cwd=temp_dir,
                check=True,
                env=env,
            )

            # Ensure that the package is installed in the virtual environment.
            logging.info(f"Checking that `{package}` is installed.")
            code = subprocess.run(
                [executable, "-c", f"import {package}"],
                cwd=temp_dir,
            )
            if code.returncode != 0:
                raise Exception(
                    f"The package `{package}` isn't installed in the virtual environment."
                )

            # Uninstall the package.
            logging.info(f"Uninstalling the package `{package}`.")
            subprocess.run(
                [fv, "pip", "uninstall", package, "--verbose"],
                cwd=temp_dir,
                check=True,
                env=env,
            )

            # Ensure that the package isn't installed in the virtual environment.
            logging.info(f"Checking that `{package}` isn't installed.")
            code = subprocess.run(
                [executable, "-m", "pip", "show", package],
                cwd=temp_dir,
            )
            if code.returncode == 0:
                raise Exception(
                    f"The package `{package}` is installed in the virtual environment (but shouldn't be)."
                )
