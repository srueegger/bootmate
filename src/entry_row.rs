// SPDX-License-Identifier: GPL-2.0-only

use crate::autostart::AutostartEntry;
use libadwaita as adw;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::glib;
use std::cell::RefCell;

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
        let prefix_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);

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

        // Set icon
        let icon = if let Some(icon_name) = &entry.icon {
            gtk::Image::from_icon_name(icon_name)
        } else {
            gtk::Image::from_icon_name("application-x-executable")
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
        enable_switch.connect_state_set(glib::clone!(
            #[weak] row,
            #[upgrade_or] glib::Propagation::Proceed,
            move |switch, enabled| {
                // Update visual state immediately
                if enabled {
                    row.set_opacity(1.0);
                    row.remove_css_class("dim-label");
                    switch.set_tooltip_text(Some(&gettext("Enabled")));
                } else {
                    row.set_opacity(0.5);
                    row.add_css_class("dim-label");
                    switch.set_tooltip_text(Some(&gettext("Disabled")));
                }

                // Save the change
                let mut modified_entry = entry_clone.clone();
                modified_entry.enabled = enabled;

                if let Err(e) = modified_entry.set_enabled(enabled) {
                    eprintln!("Failed to set enabled state: {}", e);
                    // Revert the switch state on error
                    switch.set_active(!enabled);
                    row.set_opacity(if !enabled { 1.0 } else { 0.5 });
                    if !enabled {
                        row.remove_css_class("dim-label");
                    } else {
                        row.add_css_class("dim-label");
                    }
                    return glib::Propagation::Stop;
                }

                // Refresh the entire list to ensure consistency
                if let Some(window) = row.root().and_downcast::<crate::window::BootMateWindow>() {
                    window.load_autostart_entries();
                }

                glib::Propagation::Proceed
            }
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
                    let new_exec = entry_widget.text();
                    if let Err(e) = entry_clone.save(&new_exec) {
                        eprintln!("Failed to save entry: {}", e);
                    } else {
                        // Refresh the list
                        if let Some(window) = row.root().and_downcast::<crate::window::BootMateWindow>() {
                            window.load_autostart_entries();
                        }
                    }
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
                    if let Err(e) = entry_clone.delete() {
                        eprintln!("Failed to delete entry: {}", e);
                    } else {
                        // Refresh the list
                        if let Some(window) = row.root().and_downcast::<crate::window::BootMateWindow>() {
                            window.load_autostart_entries();
                        }
                    }
                }
            ),
        );

        dialog.present(Some(&window));
    }
}
