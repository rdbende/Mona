/* initial_setup.rs
 *
 * SPDX-FileCopyrightText: Copyright 2023 rdbende
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::subclass::prelude::*;
use gtk::{glib, gio};
use crate::glib::clone;
use gtk::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/rdbende/Mona/ui/initial_setup.ui")]
    pub struct InitialSetup {
        #[template_child]
        pub nav_view: TemplateChild<adw::NavigationView>,
        #[template_child]
        pub start_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub login_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub finish_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub token_entry: TemplateChild<adw::EntryRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InitialSetup {
        const NAME: &'static str = "InitialSetup";
        type Type = super::InitialSetup;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for InitialSetup {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            self.start_button.connect_clicked(clone!(@weak obj => move |_| {
                obj.login_clicked();
            }));

            self.login_button.connect_clicked(clone!(@weak obj => move |_| {
                obj.next_clicked();
            }));

            self.finish_button.connect_clicked(clone!(@weak obj => move |_| {
                obj.finish_clicked();
            }));
        }
    }

    impl WidgetImpl for InitialSetup {}
    impl BinImpl for InitialSetup {}
}

glib::wrapper! {
    pub struct InitialSetup(ObjectSubclass<imp::InitialSetup>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl InitialSetup {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    fn login_clicked(&self) {
        let imp = self.imp();
        imp.nav_view.push_by_tag("login");
        imp.token_entry.connect_changed(clone!(@weak imp => move |entry| {
            imp.login_button.set_sensitive(!entry.text().is_empty())
        }));
    }

    fn next_clicked(&self) {
        println!("{}", self.imp().token_entry.text());
        // TODO: Fractal has a cool SpinnerButton widget
        self.imp().login_button.set_sensitive(false);
        self.imp().login_button.set_child(Some(&gtk::Spinner::builder().spinning(true).build()));
        // Authorize
        self.imp().nav_view.push_by_tag("finished");
    }

    fn finish_clicked(&self) {
        let imp = self.imp();

    }
}
