mod imp;

use adw::subclass::prelude::*;
use adw::ActionRow;
use adw::Application;

use adw::prelude::*;
use futures::StreamExt;
use glib::{clone, Object};
use gtk::gio::ActionEntry;
use gtk::glib::property::PropertySet;
use gtk::Actionable;
use gtk::CheckButton;
use gtk::NoSelection;
use gtk::{gio, glib};
use ppd::PpdProxy;

use crate::profile_object::ProfileObject;
use crate::profile_row::ProfileRow;

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

    fn profiles(&self) -> gio::ListStore {
        self.imp()
            .profiles
            .borrow()
            .clone()
            .expect("Error getting profiles")
    }

    fn setup_profiles(&self) {
        let model = gio::ListStore::new::<ProfileObject>();

        self.imp().profiles.replace(Some(model));
        let dummy_row = ProfileRow::new();
        let dummy_check = dummy_row.imp().check.get();

        let selection_model = NoSelection::new(Some(self.profiles()));
        self.imp().profile_list.bind_model(
            Some(&selection_model),
            clone!(
                #[weak(rename_to = window)]
                self,
                #[upgrade_or_panic]
                move |obj| {
                    let repo_object = obj.downcast_ref().expect("Obj should be RepoObject");
                    let row = window.create_profile_row(repo_object, &dummy_check);
                    row.upcast()
                }
            ),
        );

        self.set_profile_list_visible(&self.profiles());
        self.profiles().connect_items_changed(clone!(
            #[weak(rename_to = window)]
            self,
            move |profiles, _, _, _| {
                window.set_profile_list_visible(profiles);
            }
        ));
    }

    fn set_profile_list_visible(&self, profiles: &gio::ListStore) {
        self.imp().profile_list.set_visible(profiles.n_items() > 0);
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

        glib::spawn_future_local(clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                let fut = crate::runtime().spawn(async {
                    let conn = zbus::Connection::system().await.unwrap();
                    let proxy = PpdProxy::new(&conn).await.unwrap();
                    proxy.profiles().await.unwrap()
                });
                let profiles = fut.await.unwrap();
                window.profiles().remove_all();
                for profile in profiles {
                    let item = ProfileObject::new(profile);
                    window.profiles().append(&item);
                }
            }
        ));
    }

    fn create_profile_row(
        &self,
        profile_object: &ProfileObject,
        group: &CheckButton,
    ) -> ProfileRow {
        let mut row = ProfileRow::with_profile(profile_object);
        let check = row.imp().check.get();
        check.set_group(Some(group));
        let name = profile_object.name();
        let name2 = name.clone();
        let name3 = name.clone();
        row.set_activatable(true);
        let active_label = self.imp().current_usage.get();
        let active_binding = active_label
            .bind_property("label", &check, "active")
            .transform_to(move |_, v: String| Some(name2 == v))
            .sync_create()
            .build();

        row.push_binding(active_binding);

        row.connect_activated(clone!(move |_| {
            crate::runtime().spawn(clone!(
                #[strong]
                name,
                async move {
                    let conn = zbus::Connection::system().await.unwrap();
                    let proxy = PpdProxy::new(&conn).await.unwrap();
                    proxy.set_active_profile(name).await.unwrap();
                }
            ));
        }));

        check.connect_toggled(clone!(move |cb| {
            if cb.is_active() {
                crate::runtime().spawn(clone!(
                    #[strong]
                    name3,
                    async move {
                        let conn = zbus::Connection::system().await.unwrap();
                        let proxy = PpdProxy::new(&conn).await.unwrap();
                        proxy.set_active_profile(name3).await.unwrap();
                    }
                ));
            }
        }));

        row
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
