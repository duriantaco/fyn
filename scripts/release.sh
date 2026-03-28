#!/usr/bin/env bash
# Prepare for a release
#
# All additional options are passed to `rooster`
set -eu

script_root="$(realpath "$(dirname "$0")")"
project_root="$(dirname "$script_root")"
fyn_cmd=(cargo run --bin fyn --)

echo "Updating metadata with rooster..."
cd "$project_root"

# Update the changelog
"${fyn_cmd[@]}" tool run --python 3.12 rooster@0.1.1 release "$@"

# Bump library crate versions
"${fyn_cmd[@]}" run "$project_root/scripts/bump-workspace-crate-versions.py"

echo "Updating crate READMEs..."
"${fyn_cmd[@]}" run "$project_root/scripts/generate-crate-readmes.py"

echo "Updating lockfiles..."
cargo update -p fyn
pushd crates/fyn-trampoline; cargo update -p uv-trampoline; popd
"${fyn_cmd[@]}" lock

echo "Generating JSON schema..."
cargo dev generate-json-schema

echo "Creating release branch..."
git checkout -b "release/$("${fyn_cmd[@]}" version --short)"
git commit -am "Bump version to $("${fyn_cmd[@]}" version --short)"
