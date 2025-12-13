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

### Download Pre-built Packages

Pre-built DEB packages are automatically created for each release:

- **DEB Package**: Download from [GitHub Releases](https://github.com/srueegger/bootmate/releases)

```bash
# Install deb package (Ubuntu/Debian)
sudo dpkg -i bootmate_1.0.0_amd64.deb
sudo apt-get install -f  # Install dependencies if needed
```

### From Source

If you prefer to build from source, see the [Building](#building) section below.

## Requirements

### Runtime Dependencies

- GTK 4.10 or later (GTK 4.12+ included in Ubuntu 24.04 LTS)
- libadwaita 1.4 or later (libadwaita 1.4+ included in Ubuntu 24.04 LTS)
- GLib 2.66 or later

### Build Dependencies

- Rust 1.70 or later (1.80+ recommended)
- Meson 0.59 or later
- Cargo
- glib-compile-resources
- glib-compile-schemas
- gettext

### Ubuntu 24.04 LTS

On Ubuntu 24.04 LTS, you can install all dependencies with:

```bash
sudo apt install build-essential meson cargo rustc \
    libgtk-4-dev libadwaita-1-dev libglib2.0-dev \
    gettext appstream desktop-file-utils
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
- Designed for modern GNOME desktop environments on Ubuntu 24.04 LTS and later
