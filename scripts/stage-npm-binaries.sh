#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PACKAGES="$ROOT/js/npm-packages"
BUNDLED="$ROOT/js/binaries"

stage() {
    local pkg="$1"
    local src="$2"
    if [[ ! -f "$src" ]]; then
        echo "skip ${pkg}: missing ${src}" >&2
        return 0
    fi
    local dest="$PACKAGES/${pkg}/effect-schema"
    mkdir -p "$(dirname "$dest")" "$BUNDLED/${pkg}"
    cp "$src" "$dest"
    cp "$src" "$BUNDLED/${pkg}/effect-schema"
    chmod +x "$dest" "$BUNDLED/${pkg}/effect-schema"
    if command -v strip >/dev/null 2>&1; then
        strip "$dest" "$BUNDLED/${pkg}/effect-schema" || true
    fi
    echo "staged ${pkg} ($(wc -c < "$dest") bytes)"
}

stage linux-x64-gnu "$ROOT/target/release/effect-schema"
stage linux-x64-gnu "$ROOT/target/x86_64-unknown-linux-gnu/release/effect-schema"
stage linux-x64-musl "$ROOT/target/x86_64-unknown-linux-musl/release/effect-schema"
stage linux-arm64-gnu "$ROOT/target/aarch64-unknown-linux-gnu/release/effect-schema"
stage linux-arm64-musl "$ROOT/target/aarch64-unknown-linux-musl/release/effect-schema"
stage darwin-arm64 "$ROOT/target/aarch64-apple-darwin/release/effect-schema"
stage darwin-x64 "$ROOT/target/x86_64-apple-darwin/release/effect-schema"
