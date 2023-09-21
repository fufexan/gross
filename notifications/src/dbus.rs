use super::NotificationArc;
use chrono::Local;
use gtk::{prelude::IconThemeExt, IconLookupFlags, IconTheme};
use std::collections::HashMap;
use zbus::zvariant::Value;
use zbus::{dbus_interface, Connection};

use crate::{icon::Icon, Notification};

const CAPABILITIES: [&str; 6] = [
    "icons",
    "actions",
    "body",
    "body-images",
    "action-icons",
    "persistence",
];

const SERVER_INFORMATION: (&str, &str, &str, &str) =
    ("notify-receive", "neoney.dev", "0.0.1", "1.0");

const NAME: &str = "org.freedesktop.Notifications";
const PATH: &str = "/org/freedesktop/Notifications";

struct NotificationServer {
    notifications: NotificationArc,
    biggest_id: u32,
}

impl NotificationServer {
    fn new(notifications: NotificationArc) -> Self {
        NotificationServer {
            notifications,
            biggest_id: 0,
        }
    }
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl NotificationServer {
    async fn notify(
        &mut self,
        app_name: String,
        replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        hints: HashMap<String, Value<'_>>,
        expire_timeout: i32,
    ) -> u32 {
        gtk::init().expect("Failed to initialize GTK");

        let dt = Local::now();
        let time = dt.time().format("%H:%M").to_string();

        let changed_id = if replaces_id != 0 {
            replaces_id
        } else {
            self.biggest_id += 1;
            self.biggest_id
        };

        let icon = match app_icon.as_str() {
            "" => "".to_string(),
            app_icon => {
                let icon_theme = IconTheme::default().expect("Failed to find icon theme.");
                let icon = icon_theme.lookup_icon(app_icon, 256, IconLookupFlags::empty());

                icon.map_or(app_icon.to_string(), |icon| {
                    icon.filename().unwrap().to_string_lossy().to_string()
                })
            }
        };

        let action_icons = hints.get("action-icons");
        let desktop_entry: String = hints
            .get("desktop-entry")
            .and_then(|v| v.clone().downcast())
            .unwrap_or(app_name.clone());
        // let image_data_d = hints.get("image_data");
        // let image_data = hints.get("image-data");
        // let image_path_d = hints.get("image_path");
        // let image_path = hints.get("image-path");

        let icon_data_hint = hints.get("icon_data");
        let icon_data: Option<Icon> = icon_data_hint.and_then(|v| v.clone().downcast());

        let mut image = String::new();
        if let Some(icon) = icon_data {
            if let Ok(imgpath) = icon.write_image(&app_name, changed_id) {
                image = imgpath;
            }
        };
        println!("action_icons: {:?}", action_icons);
        println!("desktop_entry: {:?}", desktop_entry);
        // println!("icon_data: {:?}", icon_data);

        // let image: String = if image_data.is_some() {};

        let payload = Notification {
            id: changed_id,
            app_name,
            app_icon: icon,
            summary,
            time,
            body,
            actions,
            urgency: hints
                .get("urgency")
                .and_then(|v| v.clone().downcast())
                .unwrap_or_default(),
            image,
            timeout: expire_timeout,
            visible: true,
        };

        let mut notifs = self.notifications.lock().await;

        notifs.insert(changed_id, payload);

        println!(
            "{}",
            serde_json::to_string(&notifs.values().collect::<Vec<_>>()).unwrap()
        );

        changed_id
    }

    async fn close(&mut self, id: u32) {
        let mut notifs = self.notifications.lock().await;

        if let Some(notif) = notifs.get_mut(&id) {
            notif.visible = false;
        };

        println!(
            "{}",
            serde_json::to_string(&notifs.values().collect::<Vec<_>>()).unwrap()
        );
    }

    fn get_capabilities(&self) -> &[&str] {
        &CAPABILITIES
    }
    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        SERVER_INFORMATION
    }
}

pub async fn async_run(notifications: NotificationArc) -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let server = NotificationServer::new(notifications);

    connection.object_server().at(PATH, server).await?;

    connection.request_name(NAME).await?;

    loop {
        std::future::pending::<()>().await;
    }
}
