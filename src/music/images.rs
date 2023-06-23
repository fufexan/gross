use image::{io::Reader, DynamicImage};
use mpris::Metadata;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::music::utils;

/// Caches cover art URLs and returns the path
pub fn get_cover(metadata: &Metadata) -> PathBuf {
    if let Some(url) = metadata.art_url() {
        if url.starts_with("file://") {
            let normalized_url = url.strip_prefix("file://").unwrap();
            return PathBuf::from(normalized_url);
        }

        let suffix = url.rsplit_once('/').map(|(_, suffix)| suffix);
        if let Some(suffix) = suffix {
            let cover_file = Path::new(suffix).to_path_buf();
            let cover = utils::cache_entry(&cover_file, "eww/covers");
            if !cover.exists() {
                let mut file = File::create(&cover).expect("Cover file could not be created");

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

fn get_image(cover: &PathBuf) -> Result<DynamicImage, image::ImageError> {
    Reader::open(cover)?.with_guessed_format()?.decode()
}

pub fn get_foreground(cover: &PathBuf) -> String {
    // check whether the cover exists or return nothing
    if cover.clone().into_os_string().is_empty() {
        return String::new();
    }

    // get cache entry
    let fg_file = utils::cache_entry(cover, "eww/foregrounds");
    // if the cache file could be read and matches known values, print that
    let mut fg = if let Ok(value) = fs::read_to_string(&fg_file) {
        if value == *"light" || value == *"dark" {
            value
        } else {
            String::from("light")
        }
    // otherwise, assume light foreground
    } else {
        String::from("light")
    };

    // generate grayscale pixel and check its luminance. over 100 we use dark foreground
    if let Ok(image) = get_image(cover) {
        let luma = image.thumbnail(1, 1).to_luma8();
        fg = if luma.into_raw()[0] > 100 {
            "dark".to_owned()
        } else {
            "light".to_owned()
        };
        // write file with foreground value
        fs::write(fg_file, &fg).expect("Foreground cache file could not be written");
    }

    fg
}

pub fn get_background(cover: &PathBuf) -> PathBuf {
    if cover.clone().into_os_string().is_empty() {
        return PathBuf::new();
    }

    let bg = utils::cache_entry(cover, "eww/backgrounds");

    if bg.exists() {
        return bg;
    }

    if let Ok(image) = get_image(cover) {
        // background blurred image
        let width = image.width() as usize;
        let height = image.height() as usize;
        let data = image.into_bytes();

        // blur
        if data.len() % 3 == 0 {
            let mut data_new = utils::unflatten(&data);
            fastblur::gaussian_blur(&mut data_new, width, height, 25.0);

            let mut buf = Vec::new();
            let header = format!("P6\n{}\n{}\n{}\n", width, height, 255);
            buf.write_all(header.as_bytes()).unwrap();

            for px in data_new {
                buf.write_all(&px).unwrap();
            }

            // write blurred file
            fs::write(&bg, &buf).expect("Background image could not be written");
        }
    }

    bg
}
