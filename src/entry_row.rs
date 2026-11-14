// SPDX-License-Identifier: GPL-2.0-only

use crate::autostart::AutostartEntry;
use libadwaita as adw;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::glib;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct EntryRow {
        pub entry: RefCell<Option<AutostartEntry>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EntryRow {
        const NAME: &'static str = "BootMateEntryRow";
        type Type = super::EntryRow;
        type ParentType = adw::ActionRow;
    }

    impl ObjectImpl for EntryRow {}
    impl WidgetImpl for EntryRow {}
    impl ListBoxRowImpl for EntryRow {}
    impl PreferencesRowImpl for EntryRow {}
    impl ActionRowImpl for EntryRow {}
}

glib::wrapper! {
    pub struct EntryRow(ObjectSubclass<imp::EntryRow>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EntryRow {
    pub fn new(entry: &AutostartEntry) -> Self {
        let row: Self = glib::Object::new();

        // Escape markup characters in title and subtitle to prevent parsing errors
        row.set_title(&glib::markup_escape_text(&entry.name));

        // Set subtitle (exec command)
        row.set_subtitle(&glib::markup_escape_text(&entry.exec));

        // Create a horizontal box for prefix (switch + icon)
        let prefix_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);

        // Add enable/disable switch
        let enable_switch = gtk::Switch::builder()
            .active(entry.enabled)
            .valign(gtk::Align::Center)
            .tooltip_text(if entry.enabled {
                gettext("Enabled")
            } else {
                gettext("Disabled")
            })
            .build();

        prefix_box.append(&enable_switch);

        // Set icon with larger size
        let icon = if let Some(icon_str) = &entry.icon {
            if icon_str.starts_with('/') {
                // Absolute path: in Flatpak, also try the host-prefixed path
                let path = if std::path::Path::new(icon_str.as_str()).exists() {
                    icon_str.clone()
                } else if std::env::var("FLATPAK_ID").is_ok() {
                    format!("/run/host{}", icon_str)
                } else {
                    icon_str.clone()
                };
                gtk::Image::builder()
                    .file(&path)
                    .pixel_size(32)
                    .build()
            } else {
                gtk::Image::builder()
                    .icon_name(icon_str)
                    .pixel_size(32)
                    .build()
            }
        } else {
            gtk::Image::builder()
                .icon_name("application-x-executable")
                .pixel_size(32)
                .build()
        };

        prefix_box.append(&icon);
        row.add_prefix(&prefix_box);

        // Apply visual styling for disabled entries
        if !entry.enabled {
            row.set_opacity(0.5);
            row.add_css_class("dim-label");
        }

        // Add Edit button
        let edit_button = gtk::Button::builder()
            .icon_name("document-edit-symbolic")
            .valign(gtk::Align::Center)
            .tooltip_text(gettext("Edit"))
            .build();
        edit_button.add_css_class("flat");

        // Add Delete button
        let delete_button = gtk::Button::builder()
            .icon_name("user-trash-symbolic")
            .valign(gtk::Align::Center)
            .tooltip_text(gettext("Delete"))
            .build();
        delete_button.add_css_class("flat");
        delete_button.add_css_class("destructive-action");

        let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        button_box.append(&edit_button);
        button_box.append(&delete_button);

        row.add_suffix(&button_box);

        // Store entry data
        row.imp().entry.replace(Some(entry.clone()));

        // Connect switch toggle handler
        let entry_clone = entry.clone();
        let reverting = Rc::new(Cell::new(false));
        enable_switch.connect_state_set(glib::clone!(
            #[weak] row,
            #[strong] reverting,
            #[upgrade_or] glib::Propagation::Proceed,
            move |switch, enabled| row.on_switch_toggled(switch, enabled, &entry_clone, &reverting)
        ));

        // Connect edit button
        let entry_clone = entry.clone();
        edit_button.connect_clicked(glib::clone!(
            #[weak] row,
            move |_| {
                row.show_edit_dialog(&entry_clone);
            }
        ));

        // Connect delete button
        let entry_clone = entry.clone();
        delete_button.connect_clicked(glib::clone!(
            #[weak] row,
            move |_| {
                row.show_delete_dialog(&entry_clone);
            }
        ));

        row
    }

    // The handlers below are methods instead of inline `glib::clone!` closures
    // on purpose: xgettext does not look into macro invocations, so
    // translatable strings in there would be missing from the translation
    // template.

    /// Save a toggled enabled state; reverts the switch if that fails
    fn on_switch_toggled(
        &self,
        switch: &gtk::Switch,
        enabled: bool,
        entry: &AutostartEntry,
        reverting: &Cell<bool>,
    ) -> glib::Propagation {
        // Emitted by our own revert below: nothing to save, just let the
        // switch follow.
        if reverting.get() {
            return glib::Propagation::Proceed;
        }

        // Update visual state immediately
        self.show_enabled_state(switch, enabled);

        if let Err(e) = entry.set_enabled(enabled) {
            self.show_error(&gettext("Could not change entry"), &e);
            // Revert the switch state on error (without saving again)
            reverting.set(true);
            switch.set_active(!enabled);
            reverting.set(false);
            self.show_enabled_state(switch, !enabled);
            return glib::Propagation::Stop;
        }

        // Refresh the entire list to ensure consistency
        self.reload_list();

        glib::Propagation::Proceed
    }

    /// Apply the look of an enabled or disabled entry to row and switch
    fn show_enabled_state(&self, switch: &gtk::Switch, enabled: bool) {
        if enabled {
            self.set_opacity(1.0);
            self.remove_css_class("dim-label");
            switch.set_tooltip_text(Some(&gettext("Enabled")));
        } else {
            self.set_opacity(0.5);
            self.add_css_class("dim-label");
            switch.set_tooltip_text(Some(&gettext("Disabled")));
        }
    }

    fn save_command(&self, entry: &AutostartEntry, new_exec: &str) {
        if let Err(e) = entry.save(new_exec) {
            self.show_error(&gettext("Could not save entry"), &e);
        } else {
            self.reload_list();
        }
    }

    fn delete_entry(&self, entry: &AutostartEntry) {
        if let Err(e) = entry.delete() {
            self.show_error(&gettext("Could not delete entry"), &e);
        } else {
            self.reload_list();
        }
    }

    fn reload_list(&self) {
        if let Some(window) = self.root().and_downcast::<crate::window::BootMateWindow>() {
            window.load_autostart_entries();
        }
    }

    /// Report a failed file operation in the window and on stderr
    fn show_error(&self, message: &str, error: &str) {
        eprintln!("{}: {}", message, error);
        if let Some(window) = self.root().and_downcast::<crate::window::BootMateWindow>() {
            window.show_toast(&format!("{}: {}", message, error));
        }
    }

    fn show_edit_dialog(&self, entry: &AutostartEntry) {
        let window = self.root().and_downcast::<gtk::Window>().unwrap();

        let dialog = adw::AlertDialog::builder()
            .heading(gettext("Edit Autostart Entry"))
            .build();

        dialog.add_response("cancel", &gettext("Cancel"));
        dialog.add_response("save", &gettext("Save"));
        dialog.set_response_appearance("save", adw::ResponseAppearance::Suggested);
        dialog.set_default_response(Some("save"));
        dialog.set_close_response("cancel");

        // Create entry for exec command
        let entry_widget = adw::EntryRow::builder()
            .title(gettext("Command"))
            .text(&entry.exec)
            .build();

        let preferences_group = adw::PreferencesGroup::new();
        preferences_group.add(&entry_widget);

        dialog.set_extra_child(Some(&preferences_group));

        let entry_clone = entry.clone();
        dialog.connect_response(
            Some("save"),
            glib::clone!(
                #[weak(rename_to = row)] self,
                move |_, _| {
                    row.save_command(&entry_clone, &entry_widget.text());
                }
            ),
        );

        dialog.present(Some(&window));
    }

    fn show_delete_dialog(&self, entry: &AutostartEntry) {
        let window = self.root().and_downcast::<gtk::Window>().unwrap();

        let dialog = adw::AlertDialog::builder()
            .heading(gettext("Delete Autostart Entry?"))
            .body(format!(
                "{}",
                gettext("This action cannot be undone.")
            ))
            .build();

        dialog.add_response("cancel", &gettext("Cancel"));
        dialog.add_response("delete", &gettext("Delete"));
        dialog.set_response_appearance("delete", adw::ResponseAppearance::Destructive);
        dialog.set_default_response(Some("cancel"));
        dialog.set_close_response("cancel");

        let entry_clone = entry.clone();
        dialog.connect_response(
            Some("delete"),
            glib::clone!(
                #[weak(rename_to = row)] self,
                move |_, _| {
                    row.delete_entry(&entry_clone);
                }
            ),
        );

        dialog.present(Some(&window));
    }
}
