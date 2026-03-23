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
fynx --python 3.12 rooster@0.1.1 release "$@"

# Bump library crate versions
fyn run "$project_root/scripts/bump-workspace-crate-versions.py"

echo "Updating crate READMEs..."
fyn run "$project_root/scripts/generate-crate-readmes.py"

echo "Updating lockfiles..."
cargo update -p uv
pushd crates/fyn-trampoline; cargo update -p uv-trampoline; popd
fyn lock

echo "Generating JSON schema..."
cargo dev generate-json-schema

echo "Creating release branch..."
git checkout -b "release/$(fyn version --short)"
git commit -am "Bump version to $(fyn version --short)"
