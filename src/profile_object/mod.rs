mod imp;

use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib::{self};
use ppd::Profile;

glib::wrapper! {
    pub struct ProfileObject(ObjectSubclass<imp::ProfileObject>);
}

impl ProfileObject {
    pub fn new(repo: Profile) -> Self {
        let r = ProfileData::from(repo);
        let obj = Object::builder();
        let built = obj.property("name", r.name).build();

        built
    }

    pub fn data(&self) -> ProfileData {
        self.imp().data.borrow().clone()
    }
}

#[derive(Default, Clone)]
pub struct ProfileData {
    pub name: String,
    pub driver: String,
    pub platform_driver: Option<String>,
    pub cpu_driver: Option<String>,
    pub active: bool,
}

impl From<Profile> for ProfileData {
    fn from(v: Profile) -> Self {
        Self {
            name: v.profile,
            driver: v.driver,
            platform_driver: v.platform_driver,
            cpu_driver: v.cpu_driver,
            active: false,
        }
    }
}
