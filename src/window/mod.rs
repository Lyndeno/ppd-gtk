mod imp;

use adw::subclass::prelude::*;
use adw::Application;

use adw::prelude::*;
use futures::StreamExt;
use glib::{clone, Object};
use gtk::gio::ActionEntry;
use gtk::{gio, glib};
use ppd::PpdProxy;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
            gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_callbacks(&self) {
        let (sender, receiver) = async_channel::bounded(1);
        crate::runtime().spawn(clone!(
            #[strong]
            sender,
            async move {
                let conn = zbus::Connection::system().await.unwrap();
                let proxy = PpdProxy::new(&conn).await.unwrap();
                let initial_profile = proxy.active_profile().await.unwrap();
                sender
                    .send(initial_profile)
                    .await
                    .expect("Channel is not open");
                let mut signal = proxy.receive_active_profile_changed().await;
                while let Some(p) = signal.next().await {
                    let name = p.get().await.unwrap();
                    sender.send(name).await.expect("Channel is not open");
                }
            }
        ));

        glib::spawn_future_local(clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                while let Ok(response) = receiver.recv().await {
                    window.imp().current_usage.set_label(&response);
                }
            }
        ));
    }

    fn setup_actions(&self) {
        let action_about = ActionEntry::builder("show_about")
            .activate(move |window: &Self, _, _| {
                let dialog = adw::AboutDialog::builder()
                    .application_name("BBase")
                    .developer_name("Lyndon Sanche")
                    .website("https://github.com/lyndeno/bbase-client")
                    .issue_url("https://github.com/lyndeno/bbase-client/issues")
                    .designers(vec!["Lyndon Sanche <lsanche@lyndeno.ca>".to_string()])
                    .version(env!("CARGO_PKG_VERSION"))
                    .developers(vec!["Lyndon Sanche <lsanche@lyndeno.ca>".to_string()])
                    .build();

                dialog.present(Some(window));
            })
            .build();

        self.add_action_entries([action_about]);
    }
}
