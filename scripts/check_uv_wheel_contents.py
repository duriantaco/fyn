"""Check that fyn and uv_build wheels contain exactly the expected files"""

import re
import sys
from argparse import ArgumentParser
from pathlib import Path
from zipfile import ZipFile

# Update these when changing the wheel contents
fyn_expected = {
    "fyn-VERSION.data/scripts/fyn",
    "fyn-VERSION.data/scripts/fynx",
    "fyn-VERSION.dist-info/METADATA",
    "fyn-VERSION.dist-info/RECORD",
    "fyn-VERSION.dist-info/WHEEL",
    "fyn-VERSION.dist-info/licenses/LICENSE-APACHE",
    "fyn-VERSION.dist-info/licenses/LICENSE-MIT",
    "fyn-VERSION.dist-info/sboms/fyn.cyclonedx.json",
    "uv/__init__.py",
    "uv/__main__.py",
    "uv/_find_uv.py",
    "uv/py.typed",
}
fyn_build_expected = {
    "uv_build-VERSION.data/scripts/fyn-build",
    "uv_build-VERSION.dist-info/METADATA",
    "uv_build-VERSION.dist-info/RECORD",
    "uv_build-VERSION.dist-info/WHEEL",
    "uv_build-VERSION.dist-info/licenses/LICENSE-APACHE",
    "uv_build-VERSION.dist-info/licenses/LICENSE-MIT",
    "uv_build-VERSION.dist-info/sboms/fyn-build.cyclonedx.json",
    "uv_build/__init__.py",
    "uv_build/__main__.py",
    "uv_build/py.typed",
}


def check_fyn_wheel(fyn_wheel: Path) -> None:
    if fyn_wheel.name.startswith("fyn-"):
        expected = fyn_expected
        # Windows wheels contain fynw, the windowed launcher.
        if "-win" in fyn_wheel.name:
            expected = expected | {"fyn-VERSION.data/scripts/fynw"}
    elif fyn_wheel.name.startswith("uv_build-"):
        expected = fyn_build_expected
    else:
        raise RuntimeError(f"Unknown wheel filename: {fyn_wheel.name}")

    with ZipFile(fyn_wheel) as wheel:
        files = wheel.namelist()
    # Escape the version and remove the Windows exe extension.
    actual = {
        re.sub(r"^([a-z_0-9]*)-([0-9]+\.)*[0-9]+", r"\1-VERSION", file).replace(
            ".exe", ""
        )
        for file in files
    }
    if expected != actual:
        # Verbose log
        print(f"Expected: {sorted(expected)}", file=sys.stderr)
        print(f"Actual:   {sorted(actual)}", file=sys.stderr)
        print("", file=sys.stderr)
        # Concise error
        print("error: fyn wheel has unexpected contents", file=sys.stderr)
        if expected - actual:
            print(f"  Missing wheel entries: {expected - actual}", file=sys.stderr)
        if actual - expected:
            print(f"  Unexpected wheel entries: {actual - expected}", file=sys.stderr)
        sys.exit(1)


def main():
    parser = ArgumentParser()
    parser.add_argument("wheels", type=Path, nargs="+")
    args = parser.parse_args()

    for fyn_wheel in args.wheels:
        if fyn_wheel.name.endswith(".tar.gz"):
            continue
        print(f"Checking {fyn_wheel}")
        check_fyn_wheel(fyn_wheel)


if __name__ == "__main__":
    main()
