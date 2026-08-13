#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import os
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any


class VerificationError(RuntimeError):
    pass


def fetch(
    url: str,
    *,
    token: str | None,
    attempts: int,
    retry_delay: float,
) -> bytes:
    headers = {
        "Accept": "application/vnd.github+json",
        "User-Agent": "fyn-release-verifier",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    if token is not None:
        headers["Authorization"] = f"Bearer {token}"

    last_error: Exception | None = None
    for attempt in range(1, attempts + 1):
        try:
            with urllib.request.urlopen(
                urllib.request.Request(url, headers=headers), timeout=30
            ) as response:
                return response.read()
        except (urllib.error.HTTPError, urllib.error.URLError, TimeoutError) as error:
            last_error = error
            if attempt == attempts:
                break
            print(
                f"Waiting for release data ({attempt}/{attempts}): {error}",
                file=sys.stderr,
            )
            time.sleep(retry_delay)

    raise VerificationError(f"failed to fetch {url}: {last_error}")


def fetch_json(
    url: str,
    *,
    token: str | None,
    attempts: int,
    retry_delay: float,
) -> dict[str, Any]:
    return json.loads(
        fetch(
            url,
            token=token,
            attempts=attempts,
            retry_delay=retry_delay,
        )
    )


def expected_release_assets(manifest_path: Path, tag: str) -> set[str]:
    manifest = json.loads(manifest_path.read_text())
    if manifest.get("announcement_tag") != tag:
        raise VerificationError(
            f"release manifest tag {manifest.get('announcement_tag')!r} "
            f"does not match {tag!r}"
        )

    expected: set[str] = set()
    for release in manifest.get("releases", []):
        expected.update(release.get("artifacts", []))
    if not expected:
        raise VerificationError("release manifest does not list any GitHub assets")
    return expected


def verify_github_release(
    *,
    repo: str,
    tag: str,
    commit: str,
    manifest_path: Path,
    token: str | None,
    attempts: int,
    retry_delay: float,
) -> None:
    encoded_tag = urllib.parse.quote(tag, safe="")
    api_base = f"https://api.github.com/repos/{repo}"
    release = fetch_json(
        f"{api_base}/releases/tags/{encoded_tag}",
        token=token,
        attempts=attempts,
        retry_delay=retry_delay,
    )
    if release.get("tag_name") != tag:
        raise VerificationError(
            f"GitHub release tag is {release.get('tag_name')!r}, expected {tag!r}"
        )

    tagged_commit = fetch_json(
        f"{api_base}/commits/{encoded_tag}",
        token=token,
        attempts=attempts,
        retry_delay=retry_delay,
    ).get("sha")
    if tagged_commit != commit:
        raise VerificationError(
            f"GitHub tag resolves to {tagged_commit!r}, expected {commit!r}"
        )

    expected_assets = expected_release_assets(manifest_path, tag)
    assets = {asset["name"]: asset for asset in release.get("assets", [])}
    missing_assets = sorted(expected_assets - assets.keys())
    if missing_assets:
        raise VerificationError(
            "GitHub release is missing assets: " + ", ".join(missing_assets)
        )

    expected_url = f"https://github.com/{repo}/releases/download/{tag}"
    installer_names = sorted(
        name
        for name in expected_assets
        if name.endswith("-installer.sh") or name.endswith("-installer.ps1")
    )
    if not installer_names:
        raise VerificationError("GitHub release does not contain installer scripts")

    release_url_pattern = re.compile(
        r"https://github\.com/([^/\s\"']+/[^/\s\"']+)/releases/download/([^/\s\"']+)"
    )
    for installer_name in installer_names:
        installer = fetch(
            assets[installer_name]["browser_download_url"],
            token=None,
            attempts=attempts,
            retry_delay=retry_delay,
        ).decode()
        if expected_url not in installer:
            raise VerificationError(
                f"{installer_name} does not contain the expected URL {expected_url}"
            )
        wrong_urls = sorted(
            url for url in release_url_pattern.findall(installer) if url != (repo, tag)
        )
        if wrong_urls:
            raise VerificationError(
                f"{installer_name} contains incorrect release URLs: {wrong_urls}"
            )

    print(
        f"Verified GitHub release {tag}: {len(expected_assets)} expected assets, "
        f"{len(installer_names)} installers"
    )


def github_run_artifacts(
    *,
    repo: str,
    run_id: str,
    token: str | None,
    attempts: int,
    retry_delay: float,
) -> list[dict[str, Any]]:
    artifacts: list[dict[str, Any]] = []
    for page in range(1, 100):
        response = fetch_json(
            f"https://api.github.com/repos/{repo}/actions/runs/{run_id}/artifacts"
            f"?per_page=100&page={page}",
            token=token,
            attempts=attempts,
            retry_delay=retry_delay,
        )
        page_artifacts = response.get("artifacts", [])
        artifacts.extend(page_artifacts)
        if len(page_artifacts) < 100:
            return artifacts
    raise VerificationError("release run has too many artifact pages")


def verify_pypi(
    *,
    repo: str,
    run_id: str,
    tag: str,
    package_prefixes: list[tuple[str, str]],
    token: str | None,
    attempts: int,
    retry_delay: float,
) -> None:
    run_artifacts = github_run_artifacts(
        repo=repo,
        run_id=run_id,
        token=token,
        attempts=attempts,
        retry_delay=retry_delay,
    )
    artifact_names = {
        artifact["name"] for artifact in run_artifacts if not artifact.get("expired")
    }

    for project, prefix in package_prefixes:
        expected_count = sum(name.startswith(prefix) for name in artifact_names)
        if expected_count == 0:
            raise VerificationError(
                f"release run does not contain artifacts with prefix {prefix!r}"
            )

        encoded_project = urllib.parse.quote(project, safe="")
        encoded_tag = urllib.parse.quote(tag, safe="")
        response = fetch_json(
            f"https://pypi.org/pypi/{encoded_project}/{encoded_tag}/json",
            token=None,
            attempts=attempts,
            retry_delay=retry_delay,
        )
        if response.get("info", {}).get("version") != tag:
            raise VerificationError(
                f"PyPI project {project} did not publish version {tag}"
            )

        filenames = {item["filename"] for item in response.get("urls", [])}
        if len(filenames) != expected_count:
            raise VerificationError(
                f"PyPI project {project} has {len(filenames)} files for {tag}; "
                f"the release run produced {expected_count} artifacts"
            )
        if not any(filename.endswith(".whl") for filename in filenames):
            raise VerificationError(f"PyPI project {project} has no wheels for {tag}")
        if not any(filename.endswith(".tar.gz") for filename in filenames):
            raise VerificationError(f"PyPI project {project} has no sdist for {tag}")

        print(f"Verified PyPI project {project} {tag}: {len(filenames)} distributions")


def parse_package_prefix(value: str) -> tuple[str, str]:
    try:
        project, prefix = value.split("=", maxsplit=1)
    except ValueError as error:
        raise argparse.ArgumentTypeError("expected PROJECT=ARTIFACT_PREFIX") from error
    if not project or not prefix:
        raise argparse.ArgumentTypeError("expected PROJECT=ARTIFACT_PREFIX")
    return project, prefix


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", required=True)
    parser.add_argument("--tag", required=True)
    parser.add_argument("--commit", required=True)
    parser.add_argument("--run-id", required=True)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument(
        "--pypi-package",
        action="append",
        default=[],
        type=parse_package_prefix,
        metavar="PROJECT=ARTIFACT_PREFIX",
    )
    parser.add_argument("--attempts", type=int, default=12)
    parser.add_argument("--retry-delay", type=float, default=5)
    args = parser.parse_args()

    token = os.environ.get("GH_TOKEN")

    verify_github_release(
        repo=args.repo,
        tag=args.tag,
        commit=args.commit,
        manifest_path=args.manifest,
        token=token,
        attempts=args.attempts,
        retry_delay=args.retry_delay,
    )
    if args.pypi_package:
        verify_pypi(
            repo=args.repo,
            run_id=args.run_id,
            tag=args.tag,
            package_prefixes=args.pypi_package,
            token=token,
            attempts=args.attempts,
            retry_delay=args.retry_delay,
        )


if __name__ == "__main__":
    main()
