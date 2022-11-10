/*
 * SPDX-FileCopyrightText: 2022 Benjamin Collet <benjamin.collet@protonmail.ch>
 *
 * SPDX-License-Identifier: CECILL-2.1
 */

use glib::clone;
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::ArchivistWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct ArchivistApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for ArchivistApplication {
        const NAME: &'static str = "ArchivistApplication";
        type Type = super::ArchivistApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for ArchivistApplication {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for ArchivistApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self, application: &Self::Type) {
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = ArchivistWindow::new(application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for ArchivistApplication {}
    impl AdwApplicationImpl for ArchivistApplication {}
}

glib::wrapper! {
    pub struct ArchivistApplication(ObjectSubclass<imp::ArchivistApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl ArchivistApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::new(&[("application-id", &application_id), ("flags", flags)])
            .expect("Failed to create ArchivistApplication")
    }

    fn setup_gactions(&self) {
        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.quit();
        }));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about();
        }));
        self.add_action(&about_action);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("Archivist")
            .application_icon("fr.mrben.Archivist")
            .developer_name("Benjamin Collet")
            .version(VERSION)
            .developers(vec!["Benjamin Collet https://github.com/mrBen".into()])
            .copyright("© 2022 Benjamin Collet")
            .license("This application comes with absolutely no warranty. See the <a href=\"https://cecill.info/licences/Licence_CeCILL_V2.1-en.html\">CeCILL License, Version 2.1</a> for details.")
            .build();

        about.present();
    }
}
