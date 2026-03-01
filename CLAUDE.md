# Development Notes for Claude Code

## Environment
- Host OS: Bluefin (Fedora-based immutable desktop)
- Package installation on host is NOT possible (immutable OS)
- Use **distrobox** for development dependencies (Rust, cargo, meson, etc.)
- Flatpak is available on the host system

## App Identity
- App ID: `me.rueegger.bootmate` (domain: rueegger.me)
- Previously was `ch.srueegger.bootmate` - changed for domain verification

## Build System
- Meson + Cargo (Rust)
- Dependencies: GTK4 >= 4.10, libadwaita >= 1.5, glib >= 2.66
- Build: `meson setup build --prefix=/usr -Dprofile=release && meson compile -C build`

## Distribution
- **Flatpak is the only distribution method** (no DEB, no RPM)
- Own Flatpak repository at `https://flatpak.rueegger.dev`
- Repository project: `~/Projects/flatpak.rueegger.dev`
- Publish script: `~/Projects/flatpak.rueegger.dev/publish.sh`
- GPG Key ID: `A597D880A0505622AA55A6CB0718DBE47E8CA409`
- Server: `scifitre@flatpak.rueegger.dev` (cPanel shared hosting, Apache, static files only)

### Publishing a new version
```bash
distrobox enter bootmate-dev -- /var/home/srueegger/Projects/flatpak.rueegger.dev/publish.sh bootmate
```

### GitHub Pages
- Project website served from `docs/` folder on `main` branch
- Dual-language (DE/EN) with automatic browser detection
- Dark/light mode via `prefers-color-scheme`

## Distrobox Usage
When you need Rust/Cargo or build tools, create a distrobox:
```bash
distrobox create --name bootmate-dev --image registry.fedoraproject.org/fedora:43
distrobox enter bootmate-dev -- sudo dnf install -y rust cargo meson gcc gtk4-devel libadwaita-devel glib2-devel gettext desktop-file-utils libappstream-glib flatpak flatpak-builder ostree
```
Then run commands inside: `distrobox enter bootmate-dev -- <command>`

## Git Commits
- No traces of Claude/AI must be visible in git commits
- Do NOT use `Co-Authored-By: Claude` or similar AI attribution in commit messages
- Commit messages should look like they were written by a human developer
