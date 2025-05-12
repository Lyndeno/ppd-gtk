mod window;

use std::sync::OnceLock;
use window::Window;

use adw::prelude::*;
use adw::Application;

use gtk::{gio, glib};

const APP_ID: &str = "org.lyndeno.bbase";

use tokio::runtime::Runtime;

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Tokio runtime needs to work"))
}

fn main() -> glib::ExitCode {
    println!("Hello, world!");

    /*
     */

    // Start graphical app
    gio::resources_register_include!("mainwindow.gresource").expect("Failed to get resource");

    // create app
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);

    window.present();
}
