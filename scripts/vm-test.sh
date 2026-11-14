#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
VM_NAME="bootmate-dev"
# The repository is mounted here inside the VM. Build directories live in the
# VM's own home (~/build, ~/build-release, ~/flatpak): the mount is slow and
# does not support everything build tools expect (e.g. atomic renames).
VM_REPO="/home/ubuntu/bootmate"

multipass exec "$VM_NAME" -- bash -lc "
    set -euo pipefail
    cd '$VM_REPO'
    if [ ! -d ~/build ]; then
        meson setup ~/build .
    fi
    CARGO_TARGET_DIR=~/build/src cargo test
    meson compile -C ~/build
    meson test -C ~/build
    appstreamcli validate --no-net ~/build/data/me.rueegger.bootmate.metainfo.xml
"
