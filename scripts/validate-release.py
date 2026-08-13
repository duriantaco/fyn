#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import re
import sys
import tomllib
from datetime import UTC, date, datetime
from pathlib import Path
from typing import Any

PROJECT_ROOT = Path(__file__).resolve().parents[1]
VERSION_FILES = {
    Path("pyproject.toml"): ("project", "version"),
    Path("crates/fyn/Cargo.toml"): ("package", "version"),
    Path("crates/fyn-build/Cargo.toml"): ("package", "version"),
    Path("crates/fyn-build/pyproject.toml"): ("project", "version"),
    Path("crates/fyn-version/Cargo.toml"): ("package", "version"),
    Path("Cargo.toml"): ("workspace", "dependencies", "fyn-version", "version"),
}
LOCKED_PACKAGES = {"fyn", "fyn-build", "fyn-version"}


def read_toml_value(path: Path, keys: tuple[str, ...]) -> Any:
    value: Any = tomllib.loads(path.read_text())
    for key in keys:
        value = value[key]
    return value


def validate_metadata(tag: str, release_date: date) -> list[str]:
    errors: list[str] = []

    if tag.startswith("v"):
        errors.append(f"release tag must not have a leading 'v': {tag}")

    for relative_path, keys in VERSION_FILES.items():
        version = read_toml_value(PROJECT_ROOT / relative_path, keys)
        if version != tag:
            errors.append(f"{relative_path} has version {version!r}, expected {tag!r}")

    cargo_lock = tomllib.loads((PROJECT_ROOT / "Cargo.lock").read_text())
    locked_versions = {
        package["name"]: package["version"]
        for package in cargo_lock["package"]
        if package["name"] in LOCKED_PACKAGES
    }
    for package in sorted(LOCKED_PACKAGES):
        version = locked_versions.get(package)
        if version != tag:
            errors.append(
                f"Cargo.lock package {package!r} has version {version!r}, "
                f"expected {tag!r}"
            )

    changelog = (PROJECT_ROOT / "CHANGELOG.md").read_text()
    match = re.search(
        rf"^## {re.escape(tag)}\n\nReleased on (\d{{4}}-\d{{2}}-\d{{2}})\.$",
        changelog,
        flags=re.MULTILINE,
    )
    if match is None:
        errors.append(
            f"CHANGELOG.md must contain '## {tag}' followed by a release date"
        )
    else:
        try:
            changelog_date = date.fromisoformat(match.group(1))
        except ValueError:
            errors.append(f"CHANGELOG.md has an invalid release date: {match.group(1)}")
        else:
            if abs((changelog_date - release_date).days) > 1:
                errors.append(
                    "CHANGELOG.md release date is not within one day of today: "
                    f"{changelog_date} vs {release_date}"
                )

    return errors


def validate_plan(manifest_path: Path, repo: str) -> list[str]:
    errors: list[str] = []
    manifest = json.loads(manifest_path.read_text())

    try:
        owner, repository = repo.split("/", maxsplit=1)
    except ValueError:
        return [f"repository must use owner/name form: {repo!r}"]

    announcement_tag = manifest.get("announcement_tag")
    if not isinstance(announcement_tag, str) or not announcement_tag:
        errors.append("release plan does not contain an announcement_tag")
        return errors
    version = announcement_tag.removeprefix("v")

    releases = manifest.get("releases")
    if not isinstance(releases, list) or not releases:
        errors.append("release plan does not contain any releases")
        return errors

    expected_base_url = (
        f"https://github.com/{repo}/releases/download/{announcement_tag}"
    )
    expected_download_path = f"/{repo}/releases/download/{announcement_tag}"

    for release in releases:
        app_name = release.get("app_name")
        if release.get("app_version") != version:
            errors.append(
                f"{app_name or 'release'} version {release.get('app_version')!r} "
                f"does not match planned version {version!r}"
            )

        artifacts = set(release.get("artifacts", []))
        for suffix in ("-installer.sh", "-installer.ps1"):
            if not any(artifact.endswith(suffix) for artifact in artifacts):
                errors.append(f"{app_name or 'release'} is missing a {suffix} artifact")

        hosting = release.get("hosting", {})
        github = hosting.get("github", {})
        simple = hosting.get("simple", {})
        if github.get("owner") != owner or github.get("repo") != repository:
            errors.append(
                f"{app_name or 'release'} GitHub hosting points to "
                f"{github.get('owner')}/{github.get('repo')}, expected {repo}"
            )
        if github.get("artifact_download_path") != expected_download_path:
            errors.append(
                f"{app_name or 'release'} GitHub download path is incorrect: "
                f"{github.get('artifact_download_path')!r}"
            )
        if simple.get("download_url") != expected_base_url:
            errors.append(
                f"{app_name or 'release'} simple download URL is incorrect: "
                f"{simple.get('download_url')!r}"
            )

    return errors


def report(errors: list[str]) -> None:
    if not errors:
        return
    for error in errors:
        print(f"error: {error}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)

    metadata = subparsers.add_parser("metadata")
    metadata.add_argument("--tag", required=True)
    metadata.add_argument(
        "--date",
        type=date.fromisoformat,
        default=datetime.now(UTC).date(),
        help="Expected release date in YYYY-MM-DD format (defaults to today in UTC)",
    )

    plan = subparsers.add_parser("plan")
    plan.add_argument("--manifest", required=True, type=Path)
    plan.add_argument("--repo", required=True)

    args = parser.parse_args()
    if args.command == "metadata":
        report(validate_metadata(args.tag, args.date))
        print(f"Release metadata is valid for {args.tag} ({args.date})")
    else:
        report(validate_plan(args.manifest, args.repo))
        print(f"Release plan is valid for {args.repo}")


if __name__ == "__main__":
    main()
