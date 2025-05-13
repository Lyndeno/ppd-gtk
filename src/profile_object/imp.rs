use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use gtk::glib;

use super::ProfileData;

//State
#[derive(Properties, Default)]
#[properties(wrapper_type = super::ProfileObject)]
pub struct ProfileObject {
    #[property(name = "name", get, set, type = String, member = name)]
    pub data: RefCell<ProfileData>,
}

#[glib::object_subclass]
impl ObjectSubclass for ProfileObject {
    const NAME: &'static str = "ProfileListObject";
    type Type = super::ProfileObject;
}

#[glib::derived_properties]
impl ObjectImpl for ProfileObject {}
