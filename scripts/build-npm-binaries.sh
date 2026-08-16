#!/usr/bin/env bash
set -euo pipefail

# Cross-compile Linux musl/gnu binaries used by the npm optional packages.
# Darwin binaries are produced on macos runners in .github/workflows/release.yml.

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if ! command -v rustup >/dev/null 2>&1; then
    echo "rustup is required" >&2
    exit 1
fi

rustup target add x86_64-unknown-linux-musl x86_64-unknown-linux-gnu

if command -v musl-gcc >/dev/null 2>&1; then
    CC_x86_64_unknown_linux_musl=musl-gcc \
        cargo build --release --target x86_64-unknown-linux-musl
else
    echo "musl-gcc not found; trying cargo zigbuild for musl" >&2
    if command -v cargo-zigbuild >/dev/null 2>&1 || cargo zigbuild --help >/dev/null 2>&1; then
        cargo zigbuild --release --target x86_64-unknown-linux-musl
    else
        echo "Install musl-tools (musl-gcc) or cargo-zigbuild to build Alpine binaries." >&2
        exit 1
    fi
fi

cargo build --release --target x86_64-unknown-linux-gnu || cargo build --release

"$ROOT/scripts/stage-npm-binaries.sh"
