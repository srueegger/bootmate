#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
PROFILE="${1:-debug}"

case "$PROFILE" in
    debug)   BUILD_DIR="build" ;;
    release) BUILD_DIR="build-release" ;;
    *) echo "Usage: $0 [debug|release]" >&2; exit 1 ;;
esac

distrobox enter bootmate-devsystem -- bash -lc "
    set -euo pipefail
    cd '$REPO_DIR'
    if [ ! -d '$BUILD_DIR' ]; then
        meson setup '$BUILD_DIR' -Dprofile=$PROFILE
    fi
    meson compile -C '$BUILD_DIR'
"
