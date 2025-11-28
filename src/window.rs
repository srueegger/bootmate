
// SPDX-License-Identifier: GPL-2.0-only

use crate::autostart::AutostartEntry;
use crate::entry_row::EntryRow;
use libadwaita as adw;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::prelude::IsA;
use gtk::{gio, glib};
use std::cell::RefCell;
use std::rc::Rc;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/ch/srueegger/bootmate/ui/window.ui")]
    pub struct BootMateWindow {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub main_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub status_page: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub scrolled_window: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub user_group: TemplateChild<gtk::Box>,
        #[template_child]
        pub user_list_box: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub system_group: TemplateChild<gtk::Box>,
        #[template_child]
        pub system_list_box: TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BootMateWindow {
        const NAME: &'static str = "BootMateWindow";
        type Type = super::BootMateWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BootMateWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_actions();
            obj.load_autostart_entries();
        }
    }

    impl WidgetImpl for BootMateWindow {}
    impl WindowImpl for BootMateWindow {}
    impl ApplicationWindowImpl for BootMateWindow {}
    impl AdwApplicationWindowImpl for BootMateWindow {}
}

glib::wrapper! {
    pub struct BootMateWindow(ObjectSubclass<imp::BootMateWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl BootMateWindow {
    pub fn new<P: IsA<adw::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn setup_actions(&self) {
        let action_refresh = gio::ActionEntry::builder("refresh")
            .activate(|window: &Self, _, _| {
                window.load_autostart_entries();
            })
            .build();

        let action_add_entry = gio::ActionEntry::builder("add-entry")
            .activate(|window: &Self, _, _| {
                window.show_add_entry_dialog();
            })
            .build();

        self.add_action_entries([action_refresh, action_add_entry]);
    }

    pub fn load_autostart_entries(&self) {
        let imp = self.imp();

        // Clear existing entries from both lists
        while let Some(child) = imp.user_list_box.first_child() {
            imp.user_list_box.remove(&child);
        }
        while let Some(child) = imp.system_list_box.first_child() {
            imp.system_list_box.remove(&child);
        }

        // Load autostart entries
        let entries = AutostartEntry::load_all();

        if entries.is_empty() {
            imp.status_page.set_title(&gettext("No Autostart Entries"));
            imp.status_page.set_description(Some(&gettext(
                "No applications are configured to start automatically"
            )));
            imp.main_stack.set_visible_child_name("empty");
        } else {
            // Separate entries into user and system
            let mut user_entries = Vec::new();
            let mut system_entries = Vec::new();

            for entry in entries {
                if entry.is_user_entry {
                    user_entries.push(entry);
                } else {
                    system_entries.push(entry);
                }
            }

            // Add user entries to user list
            for entry in &user_entries {
                let row = EntryRow::new(entry);
                imp.user_list_box.append(&row);
            }

            // Add system entries to system list
            for entry in &system_entries {
                let row = EntryRow::new(entry);
                imp.system_list_box.append(&row);
            }

            // Show/hide groups based on whether they have entries
            imp.user_group.set_visible(!user_entries.is_empty());
            imp.system_group.set_visible(!system_entries.is_empty());

            imp.main_stack.set_visible_child_name("list");
        }
    }

    fn show_add_entry_dialog(&self) {
        let dialog = adw::AlertDialog::builder()
            .heading(gettext("Add Autostart Entry"))
            .build();

        dialog.add_response("cancel", &gettext("Cancel"));
        dialog.add_response("add", &gettext("Add"));
        dialog.set_response_appearance("add", adw::ResponseAppearance::Suggested);
        dialog.set_default_response(Some("add"));
        dialog.set_close_response("cancel");

        // Create the form with two modes: program selection or custom command
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 12);

        // Name entry
        let name_row = adw::EntryRow::builder()
            .title(gettext("Name"))
            .build();

        // Mode switch group (for future use)
        let _mode_group = adw::PreferencesGroup::builder()
            .title(gettext("Entry Type"))
            .build();

        // Radio buttons for mode selection
        let use_program_check = gtk::CheckButton::builder()
            .label(gettext("Select from installed applications"))
            .active(true)
            .build();

        let use_custom_check = gtk::CheckButton::builder()
            .label(gettext("Enter custom command"))
            .group(&use_program_check)
            .build();

        let radio_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        radio_box.append(&use_program_check);
        radio_box.append(&use_custom_check);

        // Program dropdown (initially visible)
        let program_row = adw::ComboRow::builder()
            .title(gettext("Application"))
            .build();

        // Load available .desktop files from /usr/share/applications
        let programs = gtk::StringList::new(&[]);
        let applications_dir = std::path::Path::new("/usr/share/applications");
        let app_commands: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

        if let Ok(entries) = std::fs::read_dir(applications_dir) {
            let mut app_list: Vec<(String, String)> = Vec::new();

            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                    if let Ok(entry_data) = AutostartEntry::from_file(&path) {
                        app_list.push((entry_data.name.clone(), entry_data.exec.clone()));
                    }
                }
            }

            // Sort by name
            app_list.sort_by(|a, b| a.0.cmp(&b.0));

            // Add to the list and store commands
            for (name, command) in &app_list {
                programs.append(name);
                app_commands.borrow_mut().push(command.clone());
            }
        }

        program_row.set_model(Some(&programs));

        // Custom command entry (initially hidden)
        let command_row = adw::EntryRow::builder()
            .title(gettext("Command"))
            .visible(false)
            .build();

        // File chooser button for custom command
        let file_button = gtk::Button::builder()
            .label(gettext("Browse..."))
            .valign(gtk::Align::Center)
            .build();
        command_row.add_suffix(&file_button);

        // Toggle visibility based on mode
        use_program_check.connect_toggled(glib::clone!(
            #[weak] program_row,
            #[weak] command_row,
            move |button| {
                let use_program = button.is_active();
                program_row.set_visible(use_program);
                command_row.set_visible(!use_program);
            }
        ));

        // Build the preferences group
        let preferences_group = adw::PreferencesGroup::new();
        preferences_group.add(&name_row);
        main_box.append(&radio_box);
        preferences_group.add(&program_row);
        preferences_group.add(&command_row);

        main_box.append(&preferences_group);
        dialog.set_extra_child(Some(&main_box));

        // Handle file chooser
        file_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)] self,
            #[weak] command_row,
            move |_| {
                let file_dialog = gtk::FileDialog::new();
                file_dialog.open(
                    Some(&window),
                    gio::Cancellable::NONE,
                    glib::clone!(
                        #[weak] command_row,
                        move |result| {
                            if let Ok(file) = result {
                                if let Some(path) = file.path() {
                                    command_row.set_text(&path.display().to_string());
                                }
                            }
                        }
                    ),
                );
            }
        ));

        // Handle add response
        dialog.connect_response(
            Some("add"),
            glib::clone!(
                #[weak(rename_to = window)] self,
                #[weak] name_row,
                #[weak] command_row,
                #[weak] use_program_check,
                #[weak] program_row,
                move |_, _| {
                    let name = name_row.text();
                    let command = if use_program_check.is_active() {
                        // Get selected program command
                        let selected = program_row.selected();
                        if selected == gtk::INVALID_LIST_POSITION {
                            return;
                        }
                        match app_commands.borrow().get(selected as usize) {
                            Some(cmd) => cmd.clone(),
                            None => return,
                        }
                    } else {
                        command_row.text().to_string()
                    };

                    if name.is_empty() || command.is_empty() {
                        // Show error notification
                        return;
                    }

                    // Generate a filename from the entry name
                    let filename = name
                        .to_lowercase()
                        .replace(" ", "-")
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == '-')
                        .collect::<String>()
                        + ".desktop";

                    let user_dir = glib::user_config_dir();
                    let file_path = user_dir.join("autostart").join(&filename);

                    // Create new autostart entry
                    let entry = AutostartEntry {
                        name: name.to_string(),
                        exec: command,
                        icon: None,
                        comment: None,
                        enabled: true,
                        file_path,
                        is_user_entry: true,
                    };

                    if let Err(e) = entry.save(&entry.exec) {
                        eprintln!("Failed to save entry: {}", e);
                    } else {
                        window.load_autostart_entries();
                    }
                }
            ),
        );

        dialog.present(Some(self));
    }
}
