use mpris::{Metadata, PlayerFinder};
use serde_json::json;
use std::time::Duration;

mod images;
pub mod utils;

#[derive(serde::Serialize, PartialEq, Default)]
struct PlayerInfo {
    status: String,
    artist: String,
    title: String,
    duration: String,
    cover: String,
    background: String,
    foreground: String,
}

pub fn main() {
    let mut old_data = PlayerInfo::default();

    loop {
        let player = PlayerFinder::new()
            .expect("Failed to create PlayerFinder")
            .find_active();

        if let Ok(player) = player {
            let events = match player.events() {
                Ok(e) => e,
                Err(e) => {
                    log::warn!("{e}");
                    continue;
                }
            };
            let mut data = get_metadata(&player);
            if old_data != data {
                println!("{}", json!(data));
                old_data = data;
            }
            for _ in events {
                if !player.is_running() {
                    break;
                }
                data = get_metadata(&player);
                if old_data != data {
                    println!("{}", json!(data));
                    old_data = data;
                }
            }
        } else {
            if old_data != PlayerInfo::default() {
                println!("{}", json!(PlayerInfo::default()));
                old_data = PlayerInfo::default();
            }
            // Wait for a while before searching for players again
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}

fn get_metadata(player: &mpris::Player) -> PlayerInfo {
    let metadata_result = player.get_metadata();

    metadata_result.map_or_else(
        |_| PlayerInfo::default(),
        |metadata| {
            let duration = metadata.length().map(utils::get_time).unwrap_or_default();
            let cover = images::get_cover(&metadata);
            let playback_status = format!(
                "{:?}",
                player
                    .get_playback_status()
                    .expect("Could not get playback status")
            );

            PlayerInfo {
                status: playback_status,
                artist: get_artist(&metadata),
                title: get_title(&metadata),
                duration,
                cover: cover.to_string_lossy().into_owned(),
                background: images::get_background(&cover)
                    .to_string_lossy()
                    .into_owned(),
                foreground: images::get_foreground(&cover),
            }
        },
    )
}

fn get_artist(metadata: &Metadata) -> String {
    metadata.artists().map_or_else(
        || "Unknown Artist".to_string(),
        |artists| {
            if artists.is_empty() {
                "Unknown Artist".to_string()
            } else {
                artists.join(", ")
            }
        },
    )
}

fn get_title(metadata: &Metadata) -> String {
    metadata.title().unwrap_or("Unknown title").to_string()
}
