# Development Notes for Claude Code

## Environment
- Host OS: Bluefin (Fedora-based immutable desktop)
- Package installation on host is NOT possible (immutable OS)
- Use **distrobox** for development dependencies (Rust, cargo, meson, etc.)
- Flatpak is available on the host system, Flathub is configured as repo

## App Identity
- App ID: `me.rueegger.bootmate` (domain: rueegger.me)
- Previously was `ch.srueegger.bootmate` - changed for Flathub verification

## Build System
- Meson + Cargo (Rust)
- Dependencies: GTK4 >= 4.10, libadwaita >= 1.5, glib >= 2.66
- Build: `meson setup build --prefix=/usr -Dprofile=release && meson compile -C build`

## Distrobox Usage
When you need Rust/Cargo or build tools, create a distrobox:
```bash
distrobox create --name bootmate-dev --image registry.fedoraproject.org/fedora:43
distrobox enter bootmate-dev -- sudo dnf install -y rust cargo meson gcc gtk4-devel libadwaita-devel glib2-devel gettext desktop-file-utils libappstream-glib
```
Then run commands inside: `distrobox enter bootmate-dev -- <command>`

## Git Commits
- No traces of Claude/AI must be visible in git commits
- Do NOT use `Co-Authored-By: Claude` or similar AI attribution in commit messages
- Commit messages should look like they were written by a human developer
