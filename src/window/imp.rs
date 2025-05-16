use std::cell::RefCell;

use adw::subclass::prelude::*;

use adw::NavigationView;
use adw::Spinner;
use adw::ToastOverlay;
use glib::subclass::InitializingObject;
use gtk::{gio, glib, Button, CompositeTemplate};
use gtk::{Label, ListBox};

// Object for state
#[derive(CompositeTemplate, Default)]
#[template(file = "res/window.blp")]
pub struct Window {
    #[template_child]
    pub current_usage: TemplateChild<Label>,

    #[template_child]
    pub profile_list: TemplateChild<ListBox>,

    #[template_child]
    pub profile_view: TemplateChild<NavigationView>,
    pub profiles: RefCell<Option<gio::ListStore>>,
}

// Trait for subclassing
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // Name needs to match class
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all gobjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        //self.button.connect_clicked(move |button| {
        //    button.set_label("Yo");
        //});

        let obj = self.obj();
        obj.setup_profiles();
        obj.setup_callbacks();
        obj.setup_actions();
    }
}

// Trait shared by al widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all app windows
impl ApplicationWindowImpl for Window {}

// Trait shared by all app windows
impl AdwApplicationWindowImpl for Window {}
