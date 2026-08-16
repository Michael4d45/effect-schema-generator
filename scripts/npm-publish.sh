#!/usr/bin/env bash
set -euo pipefail

# Publish the wrapper package (native binaries are bundled under js/binaries/).
# Requires npm login (or NPM_TOKEN) and access to the @michael4d45 scope.

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/js"

if [[ ! -x binaries/linux-x64-musl/effect-schema ]]; then
    echo "linux-x64-musl binary is required (Alpine CI / Docker Vite)." >&2
    echo "Run scripts/build-npm-binaries.sh first." >&2
    exit 1
fi

npm publish --access public "$@"
