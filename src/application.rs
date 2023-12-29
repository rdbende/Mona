/* application.rs
 *
 * SPDX-FileCopyrightText: Copyright 2023 rdbende
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::MonaWindow;

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct MonaApplication {
      pub window: glib::WeakRef<MonaWindow>,
    }

    impl Default for MonaApplication {
        fn default() -> Self {
            Self {
                window: Default::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MonaApplication {
        const NAME: &'static str = "MonaApplication";
        type Type = super::MonaApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for MonaApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("app.about", &["F1"]);
        }
    }

    impl ApplicationImpl for MonaApplication {
        fn activate(&self) {
            let app = self.obj();

            if let Some(window) = self.window.upgrade() {
                window.present();
                return;
            }

            let window = MonaWindow::new(&*app);
            self.window.set(Some(&window));

            app.main_window().present();
        }
    }

    impl GtkApplicationImpl for MonaApplication {}
    impl AdwApplicationImpl for MonaApplication {}
}

glib::wrapper! {
    pub struct MonaApplication(ObjectSubclass<imp::MonaApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl MonaApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    pub fn main_window(&self) -> MonaWindow {
        self.imp().window.upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        let show_main_action = gio::ActionEntry::builder("show-main")
            .activate(|app: &Self, _, _| app.main_window().switch_to_main_page())
            .build();

        self.add_action_entries([quit_action, about_action, show_main_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("Mona")
            .application_icon("io.github.rdbende.Mona")
            .developer_name("rdbende")
            .version(VERSION)
            .developers(vec!["rdbende"])
            .build();

        about.present();
    }
}
