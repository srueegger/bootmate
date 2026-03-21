# Development Notes for Claude Code

## Environment
- Host OS: Ubuntu
- IDE: RustRover
- Rust toolchain: rustup (installed in `~/.cargo` / `~/.rustup`)
- Build dependencies installed directly via apt

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
~/Projects/flatpak.rueegger.dev/publish.sh bootmate
```

### GitHub Pages
- Project website served from `docs/` folder on `main` branch
- Dual-language (DE/EN) with automatic browser detection
- Dark/light mode via `prefers-color-scheme`

## Build Dependencies (Ubuntu)
```bash
sudo apt install meson gcc libgtk-4-dev libadwaita-1-dev libglib2.0-dev gettext desktop-file-utils libappstream-glib-dev
```

## Git Commits
- No traces of Claude/AI must be visible in git commits
- Do NOT use `Co-Authored-By: Claude` or similar AI attribution in commit messages
- Commit messages should look like they were written by a human developer
