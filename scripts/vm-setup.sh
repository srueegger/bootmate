#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
VM_NAME="bootmate-dev"
# The repository is mounted here inside the VM. Build directories live in the
# VM's own home (~/build, ~/build-release, ~/flatpak): the mount is slow and
# does not support everything build tools expect (e.g. atomic renames).
VM_REPO="/home/ubuntu/bootmate"

if ! command -v multipass >/dev/null 2>&1; then
    echo "Error: multipass is not installed on the host." >&2
    echo "Install it first: https://multipass.run/" >&2
    exit 1
fi

if multipass info "$VM_NAME" >/dev/null 2>&1; then
    echo "VM '$VM_NAME' already exists."
    multipass start "$VM_NAME"
else
    echo "Creating '$VM_NAME' (Ubuntu 26.10, GNOME 51) ..."
    multipass launch 26.10 --name "$VM_NAME" --cpus 4 --memory 8G --disk 30G
fi

# Mount after launch: passing --mount to 'multipass launch' crashed multipassd
# on macOS and lost the freshly created instance.
if ! multipass info "$VM_NAME" | grep -q "$VM_REPO"; then
    multipass mount "$REPO_DIR" "$VM_NAME:$VM_REPO"
fi

multipass exec "$VM_NAME" -- bash -c '
    set -euo pipefail
    sudo apt-get update
    sudo DEBIAN_FRONTEND=noninteractive apt-get install -y \
        build-essential meson ninja-build pkg-config git \
        rustc cargo \
        libgtk-4-dev libadwaita-1-dev libglib2.0-dev-bin libxml2-utils \
        gettext desktop-file-utils appstream \
        flatpak flatpak-builder
    flatpak --user remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
'

echo ""
echo "VM ready."
echo "Next steps:"
echo "  scripts/vm-build.sh            # meson debug build"
echo "  scripts/vm-build.sh release    # meson release build"
echo "  scripts/vm-test.sh             # unit tests + meson tests"
echo "  scripts/vm-flatpak.sh          # build the flatpak locally"
echo "  scripts/vm-update.sh           # refresh packages inside the VM"
echo "  scripts/vm-enter.sh            # interactive shell inside the VM"
