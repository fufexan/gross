mod dbus;
mod icon;
mod types;

use std::fs;
use std::time::Duration;
use std::{collections::HashMap, hash::BuildHasherDefault, path::Path, sync::Arc};

use std::io::Read;
use tokio::sync::Mutex;

use anyhow::Result;
use nohash_hasher::NoHashHasher;
use types::Notification;

use crate::dbus::async_run;

type NotificationArc =
    Arc<Mutex<HashMap<u32, Notification, BuildHasherDefault<NoHashHasher<u32>>>>>;

#[tokio::main]
pub async fn main() -> Result<()> {
    let notifications: NotificationArc = Arc::new(Mutex::new(HashMap::with_hasher(
        BuildHasherDefault::default(),
    )));

    fs::remove_file(Path::new("/var/run/user/1000/notify-receive.pipe")).ok();
    unix_named_pipe::create("/var/run/user/1000/notify-receive.pipe", Some(0o644))?;

    let mut reader = unix_named_pipe::open_read("/var/run/user/1000/notify-receive.pipe")?;

    let pipe_notifications = notifications.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(100));

        loop {
            let mut contents = String::new();

            if reader.read_to_string(&mut contents).is_ok() && !contents.is_empty() {
                let mut notifs = pipe_notifications.lock().await;

                for line in contents.lines() {
                    if line.is_empty() {
                        continue;
                    }

                    if let Ok(id) = line.parse::<u32>() {
                        if let Some(notif) = notifs.get_mut(&id) {
                            notif.visible = false;
                        }
                    }
                }

                println!(
                    "{}",
                    serde_json::to_string(&notifs.values().collect::<Vec<_>>()).unwrap()
                );
            }

            interval.tick().await;
        }
    });

    async_run(Arc::clone(&notifications)).await?;

    Ok(())
}
