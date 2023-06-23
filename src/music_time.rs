use mpris::{PlaybackStatus, Player, PlayerFinder, ProgressTick, ProgressTracker};
use serde_json::json;
use std::time::Duration;

use crate::music::utils;

pub fn main() {
    loop {
        if let Ok(player) = PlayerFinder::new()
            .expect("Failed to create PlayerFinder")
            .find_active()
        {
            let progress_tracker = match player.track_progress(1000) {
                Ok(progress_tracker) => progress_tracker,
                Err(e) => panic!("{}", e),
            };

            let data = get_position_data(&player);
            println!("{}", data);

            monitor_player(progress_tracker);
        } else {
            println!();
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}

fn get_position_data(player: &Player) -> serde_json::Value {
    let position;
    let position_percent;
    if let Some(length) = player.get_metadata().unwrap().length() {
        position = utils::get_time(player.get_position().unwrap());
        position_percent =
            player.get_position().unwrap().as_millis() as f64 * 100.0 / length.as_millis() as f64;
    } else {
        position = "".to_string();
        position_percent = 0.0;
    };

    json!({
        "position": position,
        "position_percent": format!("{:.2}", position_percent),
    })
}

fn monitor_player(mut progress_tracker: ProgressTracker) {
    let mut old_data = json!({});

    loop {
        let ProgressTick { progress, .. } = progress_tracker.tick();

        let position: String;
        let position_percent: f64;

        if progress.playback_status() != PlaybackStatus::Playing {
            continue;
        };

        if let Some(length) = progress.length() {
            position = utils::get_time(progress.position());
            position_percent =
                progress.position().as_millis() as f64 * 100.0 / length.as_millis() as f64;
        } else {
            position = "".to_string();
            position_percent = 0.0;
        };

        let data = json!({
            "position": position,
            "position_percent": format!("{:.2}", position_percent),
        });

        if data != old_data {
            println!("{}", data);
            old_data = data;
        }
    }
}
