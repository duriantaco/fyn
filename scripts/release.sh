#!/usr/bin/env bash
# Prepare for a release
#
# All additional options are passed to `rooster`
set -euo pipefail

script_root="$(realpath "$(dirname "$0")")"
project_root="$(dirname "$script_root")"
fyn_cmd=(cargo run --bin fyn --)
release_tag_pattern='^[0-9]+\.[0-9]+\.[0-9]+([.-][0-9A-Za-z.-]+)?$'
release_base_tag=""

collect_release_tags() {
  local tag

  while IFS= read -r tag; do
    [[ -n "$tag" ]] || continue
    [[ "$tag" =~ $release_tag_pattern ]] || continue
    printf '%s\n' "$tag"
  done < <(git tag "$@" --sort=-version:refname)
}

validate_release_base_tag() {
  local tag
  local -a release_tags=()
  local -a merged_release_tags=()
  local -a conflicting_release_tags=()

  while IFS= read -r tag; do
    [[ -n "$tag" ]] || continue
    release_tags+=("$tag")
  done < <(collect_release_tags)

  while IFS= read -r tag; do
    [[ -n "$tag" ]] || continue
    merged_release_tags+=("$tag")
  done < <(collect_release_tags --merged HEAD)

  if [[ ${#release_tags[@]} -eq 0 ]]; then
    echo "No release tags found in this repository; refusing to run release prep." >&2
    return 1
  fi

  if [[ ${#merged_release_tags[@]} -eq 0 ]]; then
    echo "No release tags are reachable from HEAD; refusing to run release prep." >&2
    return 1
  fi

  release_base_tag="${merged_release_tags[0]}"

  # `fyn` currently releases from a single active line. Higher local tags from
  # other branches or remotes will poison rooster's base-version selection.
  for tag in "${release_tags[@]}"; do
    if [[ "$tag" == "$release_base_tag" ]]; then
      break
    fi
    conflicting_release_tags+=("$tag")
  done

  if [[ ${#conflicting_release_tags[@]} -gt 0 ]]; then
    {
      echo "Found higher release tags that are not ancestors of HEAD."
      echo "rooster will choose the wrong release base in this state."
      echo
      echo "HEAD: $(git rev-parse --short HEAD)"
      echo "Highest reachable release tag: $release_base_tag"
      echo "Conflicting local release tags: ${conflicting_release_tags[*]}"
      echo
      echo "Delete those local tags before rerunning, for example:"
      echo "  git tag -d ${conflicting_release_tags[*]}"
      echo
      echo "If you need them back later, fetch them again from the remote that owns them."
    } >&2
    return 1
  fi
}

cd "$project_root"

validate_release_base_tag
echo "Using release base tag: $release_base_tag"

if [[ "${1:-}" == "--validate-release-tags" ]]; then
  exit 0
fi

echo "Updating metadata with rooster..."

# Update the changelog
"${fyn_cmd[@]}" tool run --python 3.12 rooster@0.1.1 release "$@"

# Bump library crate versions
"${fyn_cmd[@]}" run "$project_root/scripts/bump-workspace-crate-versions.py"

echo "Updating crate READMEs..."
"${fyn_cmd[@]}" run "$project_root/scripts/generate-crate-readmes.py"

echo "Updating lockfiles..."
cargo update -p fyn
pushd crates/fyn-trampoline; cargo update; popd
"${fyn_cmd[@]}" lock

echo "Generating JSON schema..."
cargo dev generate-json-schema

echo "Creating release branch..."
git checkout -b "release/$("${fyn_cmd[@]}" version --short)"
git commit -am "Bump version to $("${fyn_cmd[@]}" version --short)"
