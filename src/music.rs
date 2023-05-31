use image::io::Reader as ImageReader;
use mpris::{Metadata, PlaybackStatus, PlayerFinder, Progress, ProgressTick};
use serde_json::json;
use std::time::Duration;
extern crate dirs;
extern crate reqwest;

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
        let ProgressTick {
            progress,
            progress_changed,
            ..
        } = progress_tracker.tick();

        if !progress_changed {
            continue;
        };

        let metadata = progress.metadata();
        let duration: String;

        if let Some(length) = progress.length() {
            duration = get_time(length);
        } else {
            duration = "".to_string();
        };

        let data = json!({
            "status": get_playback_status(progress),
            "artist": get_artist(metadata),
            "title": get_title(metadata),
            "duration": duration,
            "cover": get_cover(metadata),
            // "background": get_background(metadata),
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
        PlaybackStatus::Playing => "",
        PlaybackStatus::Paused => "",
        PlaybackStatus::Stopped => "",
    }
    .to_string()
}

/// Caches cover art URLs and returns the path
fn get_cover(metadata: &Metadata) -> String {
    let cache_dir = dirs::cache_dir().unwrap().into_os_string();
    let url = metadata.art_url().unwrap_or("No cover art found");

    if !url.starts_with("https://") {
        // early return, don't cache on-disk files
        return url.to_owned();
    }

    let (_, suffix) = url.rsplit_once('/').unwrap();
    let cover = std::path::Path::new(&cache_dir)
        .join("eww/covers")
        .join(suffix);

    if !cover.exists() {
        std::fs::create_dir_all(cover.parent().unwrap()).unwrap();
        let mut file = std::fs::File::create(&cover).unwrap();

        reqwest::blocking::get(url)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();
    }

    cover.into_os_string().into_string().unwrap()
}

// fn get_background(metadata: &Metadata) -> std::ffi::OsString {
//     let url = metadata.art_url().unwrap_or("No cover art found");
//     let cover = std::path::Path::new(url);

//     if cover.exists() {
//         cover.file_stem().unwrap().to_owned()
//     } else {
//         let img = ImageReader::open(url).unwrap().decode().unwrap().blur(25.0);
//     }
// }
