# Boot Mate

A modern GNOME application for managing autostart entries, built with Rust and libadwaita.

## Features

- View all autostart entries from user and system directories
- Edit autostart entry commands and parameters
- Delete or disable autostart entries
- Multi-language support (English and German)
- Follows GNOME Human Interface Guidelines
- Fast and lightweight, built with Rust

## Screenshots

### Light Mode
<table>
  <tr>
    <td width="50%">
      <img src="screenshots/bootmate-light.png" alt="Boot Mate - Light Mode">
      <p align="center"><i>Main window showing autostart entries</i></p>
    </td>
    <td width="50%">
      <img src="screenshots/bootmate-light-add.png" alt="Boot Mate - Adding Entry (Light)">
      <p align="center"><i>Adding a new autostart entry</i></p>
    </td>
  </tr>
</table>

### Dark Mode
<table>
  <tr>
    <td width="50%">
      <img src="screenshots/bootmate-dark.png" alt="Boot Mate - Dark Mode">
      <p align="center"><i>Main window showing autostart entries</i></p>
    </td>
    <td width="50%">
      <img src="screenshots/bootmate-dark-add.png" alt="Boot Mate - Adding Entry (Dark)">
      <p align="center"><i>Adding a new autostart entry</i></p>
    </td>
  </tr>
</table>

## Installation

### Flatpak (Recommended)

Install Boot Mate from the rueegger-dev Flatpak repository:

```bash
flatpak remote-add --if-not-exists rueegger-dev https://flatpak.rueegger.dev/rueegger-dev.flatpakrepo
flatpak install rueegger-dev me.rueegger.bootmate
```

Or install directly via the flatpakref file:

```bash
flatpak install https://flatpak.rueegger.dev/me.rueegger.bootmate.flatpakref
```

### From Source

If you prefer to build from source, see the [Building](#building) section below.

## Requirements

### Runtime Dependencies

- GTK 4.22.1 or later
- libadwaita 1.9.0 or later
- GLib 2.66 or later

### Build Dependencies

- Rust 1.70 or later (1.80+ recommended)
- Meson 0.59 or later
- Cargo
- glib-compile-resources
- gettext

## Building

### Using Meson (Recommended)

```bash
# Configure the build
meson setup build --prefix=/usr/local

# Build the application
meson compile -C build

# Install (optional)
sudo meson install -C build
```

### Development Build

For development with debug symbols:

```bash
meson setup build -Dprofile=debug
meson compile -C build
```

### Release Build

For optimized release build:

```bash
meson setup build -Dprofile=release
meson compile -C build
```

### Flatpak Build

To build as a Flatpak locally:

```bash
flatpak-builder --user --install --force-clean --disable-rofiles-fuse _flatpak me.rueegger.bootmate.yml
```

## Running

After installation:

```bash
bootmate
```

Or from the build directory without installing:

```bash
./build/src/bootmate
```

Or launch from your application menu: **Boot Mate**

## Project Structure

```
bootmate/
├── data/                   # Application data files
│   ├── icons/             # Application icons
│   ├── ui/                # GTK UI templates
│   ├── *.desktop.in       # Desktop entry file
│   └── *.metainfo.xml.in  # AppStream metadata
├── po/                     # Translations
│   ├── de.po              # German translation
│   └── en.po              # English translation
├── src/                    # Rust source code
│   ├── main.rs            # Application entry point
│   ├── application.rs     # Application logic
│   ├── window.rs          # Main window
│   ├── autostart.rs       # Autostart entry management
│   └── entry_row.rs       # List row widget
├── build.rs               # Build script
├── Cargo.toml             # Rust dependencies
├── meson.build            # Meson build configuration
└── LICENSE                # GPLv2 license

```

## How It Works

Boot Mate scans the following directories for `.desktop` files:

- `~/.config/autostart/` - User autostart entries (takes precedence)
- `/etc/xdg/autostart/` - System-wide autostart entries
- `/usr/share/gnome/autostart/` - GNOME autostart entries

### Editing Entries

When you edit a system-wide autostart entry, Boot Mate creates a user-specific copy in `~/.config/autostart/` with your changes. This ensures system files remain untouched.

### Deleting Entries

- **User entries**: Deleted directly from `~/.config/autostart/`
- **System entries**: A hidden override is created in `~/.config/autostart/` to disable the entry

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

### Development Guidelines

- All code must be in English (comments, function names, variables)
- Follow Rust best practices and idioms
- Maintain GNOME HIG compliance
- Add translations for new user-facing strings

## License

This project is licensed under the GNU General Public License v2.0 only (GPL-2.0-only).

See [LICENSE](LICENSE) for the full license text.

## Author

**Samuel Rüegger**

- GitHub: [@srueegger](https://github.com/srueegger)
- Repository: [bootmate](https://github.com/srueegger/bootmate)

## Contributors

- **Actionschnitzel** ([@actionschnitzel](https://github.com/actionschnitzel)) - ARM64 build support

## Acknowledgments

- Built with [GTK 4](https://gtk.org/) and [libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/)
- Rust bindings by [gtk-rs](https://gtk-rs.org/)
- Designed for modern GNOME desktop environments
