// SPDX-License-Identifier: GPL-2.0-only

use gtk::glib;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AutostartEntry {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub comment: Option<String>,
    pub enabled: bool,
    pub file_path: PathBuf,
    pub is_user_entry: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SandboxType {
    Snap,
    Flatpak,
    None,
}

#[derive(Debug, Clone)]
pub struct DirectoryAccess {
    pub user_autostart: bool,
    pub etc_xdg_autostart: bool,
    pub usr_share_gnome_autostart: bool,
    pub usr_share_applications: bool,
    pub sandbox_type: SandboxType,
}

impl AutostartEntry {
    /// Parse a .desktop file into an AutostartEntry
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let mut name = None;
        let mut exec = None;
        let mut icon = None;
        let mut comment = None;
        let mut enabled = true;
        let mut in_desktop_entry = false;

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with('[') {
                in_desktop_entry = line == "[Desktop Entry]";
                continue;
            }

            if !in_desktop_entry || line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "Name" => name = Some(value.to_string()),
                    "Exec" => exec = Some(value.to_string()),
                    "Icon" => icon = Some(value.to_string()),
                    "Comment" => comment = Some(value.to_string()),
                    "X-GNOME-Autostart-enabled" => {
                        enabled = value.to_lowercase() != "false";
                    }
                    "Hidden" => {
                        if value.to_lowercase() == "true" {
                            enabled = false;
                        }
                    }
                    _ => {}
                }
            }
        }

        let name = name.ok_or("Missing Name field")?;
        let exec = exec.ok_or("Missing Exec field")?;

        let is_user_entry = path.to_string_lossy().contains(".config/autostart");

        Ok(AutostartEntry {
            name,
            exec,
            icon,
            comment,
            enabled,
            file_path: path.to_path_buf(),
            is_user_entry,
        })
    }

    /// Detect which sandbox environment we're running in
    fn detect_sandbox() -> SandboxType {
        if std::env::var("SNAP").is_ok() || std::env::var("SNAP_NAME").is_ok() {
            SandboxType::Snap
        } else if std::env::var("FLATPAK_ID").is_ok() {
            SandboxType::Flatpak
        } else {
            SandboxType::None
        }
    }

    /// Check which autostart directories are accessible
    pub fn check_directory_access() -> DirectoryAccess {
        let sandbox_type = Self::detect_sandbox();

        let mut access = DirectoryAccess {
            user_autostart: false,
            etc_xdg_autostart: false,
            usr_share_gnome_autostart: false,
            usr_share_applications: false,
            sandbox_type,
        };

        // Check user autostart directory
        if let Some(user_dir) = glib::user_config_dir().as_path().to_str() {
            let user_autostart = PathBuf::from(user_dir).join("autostart");
            access.user_autostart = fs::read_dir(&user_autostart).is_ok() ||
                                   fs::create_dir_all(&user_autostart).is_ok();
        }

        // Check system directories
        access.etc_xdg_autostart = fs::read_dir("/etc/xdg/autostart").is_ok();
        access.usr_share_gnome_autostart = fs::read_dir("/usr/share/gnome/autostart").is_ok();
        access.usr_share_applications = fs::read_dir("/usr/share/applications").is_ok();

        access
    }

    /// Get all autostart entries from system and user directories
    pub fn load_all() -> Vec<Self> {
        let mut entries = Vec::new();
        let mut seen_names = HashMap::new();

        // User autostart directory (takes precedence)
        if let Some(user_dir) = glib::user_config_dir().as_path().to_str() {
            let user_autostart = PathBuf::from(user_dir).join("autostart");
            if let Ok(dir_entries) = fs::read_dir(&user_autostart) {
                for entry in dir_entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                        if let Ok(autostart_entry) = Self::from_file(&path) {
                            seen_names.insert(autostart_entry.name.clone(), true);
                            entries.push(autostart_entry);
                        }
                    }
                }
            }
        }

        // System autostart directories
        let system_dirs = vec![
            "/etc/xdg/autostart",
            "/usr/share/gnome/autostart",
        ];

        for dir in system_dirs {
            let autostart_dir = PathBuf::from(dir);
            if let Ok(dir_entries) = fs::read_dir(&autostart_dir) {
                for entry in dir_entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                        if let Ok(autostart_entry) = Self::from_file(&path) {
                            // Skip if user already overrode this
                            if !seen_names.contains_key(&autostart_entry.name) {
                                entries.push(autostart_entry);
                            }
                        }
                    }
                }
            }
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));
        entries
    }

    /// Delete this autostart entry
    pub fn delete(&self) -> Result<(), String> {
        if !self.is_user_entry {
            // For system entries, create a user override that hides it
            if let Some(user_dir) = glib::user_config_dir().as_path().to_str() {
                let user_autostart = PathBuf::from(user_dir).join("autostart");
                fs::create_dir_all(&user_autostart)
                    .map_err(|e| format!("Failed to create autostart directory: {}", e))?;

                let filename = self.file_path.file_name()
                    .ok_or("Invalid file name")?;
                let user_file = user_autostart.join(filename);

                let content = format!(
                    "[Desktop Entry]\nType=Application\nName={}\nExec={}\nHidden=true\n",
                    self.name, self.exec
                );

                fs::write(&user_file, content)
                    .map_err(|e| format!("Failed to write override file: {}", e))?;
            }
        } else {
            // For user entries, just delete the file
            fs::remove_file(&self.file_path)
                .map_err(|e| format!("Failed to delete file: {}", e))?;
        }
        Ok(())
    }

    /// Save changes to this entry
    pub fn save(&self, new_exec: &str) -> Result<(), String> {
        let user_dir = glib::user_config_dir();
        let user_autostart = user_dir.join("autostart");

        fs::create_dir_all(&user_autostart)
            .map_err(|e| format!("Failed to create autostart directory: {}", e))?;

        let filename = self.file_path.file_name()
            .ok_or("Invalid file name")?;
        let user_file = user_autostart.join(filename);

        let mut content = String::from("[Desktop Entry]\n");
        content.push_str("Type=Application\n");
        content.push_str(&format!("Name={}\n", self.name));
        content.push_str(&format!("Exec={}\n", new_exec));
        content.push_str("Terminal=false\n");

        if let Some(icon) = &self.icon {
            content.push_str(&format!("Icon={}\n", icon));
        }

        if let Some(comment) = &self.comment {
            content.push_str(&format!("Comment={}\n", comment));
        }

        if self.enabled {
            content.push_str("X-GNOME-Autostart-enabled=true\n");
        } else {
            content.push_str("X-GNOME-Autostart-enabled=false\n");
        }

        fs::write(&user_file, content)
            .map_err(|e| format!("Failed to save file: {}", e))?;

        Ok(())
    }
}
