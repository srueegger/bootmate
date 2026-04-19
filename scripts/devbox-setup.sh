#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
INI_FILE="$REPO_DIR/.distrobox/bootmate-devsystem.ini"

if ! command -v distrobox >/dev/null 2>&1; then
    echo "Error: distrobox is not installed on the host." >&2
    echo "Install it first: https://distrobox.it/" >&2
    exit 1
fi

if distrobox list | grep -q '^\s*[0-9a-f]\+\s*|\s*bootmate-devsystem\b'; then
    echo "Container 'bootmate-devsystem' already exists."
    echo "To recreate: distrobox rm --force bootmate-devsystem && $0"
    exit 0
fi

echo "Creating 'bootmate-devsystem' from $INI_FILE ..."
distrobox assemble create --file "$INI_FILE"

echo ""
echo "Container ready."
echo "Next steps:"
echo "  scripts/devbox-build.sh            # meson debug build"
echo "  scripts/devbox-build.sh release    # meson release build"
echo "  scripts/devbox-run.sh              # run the compiled binary"
echo "  scripts/devbox-flatpak.sh          # build the flatpak locally"
echo "  scripts/devbox-update.sh           # refresh packages inside the box"
echo "  scripts/devbox-enter.sh            # interactive shell inside the box"
