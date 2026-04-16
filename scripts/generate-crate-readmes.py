# /// script
# requires-python = ">=3.13"
# dependencies = []
# ///

import json
import pathlib
import subprocess

GENERATED_HEADER = "<!-- This file is generated. DO NOT EDIT -->"

fyn_TEMPLATE = """{GENERATED_HEADER}

# fyn

fyn is a Python package and project manager.

See the [repository](https://github.com/oha/fyn)
for more information.

This crate is the entry point to the fyn command-line interface. The Rust API exposed here is not
considered public interface.

This is version {fyn_version}. The source can be found [here]({source_url}).

The following fyn workspace members are also available:

{WORKSPACE_MEMBERS}

fyn's workspace members are considered internal and will have frequent breaking changes.

See fyn's crate versioning policy for details on versioning.
"""


MEMBER_TEMPLATE = """{GENERATED_HEADER}

# {name}

This crate is an internal component of [fyn](https://crates.io/crates/fyn). The Rust API exposed here is
unstable and will have frequent breaking changes.

This version ({crate_version}) is a component of [fyn {fyn_version}]({fyn_crates_io_url}). The source can
be found [here]({source_url}).

See fyn's crate versioning policy for details on versioning.
"""


REPO_URL = "https://github.com/oha/fyn"
PRETTIER_VERSION = "3.8.3"


def main() -> None:
    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1"],
        capture_output=True,
        text=True,
        check=True,
    )
    content = json.loads(result.stdout)
    packages = {package["id"]: package for package in content["packages"]}

    # Find the fyn version from the fyn crate
    fyn_version = None
    for package in content["packages"]:
        if package["name"] == "fyn":
            fyn_version = package["version"]
            break
    if fyn_version is None:
        raise RuntimeError("Could not find fyn crate")

    workspace_root = pathlib.Path(content["workspace_root"])
    readme_path = workspace_root / "crates" / "fyn" / "README.md"

    workspace_members = []
    for workspace_member in content["workspace_members"]:
        package = packages[workspace_member]
        name = package["name"]
        # Skip the main fyn crate
        if name == "fyn":
            continue
        # Skip crates with publish = false
        if package.get("publish") == []:
            continue
        workspace_members.append(name)

    workspace_members.sort()

    members_list = "\n".join(
        f"- [{name}](https://crates.io/crates/{name})" for name in workspace_members
    )

    # Generate README for the main fyn crate
    fyn_source_url = f"{REPO_URL}/blob/{fyn_version}/crates/fyn"
    readme_content = fyn_TEMPLATE.format(
        GENERATED_HEADER=GENERATED_HEADER,
        WORKSPACE_MEMBERS=members_list,
        fyn_version=fyn_version,
        source_url=fyn_source_url,
    )
    readme_path.write_text(readme_content)

    # Track all generated README paths for formatting at the end
    generated_paths = [readme_path]

    # Generate READMEs for all workspace members
    for workspace_member in content["workspace_members"]:
        package = packages[workspace_member]
        name = package["name"]

        # Skip the main fyn crate (already handled above)
        if name == "fyn":
            continue

        # Determine the README path for this crate
        manifest_path = pathlib.Path(package["manifest_path"])
        crate_dir = manifest_path.parent
        member_readme_path = crate_dir / "README.md"

        # Check if README already exists
        if member_readme_path.exists():
            existing_content = member_readme_path.read_text()
            # Skip if it doesn't have the generated header
            if not existing_content.startswith(GENERATED_HEADER):
                print(f"Skipping {name}: existing README without generated header")
                continue

        # Get the crate version and compute source URL
        crate_version = package["version"]
        # Compute relative path from workspace root to crate directory
        relative_crate_path = crate_dir.relative_to(workspace_root)
        source_url = f"{REPO_URL}/blob/{fyn_version}/{relative_crate_path}"

        # Generate the README content
        fyn_crates_io_url = f"https://crates.io/crates/fyn/{fyn_version}"
        member_readme_content = MEMBER_TEMPLATE.format(
            GENERATED_HEADER=GENERATED_HEADER,
            name=name,
            crate_version=crate_version,
            fyn_version=fyn_version,
            fyn_crates_io_url=fyn_crates_io_url,
            source_url=source_url,
        )
        member_readme_path.write_text(member_readme_content)
        generated_paths.append(member_readme_path)

        print(f"Generated README for {name}")

    # Format all generated READMEs once at the end
    subprocess.run(
        ["npx", "--yes", f"prettier@{PRETTIER_VERSION}", "--write"]
        + [str(path) for path in generated_paths],
        check=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )


if __name__ == "__main__":
    main()
