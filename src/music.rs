use mpris::{Metadata, PlaybackStatus, PlayerFinder};
use serde_json::json;
use std::time::Duration;

mod images;
pub mod utils;

pub fn main() {
    loop {
        let player = PlayerFinder::new()
            .expect("Failed to create PlayerFinder")
            .find_active();

        match player {
            Ok(player) => {
                monitor_player(player);
            }
            Err(err) => {
                println!();
                eprintln!("Failed to find active player: {}", err);
                // Wait for a while before searching for players again
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

fn monitor_player(player: mpris::Player) {
    let events = player.events().unwrap();

    print_metadata(&player);

    for _ in events {
        if !player.is_running() {
            break;
        }
        print_metadata(&player);
    }
}

fn print_metadata(player: &mpris::Player) {
    let metadata_result = player.get_metadata();

    let data = metadata_result
        .map(|metadata| {
            let duration = metadata.length().map(utils::get_time).unwrap_or_default();
            let cover = images::get_cover(&metadata);

            json!({
                "status": get_playback_status(&player.get_playback_status().unwrap()),
                "artist": get_artist(&metadata),
                "title": get_title(&metadata),
                "duration": duration,
                "cover": cover.to_string_lossy(),
                "background": images::get_background(&cover).to_string_lossy(),
                "foreground": images::get_foreground(&cover),
            })
        })
        .unwrap_or_else(|_| {
            json!({
                "status": "",
                "artist": "",
                "title": "",
                "duration": "",
                "cover": "",
                "background": "",
                "foreground": "light",
            })
        });

    println!("{}", data);
}

fn get_artist(metadata: &Metadata) -> String {
    metadata
        .artists()
        .map(|artists| {
            if artists.is_empty() {
                "Unknown Artist".to_string()
            } else {
                artists.join(", ")
            }
        })
        .unwrap_or_else(|| "Unknown Artist".to_string())
}

fn get_title(metadata: &Metadata) -> String {
    metadata.title().unwrap_or("Unknown title").to_string()
}

fn get_playback_status(playback_status: &PlaybackStatus) -> String {
    match playback_status {
        PlaybackStatus::Playing => "",
        PlaybackStatus::Paused => "",
        PlaybackStatus::Stopped => "",
    }
    .to_string()
}
