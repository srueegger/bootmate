#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="${1:-build}"

distrobox enter bootmate-devsystem -- bash -lc "
    set -euo pipefail
    cd '$REPO_DIR'
    if [ ! -x '$BUILD_DIR/src/bootmate' ]; then
        echo 'No binary at $BUILD_DIR/src/bootmate — run scripts/devbox-build.sh first.' >&2
        exit 1
    fi
    exec ./'$BUILD_DIR'/src/bootmate
"
