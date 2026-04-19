#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"

# Container-local paths — keep host $HOME clean.
CACHE_ROOT=/var/cache/bootmate-flatpak

distrobox enter bootmate-devsystem -- bash -lc "
    set -euo pipefail
    cd '$REPO_DIR'

    export FLATPAK_USER_DIR='$CACHE_ROOT/flatpak-user'
    mkdir -p \"\$FLATPAK_USER_DIR\" '$CACHE_ROOT/state' '$CACHE_ROOT/repo' '$CACHE_ROOT/build'

    flatpak --user remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

    flatpak-builder \
        --user \
        --force-clean \
        --install-deps-from=flathub \
        --disable-rofiles-fuse \
        --state-dir='$CACHE_ROOT/state' \
        --repo='$CACHE_ROOT/repo' \
        '$CACHE_ROOT/build' \
        me.rueegger.bootmate.yml

    echo ''
    echo 'Flatpak built into OSTree repo at $CACHE_ROOT/repo (inside the container).'
    echo 'To install + run the test build inside the box:'
    echo '  flatpak --user remote-add --if-not-exists --no-gpg-verify bootmate-local $CACHE_ROOT/repo'
    echo '  flatpak --user install -y bootmate-local me.rueegger.bootmate'
    echo '  flatpak run me.rueegger.bootmate'
"
