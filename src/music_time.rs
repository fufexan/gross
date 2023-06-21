use mpris::{PlaybackStatus, Player, PlayerFinder, ProgressTick, ProgressTracker};
use serde_json::json;
use std::time::Duration;

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
        position = get_time(player.get_position().unwrap());
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
            position = get_time(progress.position());
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

fn get_time(duration: Duration) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = 60 * MINUTE;
    let mut time = String::new();

    let secs = duration.as_secs();
    let whole_hours = secs / HOUR;

    if whole_hours > 0 {
        time.push_str(format!("{:02}:", whole_hours).as_str())
    }

    let secs = secs - whole_hours * HOUR;
    let whole_minutes = secs / MINUTE;

    let secs = secs - whole_minutes * MINUTE;
    time.push_str(format!("{:02}:{:02}", whole_minutes, secs).as_str());

    time
}
