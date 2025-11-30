// SPDX-License-Identifier: GPL-2.0-only

mod application;
mod autostart;
mod config;
mod entry_row;
mod window;

use application::BootMateApplication;
use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR};

use gettextrs::{bind_textdomain_codeset, bindtextdomain, setlocale, textdomain, LocaleCategory};
use gtk::prelude::*;
use gtk::{gio, glib};

fn main() -> glib::ExitCode {
    // Initialize locale from environment
    setlocale(LocaleCategory::LcAll, "");

    // Initialize gettext
    // For dev builds, try to use locale files from build directory
    // Check if our translation file actually exists in the system locale dir
    let system_locale_file = std::path::PathBuf::from(LOCALEDIR)
        .join("de/LC_MESSAGES")
        .join(format!("{}.mo", GETTEXT_PACKAGE));

    let locale_dir = if system_locale_file.exists() {
        LOCALEDIR.to_string()
    } else {
        // Dev build: try multiple paths to find locale files
        let exe_path = std::env::current_exe().unwrap_or_default();

        // Try build/po relative to executable (meson build)
        let meson_po = exe_path
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join("po"));

        // Try build/po relative to project root (cargo build)
        let cargo_po = std::env::current_dir()
            .ok()
            .map(|p| p.join("build/po"));

        // Use the first path that exists
        let build_po = meson_po
            .as_ref()
            .filter(|p| p.exists())
            .or(cargo_po.as_ref().filter(|p| p.exists()))
            .cloned()
            .unwrap_or_else(|| std::path::PathBuf::from("build/po"));

        build_po.to_string_lossy().to_string()
    };

    bindtextdomain(GETTEXT_PACKAGE, &locale_dir).expect("Failed to bind text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Failed to set text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Failed to set text domain");

    // Load resources
    gio::resources_register_include!("bootmate.gresource")
        .expect("Failed to register resources");

    // Create and run the application
    let app = BootMateApplication::new(APP_ID, &gio::ApplicationFlags::default());
    app.run()
}
