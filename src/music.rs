use mpris::{Metadata, PlaybackStatus, PlayerFinder, Progress, ProgressTick};
use serde_json::json;
use std::time::Duration;

pub fn music() {
    let player = match PlayerFinder::new() {
        Ok(player) => match player.find_active() {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        },
        Err(e) => panic!("{}", e),
    };

    let identity = player.identity();
    println!("Found player {}", identity);
    let metadata = player.get_metadata().unwrap();
    let playback_status = player.get_playback_status().unwrap();
    let shuffle = player.checked_get_shuffle().unwrap().unwrap_or(false);
    let loop_status = player.checked_get_loop_status().unwrap();
    let rate = player.checked_get_playback_rate().unwrap().unwrap_or(1.0);
    let position = player
        .checked_get_position()
        .unwrap()
        .unwrap_or_else(|| Duration::new(0, 0));

    let current_volume = player.checked_get_volume().unwrap().unwrap_or(1.0);
    println!(
        "metadata: {:#?}\nstatus: {:#?}\nshuffle: {}\nloop: {:#?}\nrate: {}\nposition: {:#?}\nvolume: {}",
        metadata,
        playback_status,
        shuffle,
        loop_status.unwrap(),
        rate,
        position,
        current_volume
    );

    let mut progress_tracker = match player.track_progress(1000) {
        Ok(progress_tracker) => progress_tracker,
        Err(e) => panic!("{}", e),
    };

    loop {
        let ProgressTick {
            progress,
            progress_changed,
            ..
        } = progress_tracker.tick();

        if !progress_changed {
            continue;
        };

        let metadata = progress.metadata();
        let (duration, position): (String, String);
        let position_percent: u128;

        if let Some(length) = progress.length() {
            duration = get_time(length);
            position = get_time(progress.position());
            position_percent = progress.position().as_millis() * 100 / length.as_millis();
        } else {
            duration = "".to_string();
            position = "".to_string();
            position_percent = 0;
        };

        let data = json!({
            "status": get_playback_status(progress),
            "artist": get_artist(metadata),
            "title": get_title(metadata),
            "duration": duration,
            "position": position,
            "position_percent": position_percent,
        });

        println!("{}", data);
    }
}

fn get_time(duration: Duration) -> String {
    let secs = duration.as_secs();
    let whole_hours = secs / (60 * 60);

    let secs = secs - whole_hours * 60 * 60;
    let whole_minutes = secs / 60;

    let secs = secs - whole_minutes * 60;

    format!("{:02}:{:02}:{:02}", whole_hours, whole_minutes, secs)
}

fn get_artist(metadata: &Metadata) -> String {
    if let Some(artists) = metadata.artists() {
        if !artists.is_empty() {
            artists.join(", ")
        } else {
            "Unknown Artist".to_string()
        }
    } else {
        "Unknown Artist".to_string()
    }
}

fn get_title(metadata: &Metadata) -> String {
    metadata.title().unwrap_or("Unknown title").to_string()
}

fn get_playback_status(progress: &Progress) -> String {
    match progress.playback_status() {
        PlaybackStatus::Playing => "",
        PlaybackStatus::Paused => "",
        PlaybackStatus::Stopped => "",
    }
    .to_string()
}
