/* window.rs
 *
 * SPDX-FileCopyrightText: Copyright 2023 rdbende
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::views::InitialSetup;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/rdbende/Mona/window.ui")]
    pub struct MonaWindow {
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MonaWindow {
        const NAME: &'static str = "MonaWindow";
        type Type = super::MonaWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MonaWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.stack.add_child(&InitialSetup::new());
        }
    }

    impl WidgetImpl for MonaWindow {}
    impl WindowImpl for MonaWindow {}
    impl ApplicationWindowImpl for MonaWindow {}
    impl AdwApplicationWindowImpl for MonaWindow {}
}

glib::wrapper! {
    pub struct MonaWindow(ObjectSubclass<imp::MonaWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl MonaWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
