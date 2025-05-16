use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::Binding;
use gtk::{glib, CheckButton, CompositeTemplate, Label};

// state
#[derive(Default, CompositeTemplate)]
#[template(file = "res/profile_row.blp")]
pub struct ProfileRow {
    #[template_child]
    pub check: TemplateChild<CheckButton>,
    pub bindings: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ProfileRow {
    const NAME: &'static str = "ProfileActionRow";
    type Type = super::ProfileRow;
    type ParentType = adw::ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ProfileRow {}
impl WidgetImpl for ProfileRow {}
impl ActionRowImpl for ProfileRow {}
impl PreferencesRowImpl for ProfileRow {}
impl ListBoxRowImpl for ProfileRow {}
