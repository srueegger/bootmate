#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
VM_NAME="bootmate-dev"
# The repository is mounted here inside the VM. Build directories live in the
# VM's own home (~/build, ~/build-release, ~/flatpak): the mount is slow and
# does not support everything build tools expect (e.g. atomic renames).
VM_REPO="/home/ubuntu/bootmate"
PROFILE="${1:-debug}"

case "$PROFILE" in
    debug)   BUILD_DIR="build" ;;
    release) BUILD_DIR="build-release" ;;
    *) echo "Usage: $0 [debug|release]" >&2; exit 1 ;;
esac

multipass exec "$VM_NAME" -- bash -lc "
    set -euo pipefail
    cd '$VM_REPO'
    if [ ! -d ~/'$BUILD_DIR' ]; then
        meson setup ~/'$BUILD_DIR' . -Dprofile=$PROFILE
    fi
    meson compile -C ~/'$BUILD_DIR'
"
