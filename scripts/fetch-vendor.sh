#!/usr/bin/env bash
# Verify and fetch vendored crate sources.
#
# Vendored crates live in `vendor/<crate>` and are bundles of a published
# crates.io crate, pinned to the version recorded in `Cargo.lock`, with any
# fyn-specific modifications applied on top (see `vendor/README.md`).
#
# Usage:
#
#   scripts/fetch-vendor.sh [--check]   Verify every vendored crate is complete
#                                       and its version matches `Cargo.lock`.
#                                       This is the default and is used by CI
#                                       (check-generated-files.yml).
#   scripts/fetch-vendor.sh --fetch     Download the crates.io source archive for
#                                       every vendored crate at the pinned
#                                       version into `target/vendor-src/` and
#                                       print a diff summary against `vendor/`.
#
# Exit status is non-zero if any vendored crate is missing files or its version
# diverges from `Cargo.lock`.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VENDOR_DIR="$ROOT/vendor"
LOCK_FILE="$ROOT/Cargo.lock"
FETCH_DIR="$ROOT/target/vendor-src"

lock_version() {
    # Prints the version pinned in Cargo.lock for the given package name.
    awk -v crate="$1" '
        /^\[\[package\]\]/ { in_package = 1; package_name = ""; next }
        /^\[\[/ { in_package = 0 }
        in_package && /^name[[:space:]]*=/ { gsub(/[" ]/, "", $0); sub(/^name=/, "", $0); package_name = $0 }
        in_package && /^version[[:space:]]*=/ && package_name == crate {
            gsub(/[" ]/, "", $0); sub(/^version=/, "", $0); print; exit
        }
    ' "$LOCK_FILE"
}

verify_crate() {
    local manifest="$1"
    local dir
    dir="$(dirname "$manifest")"
    local name version
    name="$(sed -n 's/^name = "\(.*\)"/\1/p' "$manifest" | head -n 1)"
    version="$(sed -n 's/^version = "\(.*\)"/\1/p' "$manifest" | head -n 1)"

    if [[ -z "$name" || -z "$version" ]]; then
        echo "error: could not parse $manifest" >&2
        return 1
    fi

    local failures=0
    if [[ ! -f "$dir/src/lib.rs" ]]; then
        echo "error: missing source file: $dir/src/lib.rs" >&2
        failures=1
    fi
    if [[ ! -f "$dir/LICENSE" ]]; then
        echo "error: missing license file: $dir/LICENSE" >&2
        failures=1
    fi
    if [[ ! -f "$dir/README.md" ]]; then
        echo "error: missing readme file: $dir/README.md" >&2
        failures=1
    fi

    local pinned
    pinned="$(lock_version "$name")"
    if [[ -z "$pinned" ]]; then
        echo "error: $name is not pinned in $LOCK_FILE" >&2
        failures=1
    elif [[ "$pinned" != "$version" ]]; then
        echo "error: $name version $version in $manifest does not match Cargo.lock ($pinned)" >&2
        failures=1
    fi

    if [[ "$failures" -eq 0 ]]; then
        echo "ok: $name $version"
    fi
    return "$failures"
}

fetch_crate() {
    local manifest="$1"
    local dir
    dir="$(dirname "$manifest")"
    local name version
    name="$(sed -n 's/^name = "\(.*\)"/\1/p' "$manifest" | head -n 1)"
    version="$(sed -n 's/^version = "\(.*\)"/\1/p' "$manifest" | head -n 1)"

    local archive="$FETCH_DIR/${name}-${version}.crate"
    local extracted="$FETCH_DIR/${name}-${version}"
    mkdir -p "$FETCH_DIR"
    if [[ ! -f "$archive" ]]; then
        echo "Fetching $name $version from crates.io..."
        curl -fsSL --retry 5 -o "$archive" \
            "https://static.crates.io/crates/$name/$name-$version.crate"
    fi
    if [[ ! -d "$extracted" ]]; then
        mkdir -p "$extracted"
        tar -xzf "$archive" -C "$extracted" --strip-components=1
    fi
    echo "Diff summary for $name $version:"
    if git -C "$ROOT" diff --no-index --stat "$extracted" "$dir" >/dev/null 2>&1; then
        echo "  vendored sources match the published crate"
    else
        git -C "$ROOT" diff --no-index --stat "$extracted" "$dir" || true
    fi
}

mode="check"
if [[ "${1:-}" == "--fetch" ]]; then
    mode="fetch"
elif [[ -n "${1:-}" && "${1:-}" != "--check" ]]; then
    echo "usage: $0 [--check|--fetch]" >&2
    exit 2
fi

if [[ ! -d "$VENDOR_DIR" ]]; then
    echo "error: no vendored crates found in $VENDOR_DIR" >&2
    exit 1
fi

overall=0
for manifest in "$VENDOR_DIR"/*/Cargo.toml; do
    [[ -f "$manifest" ]] || continue
    if [[ "$mode" == "fetch" ]]; then
        fetch_crate "$manifest"
    else
        verify_crate "$manifest" || overall=1
    fi
done
exit "$overall"