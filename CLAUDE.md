# Development Notes for Claude Code

## Environment
- Two development hosts: Ubuntu (IDE: RustRover) and macOS (Apple Silicon)
- **Nothing is built on the host.** Builds run in one of two environments, both with GNOME 51 (GTK 4.24, libadwaita 1.10):
  - Ubuntu host: `bootmate-devsystem` Distrobox container (Fedora 45), scripts `scripts/devbox-*.sh`
  - macOS host: `bootmate-dev` Multipass VM (Ubuntu 26.10), scripts `scripts/vm-*.sh`

## App Identity
- App ID: `me.rueegger.bootmate` (domain: rueegger.me)
- Previously was `ch.srueegger.bootmate` - changed for domain verification

## Build System
- Meson + Cargo (Rust edition 2024)
- Dependencies: GTK4 >= 4.24.0, libadwaita >= 1.10.0, glib >= 2.66

### Distrobox workflow
The container definition lives in [.distrobox/bootmate-devsystem.ini](.distrobox/bootmate-devsystem.ini). Thin wrapper scripts in [scripts/](scripts/) execute commands inside the container:

```bash
scripts/devbox-setup.sh            # one-time: create the container (runs dnf upgrade on init)
scripts/devbox-build.sh            # meson compile (debug)
scripts/devbox-build.sh release    # meson compile (release)
scripts/devbox-run.sh              # run the compiled binary
scripts/devbox-flatpak.sh          # local flatpak-builder test build
scripts/devbox-update.sh           # dnf upgrade --refresh inside the box
scripts/devbox-enter.sh            # interactive shell inside the box
```

All test-flatpak state (state dir, OSTree repo, flatpak-builder build dir, per-user flatpak installation) lives under `/var/cache/bootmate-flatpak/` **inside the container** — nothing lands in the host `$HOME`.

Host-side `meson`/`cargo`/`flatpak-builder` invocations are intentionally not used any more.

### Multipass workflow (macOS)
```bash
scripts/vm-setup.sh                # one-time: create the VM, mount the repo, install the toolchain
scripts/vm-build.sh                # meson compile (debug), build dir ~/build inside the VM
scripts/vm-build.sh release        # meson compile (release), build dir ~/build-release inside the VM
scripts/vm-test.sh                 # cargo test + meson test + metainfo validation
scripts/vm-flatpak.sh              # local flatpak-builder test build (state in ~/flatpak inside the VM)
scripts/vm-update.sh               # apt full-upgrade inside the VM
scripts/vm-enter.sh                # interactive shell inside the VM
```

- The repo is mounted at `/home/ubuntu/bootmate`. Build directories must stay outside the mount; `sed -i` and other rename-based tools fail on it.
- The VM has no graphical session. For a UI check run the app under `xvfb-run` and take a screenshot.
- On Apple Silicon the VM is `aarch64`: Flatpaks built there are ARM-only test builds. Published builds come from the Distrobox on the x86_64 Ubuntu host.

## Distribution
- **Flatpak is the only distribution method** (no DEB, no RPM)
- Own Flatpak repository at `https://flatpak.rueegger.dev`
- Repository project: `~/Sites/flatpak.rueegger.dev`
- Publish script: `~/Sites/flatpak.rueegger.dev/publish.sh`
- GPG Key ID: `A597D880A0505622AA55A6CB0718DBE47E8CA409`
- Server: `scifitre@flatpak.rueegger.dev` (cPanel shared hosting, Apache, static files only)

### Publishing a new version
The publish script runs on the Ubuntu host only (it needs the signing key and rsync/SSH setup, and re-enters the Distrobox). The build itself happens inside the container — the publish script invokes `scripts/devbox-flatpak.sh` under the hood.

```bash
~/Sites/flatpak.rueegger.dev/publish.sh bootmate
```

### GitHub Pages
- Project website served from `docs/` folder on `main` branch
- Dual-language (DE/EN) with automatic browser detection
- Dark/light mode via `prefers-color-scheme`

## Git Commits
- No traces of Claude/AI must be visible in git commits
- Do NOT use `Co-Authored-By: Claude` or similar AI attribution in commit messages
- Commit messages should look like they were written by a human developer
