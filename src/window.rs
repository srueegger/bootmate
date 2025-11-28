// SPDX-License-Identifier: GPL-2.0-only

use crate::autostart::AutostartEntry;
use crate::entry_row::EntryRow;
use libadwaita as adw;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::prelude::IsA;
use gtk::{gio, glib};

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

        self.add_action_entries([action_refresh]);
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
}
