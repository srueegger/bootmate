// SPDX-License-Identifier: GPL-2.0-only

use crate::config::VERSION;
use crate::window::BootMateWindow;

use libadwaita as adw;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct BootMateApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for BootMateApplication {
        const NAME: &'static str = "BootMateApplication";
        type Type = super::BootMateApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for BootMateApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("window.close", &["<primary>w"]);
        }
    }

    impl ApplicationImpl for BootMateApplication {
        fn activate(&self) {
            let application = self.obj();
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = BootMateWindow::new(&*application);
                window.upcast()
            };
            window.present();
        }
    }

    impl GtkApplicationImpl for BootMateApplication {}
    impl AdwApplicationImpl for BootMateApplication {}
}

glib::wrapper! {
    pub struct BootMateApplication(ObjectSubclass<imp::BootMateApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl BootMateApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();

        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();

        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_name(gettext("Boot Mate"))
            .application_icon("ch.srueegger.bootmate")
            .developer_name("Samuel Rüegger")
            .version(VERSION)
            .developers(vec!["Samuel Rüegger"])
            .license_type(gtk::License::Gpl20Only)
            .comments(gettext("Manage autostart entries"))
            .website("https://github.com/srueegger/bootmate")
            .issue_url("https://github.com/srueegger/bootmate/issues")
            .build();

        about.present(Some(&window));
    }
}
