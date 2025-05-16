mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib::{self, Binding};

use crate::profile_object::ProfileObject;

glib::wrapper! {
    pub struct ProfileRow(ObjectSubclass<imp::ProfileRow>)
    @extends adw::ActionRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, adw::PreferencesRow, gtk::ListBoxRow;
}

impl Default for ProfileRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_profile(profile: &ProfileObject) -> Self {
        let obj: ProfileRow = Object::builder().build();
        obj.bind(profile);
        obj
    }

    pub fn bind(&self, profile_object: &ProfileObject) {
        let check = self.imp().check.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        let name_label_binding = profile_object
            .bind_property("name", self, "title")
            .sync_create()
            .build();
        bindings.push(name_label_binding);

        let active_binding = profile_object
            .bind_property("active", &check, "active")
            .sync_create()
            .bidirectional()
            .build();
        bindings.push(active_binding);
    }

    pub fn push_binding(&mut self, binding: Binding) {
        let mut bindings = self.imp().bindings.borrow_mut();
        bindings.push(binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
