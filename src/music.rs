use image::io::Reader;
use mpris::{Metadata, PlaybackStatus, PlayerFinder};
use serde_json::json;
use std::{fs::File, io::Write, path::PathBuf, time::Duration};

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
            let duration = metadata.length().map(get_time).unwrap_or_default();
            let cover = get_cover(&metadata);
            let (background, foreground) = get_background(&cover);

            json!({
                "status": get_playback_status(&player.get_playback_status().unwrap()),
                "artist": get_artist(&metadata),
                "title": get_title(&metadata),
                "duration": duration,
                "cover": cover.to_string_lossy(),
                "background": background.to_string_lossy(),
                "foreground": foreground,
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

fn get_time(duration: Duration) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = 60 * MINUTE;
    let mut time = String::new();

    let secs = duration.as_secs();
    let whole_hours = secs / HOUR;

    if whole_hours > 0 {
        time.push_str(&format!("{:02}:", whole_hours));
    }

    let secs = secs - whole_hours * HOUR;
    let whole_minutes = secs / MINUTE;
    let secs = secs - whole_minutes * MINUTE;

    time.push_str(&format!("{:02}:{:02}", whole_minutes, secs));

    time
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

/// Caches cover art URLs and returns the path
fn get_cover(metadata: &Metadata) -> PathBuf {
    let cache_dir = dirs::cache_dir().unwrap();

    if let Some(url) = metadata.art_url() {
        if url.starts_with("file://") {
            let normalized_url = url.strip_prefix("file://").unwrap();
            return PathBuf::from(normalized_url);
        }

        let suffix = url.rsplit_once('/').map(|(_, suffix)| suffix);
        if let Some(suffix) = suffix {
            let cover = cache_dir.join("eww/covers").join(suffix);
            if !cover.exists() {
                std::fs::create_dir_all(cover.parent().unwrap())
                    .expect("Covers cache directory could not be created");

                let mut file = File::create(&cover).unwrap();

                if let Ok(mut response) = reqwest::blocking::get(url) {
                    if let Err(err) = std::io::copy(&mut response, &mut file) {
                        eprintln!("Failed to download cover art: {}", err);
                    }
                } else {
                    eprintln!("Failed to download cover art: Request failed");
                }
            }
            return cover;
        }
    }

    PathBuf::new()
}

fn get_background(cover: &PathBuf) -> (PathBuf, String) {
    let cache_dir = dirs::cache_dir().unwrap();

    if cover.clone().into_os_string().is_empty() {
        return (PathBuf::new(), String::new());
    }

    let bg = cache_dir
        .join("eww/backgrounds")
        .join(cover.file_stem().unwrap());
    std::fs::create_dir_all(bg.parent().unwrap()).expect("Background dir could not be created");
    let fg_file = cache_dir
        .join("eww/foregrounds")
        .join(cover.file_stem().unwrap());
    std::fs::create_dir_all(fg_file.parent().unwrap())
        .expect("Foreground dir could not be created");

    let mut fg = if let Ok(value) = std::fs::read_to_string(&fg_file) {
        value
    } else {
        String::from("light")
    };

    if bg.exists() && fg_file.exists() {
        return (bg, fg);
    }

    if let Ok(image) = Reader::open(cover)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
    {
        // foreground
        let luma = image.thumbnail(1, 1).to_luma8();
        fg = if luma.into_raw()[0] > 100 {
            "dark".to_owned()
        } else {
            "light".to_owned()
        };
        // write file with foreground value
        std::fs::write(fg_file, &fg).expect("Foreground cache file could not be written");

        // background blurred image
        let width = image.width() as usize;
        let height = image.height() as usize;
        let data = image.into_bytes();

        // blur
        if data.len() % 3 == 0 {
            let mut data_new = unflatten(&data);
            fastblur::gaussian_blur(&mut data_new, width, height, 25.0);

            let mut buf = Vec::new();
            let header = format!("P6\n{}\n{}\n{}\n", width, height, 255);
            buf.write_all(header.as_bytes()).unwrap();

            for px in data_new {
                buf.write_all(&px).unwrap();
            }

            // write blurred file
            std::fs::write(&bg, &buf).expect("Background image could not be written");
        }
    }

    (bg, fg)
}

fn unflatten(data: &[u8]) -> Vec<[u8; 3]> {
    data.chunks(3).map(|rgb| [rgb[0], rgb[1], rgb[2]]).collect()
}
