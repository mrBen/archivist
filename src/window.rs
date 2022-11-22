/*
 * SPDX-FileCopyrightText: 2022 Benjamin Collet <benjamin.collet@protonmail.ch>
 *
 * SPDX-License-Identifier: CECILL-2.1
 */

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/fr/mrben/Archivist/window.ui")]
    pub struct ArchivistWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub preferences_group: TemplateChild<adw::PreferencesGroup>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ArchivistWindow {
        const NAME: &'static str = "ArchivistWindow";
        type Type = super::ArchivistWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ArchivistWindow {}
    impl WidgetImpl for ArchivistWindow {}
    impl WindowImpl for ArchivistWindow {}
    impl ApplicationWindowImpl for ArchivistWindow {}
    impl AdwApplicationWindowImpl for ArchivistWindow {}
}

glib::wrapper! {
    pub struct ArchivistWindow(ObjectSubclass<imp::ArchivistWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl ArchivistWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create ArchivistWindow")
    }
}
