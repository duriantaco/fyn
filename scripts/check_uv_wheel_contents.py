"""Check that fv and uv_build wheels contain exactly the expected files"""

import re
import sys
from argparse import ArgumentParser
from pathlib import Path
from zipfile import ZipFile

# Update these when changing the wheel contents
fv_expected = {
    "fv-VERSION.data/scripts/fv",
    "fv-VERSION.data/scripts/fvx",
    "fv-VERSION.dist-info/METADATA",
    "fv-VERSION.dist-info/RECORD",
    "fv-VERSION.dist-info/WHEEL",
    "fv-VERSION.dist-info/licenses/LICENSE-APACHE",
    "fv-VERSION.dist-info/licenses/LICENSE-MIT",
    "fv-VERSION.dist-info/sboms/fv.cyclonedx.json",
    "uv/__init__.py",
    "uv/__main__.py",
    "uv/_find_uv.py",
    "uv/py.typed",
}
fv_build_expected = {
    "uv_build-VERSION.data/scripts/fv-build",
    "uv_build-VERSION.dist-info/METADATA",
    "uv_build-VERSION.dist-info/RECORD",
    "uv_build-VERSION.dist-info/WHEEL",
    "uv_build-VERSION.dist-info/licenses/LICENSE-APACHE",
    "uv_build-VERSION.dist-info/licenses/LICENSE-MIT",
    "uv_build-VERSION.dist-info/sboms/fv-build.cyclonedx.json",
    "uv_build/__init__.py",
    "uv_build/__main__.py",
    "uv_build/py.typed",
}


def check_fv_wheel(fv_wheel: Path) -> None:
    if fv_wheel.name.startswith("fv-"):
        expected = fv_expected
        # Windows wheels contain fvw, the windowed launcher.
        if "-win" in fv_wheel.name:
            expected = expected | {"fv-VERSION.data/scripts/fvw"}
    elif fv_wheel.name.startswith("uv_build-"):
        expected = fv_build_expected
    else:
        raise RuntimeError(f"Unknown wheel filename: {fv_wheel.name}")

    with ZipFile(fv_wheel) as wheel:
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
        print("error: fv wheel has unexpected contents", file=sys.stderr)
        if expected - actual:
            print(f"  Missing wheel entries: {expected - actual}", file=sys.stderr)
        if actual - expected:
            print(f"  Unexpected wheel entries: {actual - expected}", file=sys.stderr)
        sys.exit(1)


def main():
    parser = ArgumentParser()
    parser.add_argument("wheels", type=Path, nargs="+")
    args = parser.parse_args()

    for fv_wheel in args.wheels:
        if fv_wheel.name.endswith(".tar.gz"):
            continue
        print(f"Checking {fv_wheel}")
        check_fv_wheel(fv_wheel)


if __name__ == "__main__":
    main()
