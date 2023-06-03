use image::{io::Reader, ImageOutputFormat};
use mpris::{Metadata, PlaybackStatus, PlayerFinder};
use serde_json::json;
use std::{
    fs::File,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

extern crate dirs;
extern crate reqwest;

pub fn main() {
    loop {
        let player = PlayerFinder::new()
            .expect("{{}}")
            .find_active()
            .expect("{{}}");

        let events = player.events().unwrap();

        for _ in events {
            if !player.is_running() {
                break;
            }

            let metadata = player.get_metadata().unwrap();
            let duration: String;

            if let Some(length) = metadata.length() {
                duration = get_time(length);
            } else {
                duration = "".to_string();
            };

            let cover = get_cover(&metadata);

            let data = json!({
                "status": get_playback_status(&player.get_playback_status().unwrap()),
                "artist": get_artist(&metadata),
                "title": get_title(&metadata),
                "duration": duration,
                "cover": cover,
                "background": get_background(&cover),
            });

            println!("{}", data);
        }
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

fn get_playback_status(playback_status: &PlaybackStatus) -> String {
    match playback_status {
        PlaybackStatus::Playing => "",
        PlaybackStatus::Paused => "",
        PlaybackStatus::Stopped => "",
    }
    .to_string()
}

/// Caches cover art URLs and returns the path
fn get_cover(metadata: &Metadata) -> PathBuf {
    let cache_dir = dirs::cache_dir().unwrap().into_os_string();

    let url: String = match metadata.art_url() {
        Some(url) => url.to_owned(),
        None => {
            return PathBuf::new();
        }
    };

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

    if cover.clone().into_os_string().is_empty() {
        return PathBuf::new();
    }

    let bg = Path::new(&cache_dir)
        .join("eww/backgrounds")
        .join(cover.file_stem().unwrap());
    std::fs::create_dir_all(bg.parent().unwrap()).unwrap();

    if bg.exists() {
        return bg;
    }

    // blur cover
    println!("generating background...");
    let bg_start = Instant::now();

    let image = Reader::open(cover)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .blur(25.0);

    let mut bg_file = File::create(&bg).unwrap();
    image
        .write_to(&mut bg_file, ImageOutputFormat::Jpeg(80))
        .expect("Background image could not be written");

    println!(
        "background generated: {}.{} s",
        bg_start.elapsed().as_secs(),
        bg_start.elapsed().as_millis()
    );
    bg
}
