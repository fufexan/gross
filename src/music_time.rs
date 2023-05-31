use mpris::{PlaybackStatus, PlayerFinder, ProgressTick};
use serde_json::json;
use std::time::Duration;

pub fn main() {
    let player = match PlayerFinder::new() {
        Ok(player) => match player.find_active() {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        },
        Err(e) => panic!("{}", e),
    };

    let mut progress_tracker = match player.track_progress(1000) {
        Ok(progress_tracker) => progress_tracker,
        Err(e) => panic!("{}", e),
    };

    loop {
        let ProgressTick { progress, .. } = progress_tracker.tick();

        let position: String;
        let position_percent: u128;

        if progress.playback_status() != PlaybackStatus::Playing {
            continue;
        };

        if let Some(length) = progress.length() {
            position = get_time(progress.position());
            position_percent = progress.position().as_millis() * 100 / length.as_millis();
        } else {
            position = "".to_string();
            position_percent = 0;
        };

        let data = json!({
            "position": position,
            "position_percent": position_percent,
        });

        println!("{}", data);
    }
}

fn get_time(duration: Duration) -> String {
    let mut time = String::new();

    let secs = duration.as_secs();
    let whole_hours = secs / (60 * 60);

    if whole_hours > 0 {
        time.push_str(format!("{:02}:", whole_hours).as_str())
    }

    let secs = secs - whole_hours * 60 * 60;
    let whole_minutes = secs / 60;

    let secs = secs - whole_minutes * 60;
    time.push_str(format!("{:02}:{:02}", whole_minutes, secs).as_str());

    time
}
