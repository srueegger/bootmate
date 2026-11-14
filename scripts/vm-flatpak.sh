#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
VM_NAME="bootmate-dev"
# The repository is mounted here inside the VM. Build directories live in the
# VM's own home (~/build, ~/build-release, ~/flatpak): the mount is slow and
# does not support everything build tools expect (e.g. atomic renames).
VM_REPO="/home/ubuntu/bootmate"

# Note: the VM has the architecture of the host. On Apple Silicon this
# produces an aarch64 Flatpak, which is fine for testing but not what
# publish.sh ships (that one runs in the Distrobox on the x86_64 host).
multipass exec "$VM_NAME" -- bash -lc "
    set -euo pipefail
    cd '$VM_REPO'
    mkdir -p ~/flatpak

    flatpak-builder \\
        --user \\
        --force-clean \\
        --install \\
        --install-deps-from=flathub \\
        --disable-rofiles-fuse \\
        --state-dir=\$HOME/flatpak/state \\
        --repo=\$HOME/flatpak/repo \\
        \$HOME/flatpak/build \\
        me.rueegger.bootmate.yml

    echo ''
    echo 'Flatpak built and installed inside the VM (user installation).'
    echo 'Run it from a shell in the VM: flatpak run me.rueegger.bootmate'
"
