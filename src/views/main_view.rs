/* initial_setup.rs
 *
 * SPDX-FileCopyrightText: Copyright 2023 rdbende
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/rdbende/Mona/ui/views/main.ui")]
    pub struct MainView {
        #[template_child]
        pub split_view: TemplateChild<adw::OverlaySplitView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainView {
        const NAME: &'static str = "MainView";
        type Type = super::MainView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainView {}
    impl WidgetImpl for MainView {}
    impl BinImpl for MainView {}
}

glib::wrapper! {
    pub struct MainView(ObjectSubclass<imp::MainView>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl MainView {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
