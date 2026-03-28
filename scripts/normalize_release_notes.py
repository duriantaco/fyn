#!/usr/bin/env python3

from __future__ import annotations

import argparse
import re
from pathlib import Path


def replace_release_urls(body: str, repo: str, tag: str) -> str:
    repo_url = f"https://github.com/{repo}"
    body = re.sub(
        rf"https://github\.com/[^/]+/[^/]+/releases/download/{re.escape(tag)}/",
        f"{repo_url}/releases/download/{tag}/",
        body,
    )
    body = re.sub(
        r"https://github\.com/[^/]+/[^/]+/attestations",
        f"{repo_url}/attestations",
        body,
    )
    body = re.sub(
        r"--repo\s+[^\s/]+/[^\s/]+",
        f"--repo {repo}",
        body,
    )
    return body


def build_install_section(tag: str, repo: str, assets: set[str]) -> str:
    shell_asset = next((name for name in sorted(assets) if name.endswith(".sh")), None)
    powershell_asset = next((name for name in sorted(assets) if name.endswith(".ps1")), None)

    if not shell_asset and not powershell_asset:
        return ""

    lines = [f"## Install fyn {tag}", ""]
    if shell_asset:
        lines.extend(
            [
                "### Install prebuilt binaries via shell script",
                "",
                "```sh",
                f"curl --proto '=https' --tlsv1.2 -LsSf https://github.com/{repo}/releases/download/{tag}/{shell_asset} | sh",
                "```",
                "",
            ]
        )
    if powershell_asset:
        lines.extend(
            [
                "### Install prebuilt binaries via powershell script",
                "",
                "```sh",
                f'powershell -ExecutionPolicy Bypass -c "irm https://github.com/{repo}/releases/download/{tag}/{powershell_asset} | iex"',
                "```",
                "",
            ]
        )
    return "\n".join(lines).strip() + "\n\n"


def normalize_download_table(download_block: str, assets: set[str]) -> str:
    lines = download_block.splitlines()
    if len(lines) < 3:
        return download_block

    kept = lines[:2]
    for line in lines[2:]:
        match = re.match(r"\| \[([^\]]+)\]\(", line)
        if not match or match.group(1) in assets:
            kept.append(line)
    return "\n".join(kept)


def normalize_notes(body: str, repo: str, tag: str, assets: set[str]) -> str:
    body = replace_release_urls(body, repo, tag)

    download_heading = f"## Download fyn {tag}"
    install_start = body.find(f"## Install fyn {tag}")
    download_start = body.find(download_heading)
    if install_start != -1 and download_start != -1 and install_start < download_start:
        body = body[:install_start] + build_install_section(tag, repo, assets) + body[download_start:]

    table_match = re.search(
        rf"({re.escape(download_heading)}\n\n\|  File  \| Platform \| Checksum \|\n\|[-|]+\|\n(?:\|.*\n)+)",
        body,
    )
    if table_match:
        body = body.replace(
            table_match.group(1),
            normalize_download_table(table_match.group(1), assets),
        )

    return body


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--notes-file", required=True)
    parser.add_argument("--artifacts-dir", required=True)
    parser.add_argument("--repo", required=True)
    parser.add_argument("--tag", required=True)
    args = parser.parse_args()

    notes_path = Path(args.notes_file)
    assets = {path.name for path in Path(args.artifacts_dir).iterdir() if path.is_file()}
    body = notes_path.read_text()
    notes_path.write_text(normalize_notes(body, args.repo, args.tag, assets))


if __name__ == "__main__":
    main()
