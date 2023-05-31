use image::{ImageFormat, ImageOutputFormat};
use mpris::{Metadata, PlaybackStatus, PlayerFinder, Progress, ProgressTick};
use serde_json::json;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    time::Duration,
};

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

        let cover = get_cover(metadata);

        let data = json!({
            "status": get_playback_status(progress),
            "artist": get_artist(metadata),
            "title": get_title(metadata),
            "duration": duration,
            "cover": cover,
            "background": get_background(&cover),
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
fn get_cover(metadata: &Metadata) -> PathBuf {
    let cache_dir = dirs::cache_dir().unwrap().into_os_string();
    let url = metadata.art_url().unwrap_or("No cover art found");

    if url.starts_with("file://") {
        // early return, don't cache on-disk files
        let normalized_url = url.replace("file://", "");
        let cover = Path::new(&normalized_url);
        return cover.to_path_buf();
    }

    let (_, suffix) = url.rsplit_once('/').unwrap();
    let cover = Path::new(&cache_dir).join("eww/covers").join(suffix);

    if !cover.exists() {
        std::fs::create_dir_all(cover.parent().unwrap()).unwrap();
        let mut file = File::create(&cover).unwrap();

        reqwest::blocking::get(url)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();
    }

    cover
}

fn get_background(cover: &PathBuf) -> PathBuf {
    let cache_dir = dirs::cache_dir().unwrap().into_os_string();
    let bg = Path::new(&cache_dir)
        .join("eww/backgrounds")
        .join(cover.file_stem().unwrap());
    std::fs::create_dir_all(bg.parent().unwrap()).unwrap();

    if bg.exists() {
        return bg;
    }

    // blur cover
    let image = image::load(
        BufReader::new(File::open(cover).unwrap()),
        ImageFormat::Jpeg,
    )
    .unwrap()
    .blur(25.0);

    let mut bg_file = File::create(&bg).unwrap();
    image
        .write_to(&mut bg_file, ImageOutputFormat::Jpeg(80))
        .expect("Background image could not be written");

    bg
}
