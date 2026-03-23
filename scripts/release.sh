#!/usr/bin/env bash
# Prepare for a release
#
# All additional options are passed to `rooster`
set -eu

script_root="$(realpath "$(dirname "$0")")"
project_root="$(dirname "$script_root")"

echo "Updating metadata with rooster..."
cd "$project_root"

# Update the changelog
fvx --python 3.12 rooster@0.1.1 release "$@"

# Bump library crate versions
fv run "$project_root/scripts/bump-workspace-crate-versions.py"

echo "Updating crate READMEs..."
fv run "$project_root/scripts/generate-crate-readmes.py"

echo "Updating lockfiles..."
cargo update -p uv
pushd crates/fv-trampoline; cargo update -p uv-trampoline; popd
fv lock

echo "Generating JSON schema..."
cargo dev generate-json-schema

echo "Creating release branch..."
git checkout -b "release/$(fv version --short)"
git commit -am "Bump version to $(fv version --short)"
