# Boot Mate

A modern GNOME application for managing autostart entries, built with Rust and libadwaita.

## Features

- 📋 View all autostart entries from user and system directories
- ✏️ Edit autostart entry commands and parameters
- 🗑️ Delete or disable autostart entries
- 🌍 Multi-language support (English and German)
- 🎨 Follows GNOME Human Interface Guidelines
- ⚡ Fast and lightweight, built with Rust

## Screenshots

![Main Window](screenshots/main-window.png)

## Requirements

### Runtime Dependencies

- GTK 4.10 or later
- libadwaita 1.4 or later (1.8+ recommended for Ubuntu 25.10)
- GLib 2.66 or later

### Build Dependencies

- Rust 1.70 or later
- Meson 0.59 or later
- Cargo
- glib-compile-resources
- glib-compile-schemas
- gettext

### Ubuntu 25.10

On Ubuntu 25.10, you can install all dependencies with:

```bash
sudo apt install build-essential meson cargo rustc \
    libgtk-4-dev libadwaita-1-dev libglib2.0-dev \
    gettext appstream-util desktop-file-utils
```

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

## Building a Snap

To build a snap package for distribution via the Snap Store:

```bash
# Install snapcraft
sudo snap install snapcraft --classic

# Build the snap (snapcraft.yaml to be added)
snapcraft
```

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

## Acknowledgments

- Built with [GTK 4](https://gtk.org/) and [libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/)
- Rust bindings by [gtk-rs](https://gtk-rs.org/)
- Inspired by the need for autostart management in Ubuntu 25.10+
