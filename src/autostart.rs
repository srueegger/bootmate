// SPDX-License-Identifier: GPL-2.0-only

use gtk::glib;
use std::collections::HashSet;
use std::ffi::OsString;
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

        Self::parse(&content, path).map(|(entry, _hidden)| entry)
    }

    /// Parse the content of a .desktop file.
    /// Returns the entry and whether it is marked as `Hidden=true`.
    fn parse(content: &str, path: &Path) -> Result<(Self, bool), String> {
        let mut name = None;
        let mut exec = None;
        let mut icon = None;
        let mut comment = None;
        let mut autostart_enabled = true;
        let mut hidden = false;
        let mut in_desktop_entry = false;
        let mut flatpak_id = None;

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
                    "X-Flatpak" => flatpak_id = Some(value.to_string()),
                    "X-XDP-Autostart" => {
                        if flatpak_id.is_none() {
                            flatpak_id = Some(value.to_string());
                        }
                    }
                    "X-GNOME-Autostart-enabled" => {
                        autostart_enabled = value.to_lowercase() != "false";
                    }
                    "Hidden" => {
                        hidden = value.to_lowercase() == "true";
                    }
                    _ => {}
                }
            }
        }

        let name = name.ok_or("Missing Name field")?;
        let exec = exec.ok_or("Missing Exec field")?;

        // If no Icon= was set, derive from X-Flatpak/X-XDP-Autostart ID,
        // then fall back to the filename stem (works for most Flatpak-generated entries)
        if icon.is_none() {
            if let Some(id) = flatpak_id {
                icon = Some(id);
            } else if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                icon = Some(stem.to_string());
            }
        }

        let is_user_entry = path.starts_with(Self::user_autostart_path());

        Ok((
            AutostartEntry {
                name,
                exec,
                icon,
                comment,
                enabled: autostart_enabled && !hidden,
                file_path: path.to_path_buf(),
                is_user_entry,
            },
            hidden,
        ))
    }

    /// Detect which sandbox environment we're running in
    fn detect_sandbox() -> SandboxType {
        if std::env::var("FLATPAK_ID").is_ok() {
            SandboxType::Flatpak
        } else {
            SandboxType::None
        }
    }

    /// Get the host path prefix for system directories.
    /// In Flatpak, host paths are mounted at /run/host/.
    fn host_prefix() -> &'static str {
        if Self::detect_sandbox() == SandboxType::Flatpak {
            "/run/host"
        } else {
            ""
        }
    }

    /// Get the user autostart directory path.
    /// In Flatpak, glib::user_config_dir() returns the sandboxed path
    /// (~/.var/app/<id>/config/), but we need the real ~/.config/autostart
    /// which is accessible via --filesystem=xdg-config/autostart.
    pub fn user_autostart_path() -> PathBuf {
        if Self::detect_sandbox() == SandboxType::Flatpak {
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home).join(".config").join("autostart");
            }
        }
        glib::user_config_dir().join("autostart")
    }

    /// Get the path to /etc/xdg/autostart (host-aware)
    pub fn etc_xdg_autostart_path() -> PathBuf {
        PathBuf::from(format!("{}/etc/xdg/autostart", Self::host_prefix()))
    }

    /// Get the path to /usr/share/gnome/autostart (host-aware)
    pub fn usr_share_gnome_autostart_path() -> PathBuf {
        PathBuf::from(format!("{}/usr/share/gnome/autostart", Self::host_prefix()))
    }

    /// Get the path to /usr/share/applications (host-aware)
    pub fn usr_share_applications_path() -> PathBuf {
        PathBuf::from(format!("{}/usr/share/applications", Self::host_prefix()))
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
        let user_autostart = Self::user_autostart_path();
        access.user_autostart = fs::read_dir(&user_autostart).is_ok() ||
                               fs::create_dir_all(&user_autostart).is_ok();

        // Check system directories (use host-aware paths). A directory that
        // simply does not exist on this system (e.g. /usr/share/gnome/autostart
        // on Ubuntu) is not a missing permission as long as its host mount is
        // readable.
        let accessible = |dir: PathBuf, mount: &str| {
            fs::read_dir(&dir).is_ok()
                || (!dir.exists()
                    && fs::read_dir(format!("{}{}", Self::host_prefix(), mount)).is_ok())
        };
        access.etc_xdg_autostart = accessible(Self::etc_xdg_autostart_path(), "/etc");
        access.usr_share_gnome_autostart =
            accessible(Self::usr_share_gnome_autostart_path(), "/usr");
        access.usr_share_applications = accessible(Self::usr_share_applications_path(), "/usr");

        access
    }

    /// Get all autostart entries from system and user directories
    pub fn load_all() -> Vec<Self> {
        Self::load_from(
            &Self::user_autostart_path(),
            &[
                Self::etc_xdg_autostart_path(),
                Self::usr_share_gnome_autostart_path(),
            ],
        )
    }

    /// Load entries from the given directories.
    ///
    /// As in the XDG autostart specification, a file overrides files with the
    /// same *file name* in directories of lower precedence (user directory
    /// first, then the system directories in the given order).
    ///
    /// A user file that overrides a system file is still reported as a system
    /// entry (`is_user_entry == false`): removing the user file would only
    /// bring the system entry back. Such overrides marked `Hidden=true` are
    /// deleted system entries and are not listed at all.
    fn load_from(user_dir: &Path, system_dirs: &[PathBuf]) -> Vec<Self> {
        let mut entries = Vec::new();
        let mut seen: HashSet<OsString> = HashSet::new();

        for path in Self::desktop_files(user_dir) {
            let Some(filename) = path.file_name().map(|f| f.to_os_string()) else {
                continue;
            };
            let overrides_system = system_dirs.iter().any(|dir| dir.join(&filename).is_file());

            // The user file takes precedence even if we cannot parse it
            // (e.g. a minimal override containing only Hidden=true).
            seen.insert(filename);

            let Ok(content) = fs::read_to_string(&path) else {
                continue;
            };
            let Ok((mut entry, hidden)) = Self::parse(&content, &path) else {
                continue;
            };

            if hidden && overrides_system {
                continue;
            }

            entry.is_user_entry = !overrides_system;
            entries.push(entry);
        }

        for dir in system_dirs {
            for path in Self::desktop_files(dir) {
                let Some(filename) = path.file_name().map(|f| f.to_os_string()) else {
                    continue;
                };
                if !seen.insert(filename) {
                    continue;
                }
                if let Ok(mut entry) = Self::from_file(&path) {
                    entry.is_user_entry = false;
                    entries.push(entry);
                }
            }
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));
        entries
    }

    /// List all .desktop files in a directory (sorted, empty if unreadable)
    fn desktop_files(dir: &Path) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = fs::read_dir(dir)
            .map(|entries| {
                entries
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("desktop"))
                    .collect()
            })
            .unwrap_or_default();
        files.sort();
        files
    }

    /// Delete this autostart entry
    pub fn delete(&self) -> Result<(), String> {
        self.delete_in(&Self::user_autostart_path())
    }

    fn delete_in(&self, user_dir: &Path) -> Result<(), String> {
        if self.is_user_entry {
            // Pure user entries can simply be removed
            fs::remove_file(&self.file_path)
                .map_err(|e| format!("Failed to delete file: {}", e))
        } else {
            // System entries cannot be removed; hide them with a user override
            self.write_user_file(user_dir, &[("Hidden", Some("true"))])
        }
    }

    /// Save a changed command for this entry
    pub fn save(&self, new_exec: &str) -> Result<(), String> {
        self.save_in(&Self::user_autostart_path(), new_exec)
    }

    fn save_in(&self, user_dir: &Path, new_exec: &str) -> Result<(), String> {
        self.write_user_file(user_dir, &[("Exec", Some(new_exec))])
    }

    /// Set the enabled state of this entry
    pub fn set_enabled(&self, enabled: bool) -> Result<(), String> {
        self.set_enabled_in(&Self::user_autostart_path(), enabled)
    }

    fn set_enabled_in(&self, user_dir: &Path, enabled: bool) -> Result<(), String> {
        if enabled {
            self.write_user_file(
                user_dir,
                &[("X-GNOME-Autostart-enabled", Some("true")), ("Hidden", None)],
            )
        } else {
            self.write_user_file(user_dir, &[("X-GNOME-Autostart-enabled", Some("false"))])
        }
    }

    /// Write this entry to the user autostart directory, applying `updates`
    /// (key, new value or `None` to remove the key) to the `[Desktop Entry]`
    /// group. Everything else in the original file is kept as it is.
    fn write_user_file(
        &self,
        user_dir: &Path,
        updates: &[(&str, Option<&str>)],
    ) -> Result<(), String> {
        fs::create_dir_all(user_dir)
            .map_err(|e| format!("Failed to create autostart directory: {}", e))?;

        let filename = self.file_path.file_name().ok_or("Invalid file name")?;
        let user_file = user_dir.join(filename);

        let base = fs::read_to_string(&self.file_path).unwrap_or_else(|_| self.new_file_content());
        let content = update_desktop_content(&base, updates);

        fs::write(&user_file, content).map_err(|e| format!("Failed to write file: {}", e))
    }

    /// Content for an entry that does not exist on disk yet
    fn new_file_content(&self) -> String {
        let mut content = String::from("[Desktop Entry]\n");
        content.push_str("Type=Application\n");
        content.push_str(&format!("Name={}\n", single_line(&self.name)));
        content.push_str(&format!("Exec={}\n", single_line(&self.exec)));
        content.push_str("Terminal=false\n");

        if let Some(icon) = &self.icon {
            content.push_str(&format!("Icon={}\n", single_line(icon)));
        }

        if let Some(comment) = &self.comment {
            content.push_str(&format!("Comment={}\n", single_line(comment)));
        }

        content.push_str(&format!("X-GNOME-Autostart-enabled={}\n", self.enabled));
        content
    }
}

/// Values in .desktop files must not span multiple lines
fn single_line(value: &str) -> String {
    value.replace(['\n', '\r'], " ")
}

/// Key of a `Key=Value` line, `None` for comments, blank lines and group headers
fn line_key(line: &str) -> Option<&str> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
        return None;
    }
    line.split_once('=').map(|(key, _)| key.trim())
}

/// Set or remove keys in the `[Desktop Entry]` group of a .desktop file while
/// keeping all other lines (unknown keys, translations, comments, other
/// groups) untouched.
fn update_desktop_content(content: &str, updates: &[(&str, Option<&str>)]) -> String {
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();

    let start = match lines.iter().position(|l| l.trim() == "[Desktop Entry]") {
        Some(index) => index,
        None => {
            lines.insert(0, "[Desktop Entry]".to_string());
            0
        }
    };

    for &(key, value) in updates {
        let end = (start + 1..lines.len())
            .find(|&i| lines[i].trim_start().starts_with('['))
            .unwrap_or(lines.len());
        let matches: Vec<usize> = (start + 1..end)
            .filter(|&i| line_key(&lines[i]) == Some(key))
            .collect();

        match value {
            Some(value) => {
                let new_line = format!("{}={}", key, single_line(value));
                if let Some((&first, duplicates)) = matches.split_first() {
                    lines[first] = new_line;
                    for &i in duplicates.iter().rev() {
                        lines.remove(i);
                    }
                } else {
                    // Append to the group, before any trailing blank lines
                    let mut insert_at = end;
                    while insert_at > start + 1 && lines[insert_at - 1].trim().is_empty() {
                        insert_at -= 1;
                    }
                    lines.insert(insert_at, new_line);
                }
            }
            None => {
                for &i in matches.iter().rev() {
                    lines.remove(i);
                }
            }
        }
    }

    let mut result = lines.join("\n");
    result.push('\n');
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SYSTEM_FILE: &str = "\
# A comment
[Desktop Entry]
Type=Application
Name=Key Ring
Name[de]=Schlüsselbund
Exec=/usr/bin/keyring --start
Terminal=true
OnlyShowIn=GNOME;
X-GNOME-Autostart-Phase=PreDisplayServer

[Desktop Action Foo]
Name=Foo
Exec=foo
";

    struct Dirs {
        root: PathBuf,
        user: PathBuf,
        etc: PathBuf,
        gnome: PathBuf,
    }

    impl Dirs {
        fn new(test: &str) -> Self {
            let root = std::env::temp_dir()
                .join(format!("bootmate-test-{}-{}", std::process::id(), test));
            let _ = fs::remove_dir_all(&root);
            let dirs = Dirs {
                user: root.join("user"),
                etc: root.join("etc"),
                gnome: root.join("gnome"),
                root,
            };
            for dir in [&dirs.user, &dirs.etc, &dirs.gnome] {
                fs::create_dir_all(dir).unwrap();
            }
            dirs
        }

        fn load(&self) -> Vec<AutostartEntry> {
            AutostartEntry::load_from(&self.user, &[self.etc.clone(), self.gnome.clone()])
        }
    }

    impl Drop for Dirs {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn update_replaces_only_the_requested_key() {
        let updated = update_desktop_content(SYSTEM_FILE, &[("Exec", Some("/bin/new"))]);
        assert_eq!(
            updated,
            SYSTEM_FILE.replace("Exec=/usr/bin/keyring --start", "Exec=/bin/new")
        );
    }

    #[test]
    fn update_appends_missing_key_inside_desktop_entry_group() {
        let updated =
            update_desktop_content(SYSTEM_FILE, &[("X-GNOME-Autostart-enabled", Some("false"))]);
        assert_eq!(
            updated,
            SYSTEM_FILE.replace(
                "X-GNOME-Autostart-Phase=PreDisplayServer\n",
                "X-GNOME-Autostart-Phase=PreDisplayServer\nX-GNOME-Autostart-enabled=false\n"
            )
        );
    }

    #[test]
    fn update_removes_keys_and_duplicates() {
        let content = "[Desktop Entry]\nName=A\nHidden=true\nExec=a\nHidden=true\n";
        assert_eq!(
            update_desktop_content(content, &[("Hidden", None)]),
            "[Desktop Entry]\nName=A\nExec=a\n"
        );
        assert_eq!(
            update_desktop_content(content, &[("Hidden", Some("false"))]),
            "[Desktop Entry]\nName=A\nHidden=false\nExec=a\n"
        );
    }

    #[test]
    fn update_does_not_touch_other_groups() {
        let updated = update_desktop_content(SYSTEM_FILE, &[("Name", Some("Renamed"))]);
        assert!(updated.contains("[Desktop Action Foo]\nName=Foo\n"));
        assert!(updated.contains("Name=Renamed\nName[de]=Schlüsselbund\n"));
    }

    #[test]
    fn update_keeps_values_on_one_line() {
        let updated = update_desktop_content("[Desktop Entry]\n", &[("Exec", Some("a\nHidden=true"))]);
        assert_eq!(updated, "[Desktop Entry]\nExec=a Hidden=true\n");
    }

    #[test]
    fn hidden_wins_regardless_of_key_order() {
        let content = "[Desktop Entry]\nName=A\nExec=a\nHidden=true\nX-GNOME-Autostart-enabled=true\n";
        let (entry, hidden) = AutostartEntry::parse(content, Path::new("/tmp/a.desktop")).unwrap();
        assert!(hidden);
        assert!(!entry.enabled);
    }

    #[test]
    fn disabling_a_system_entry_keeps_all_keys_and_stays_a_system_entry() {
        let dirs = Dirs::new("disable-system");
        fs::write(dirs.etc.join("keyring.desktop"), SYSTEM_FILE).unwrap();

        let entry = dirs.load().remove(0);
        assert!(!entry.is_user_entry);
        assert!(entry.enabled);
        entry.set_enabled_in(&dirs.user, false).unwrap();

        let written = fs::read_to_string(dirs.user.join("keyring.desktop")).unwrap();
        for line in SYSTEM_FILE.lines() {
            assert!(written.contains(line), "lost line: {line}");
        }
        assert!(written.contains("X-GNOME-Autostart-enabled=false"));

        let entries = dirs.load();
        assert_eq!(entries.len(), 1);
        assert!(!entries[0].enabled);
        assert!(!entries[0].is_user_entry);

        // Re-enabling works on the override and keeps the file intact
        entries[0].set_enabled_in(&dirs.user, true).unwrap();
        let entries = dirs.load();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].enabled);
        let written = fs::read_to_string(dirs.user.join("keyring.desktop")).unwrap();
        assert!(written.contains("Terminal=true"));
    }

    #[test]
    fn deleting_a_system_entry_removes_it_from_the_list() {
        let dirs = Dirs::new("delete-system");
        fs::write(dirs.etc.join("keyring.desktop"), SYSTEM_FILE).unwrap();

        dirs.load().remove(0).delete_in(&dirs.user).unwrap();

        assert!(dirs.etc.join("keyring.desktop").exists());
        assert!(dirs.load().is_empty());
    }

    #[test]
    fn deleting_a_disabled_system_entry_does_not_bring_it_back() {
        let dirs = Dirs::new("delete-disabled-system");
        fs::write(dirs.etc.join("keyring.desktop"), SYSTEM_FILE).unwrap();

        dirs.load().remove(0).set_enabled_in(&dirs.user, false).unwrap();
        dirs.load().remove(0).delete_in(&dirs.user).unwrap();

        assert!(dirs.load().is_empty());
        assert!(dirs.user.join("keyring.desktop").exists());
    }

    #[test]
    fn minimal_hidden_override_masks_system_entry() {
        let dirs = Dirs::new("minimal-hidden");
        fs::write(dirs.etc.join("keyring.desktop"), SYSTEM_FILE).unwrap();
        fs::write(dirs.user.join("keyring.desktop"), "[Desktop Entry]\nHidden=true\n").unwrap();

        assert!(dirs.load().is_empty());
    }

    #[test]
    fn overrides_are_matched_by_file_name_not_by_name() {
        let dirs = Dirs::new("dedupe");
        fs::write(dirs.etc.join("a.desktop"), "[Desktop Entry]\nName=System A\nExec=a\n").unwrap();
        fs::write(dirs.gnome.join("a.desktop"), "[Desktop Entry]\nName=Gnome A\nExec=a\n").unwrap();
        fs::write(dirs.user.join("a.desktop"), "[Desktop Entry]\nName=My A\nExec=a\n").unwrap();
        // Same Name as another entry, but a different file: must not be dropped
        fs::write(dirs.gnome.join("b.desktop"), "[Desktop Entry]\nName=My A\nExec=b\n").unwrap();
        fs::write(dirs.user.join("c.desktop"), "[Desktop Entry]\nName=Own\nExec=c\n").unwrap();

        let entries = dirs.load();
        let summary: Vec<(String, String, bool)> = entries
            .iter()
            .map(|e| (e.name.clone(), e.exec.clone(), e.is_user_entry))
            .collect();
        assert_eq!(
            summary,
            vec![
                ("My A".to_string(), "a".to_string(), false),
                ("My A".to_string(), "b".to_string(), false),
                ("Own".to_string(), "c".to_string(), true),
            ]
        );
    }

    #[test]
    fn editing_a_user_entry_keeps_unknown_keys() {
        let dirs = Dirs::new("edit-user");
        let content = "[Desktop Entry]\nType=Application\nName=App\nName[de]=Programm\nExec=old\nTerminal=true\nX-Flatpak=org.example.App\nX-GNOME-Autostart-enabled=false\n";
        fs::write(dirs.user.join("app.desktop"), content).unwrap();

        dirs.load().remove(0).save_in(&dirs.user, "new --flag").unwrap();

        assert_eq!(
            fs::read_to_string(dirs.user.join("app.desktop")).unwrap(),
            content.replace("Exec=old", "Exec=new --flag")
        );
    }

    #[test]
    fn deleting_a_user_entry_removes_the_file() {
        let dirs = Dirs::new("delete-user");
        fs::write(dirs.user.join("app.desktop"), "[Desktop Entry]\nName=App\nExec=app\n").unwrap();

        dirs.load().remove(0).delete_in(&dirs.user).unwrap();

        assert!(!dirs.user.join("app.desktop").exists());
    }

    #[test]
    fn new_entries_get_a_complete_file() {
        let dirs = Dirs::new("new-entry");
        let entry = AutostartEntry {
            name: "My App".to_string(),
            exec: "my-app".to_string(),
            icon: None,
            comment: None,
            enabled: true,
            file_path: dirs.user.join("my-app.desktop"),
            is_user_entry: true,
        };
        entry.save_in(&dirs.user, &entry.exec).unwrap();

        assert_eq!(
            fs::read_to_string(dirs.user.join("my-app.desktop")).unwrap(),
            "[Desktop Entry]\nType=Application\nName=My App\nExec=my-app\nTerminal=false\nX-GNOME-Autostart-enabled=true\n"
        );
        let entries = dirs.load();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].is_user_entry && entries[0].enabled);
    }
}
