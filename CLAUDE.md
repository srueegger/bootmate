# Development Notes for Claude Code

## Environment
- Host OS: Ubuntu
- IDE: RustRover
- **All builds run inside the `bootmate-devsystem` Distrobox container** (Fedora 44, GNOME 50, libadwaita 1.9+). The host does not need any build dependencies.

## App Identity
- App ID: `me.rueegger.bootmate` (domain: rueegger.me)
- Previously was `ch.srueegger.bootmate` - changed for domain verification

## Build System
- Meson + Cargo (Rust edition 2024)
- Dependencies: GTK4 >= 4.22.1, libadwaita >= 1.9.0, glib >= 2.66

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

## Distribution
- **Flatpak is the only distribution method** (no DEB, no RPM)
- Own Flatpak repository at `https://flatpak.rueegger.dev`
- Repository project: `~/Projects/flatpak.rueegger.dev`
- Publish script: `~/Projects/flatpak.rueegger.dev/publish.sh`
- GPG Key ID: `A597D880A0505622AA55A6CB0718DBE47E8CA409`
- Server: `scifitre@flatpak.rueegger.dev` (cPanel shared hosting, Apache, static files only)

### Publishing a new version
The publish script runs on the host (it needs the signing key and rsync/SSH setup). The build itself happens inside the container — the publish script invokes `scripts/devbox-flatpak.sh` under the hood.

```bash
~/Projects/flatpak.rueegger.dev/publish.sh bootmate
```

### GitHub Pages
- Project website served from `docs/` folder on `main` branch
- Dual-language (DE/EN) with automatic browser detection
- Dark/light mode via `prefers-color-scheme`

## Git Commits
- No traces of Claude/AI must be visible in git commits
- Do NOT use `Co-Authored-By: Claude` or similar AI attribution in commit messages
- Commit messages should look like they were written by a human developer
