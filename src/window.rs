/* window.rs
 *
 * SPDX-FileCopyrightText: Copyright 2023 rdbende
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::views::InitialSetup;
use crate::views::MainView;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/rdbende/Mona/ui/window.ui")]
    pub struct MonaWindow {
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub login: TemplateChild<InitialSetup>,
        #[template_child]
        pub main: TemplateChild<MainView>,
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

    pub fn switch_to_main_page(&self) {
        let imp = self.imp();
        imp.stack.set_visible_child(&*imp.main);
    }
}
