use image::{io::Reader, DynamicImage, GenericImageView};
use material_color_utilities_rs::{
    palettes, quantize::quantizer_celebi::QuantizerCelebi, scheme::Scheme, score::score,
};
use mpris::Metadata;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::music::utils;

/// 24-bit pixel
#[derive(Debug, serde::Serialize, Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// Caches cover art URLs and returns the path
pub fn get_cover(metadata: &Metadata) -> PathBuf {
    if let Some(url) = metadata.art_url() {
        if url.starts_with("file://") {
            let mut normalized_url = url
                .strip_prefix("file://")
                .expect("Could not strip prefix of image url")
                .to_owned();
            if normalized_url.contains('%') {
                normalized_url = urlencoding::decode(&normalized_url)
                    .expect("Could not decode url")
                    .to_string();
            }
            println!("{normalized_url}");
            return PathBuf::from(normalized_url);
        }

        let suffix = url.rsplit_once('/').map(|(_, suffix)| suffix);
        if let Some(suffix) = suffix {
            let cover_file = Path::new(suffix).to_path_buf();
            let cover = utils::cache_entry(&cover_file, "eww/covers");
            if !cover.exists() {
                let mut file = File::create(&cover).expect("Cover file could not be created");

                reqwest::blocking::get(url).map_or_else(
                    |_| {
                        eprintln!("Failed to download cover art: Request failed");
                    },
                    |mut response| {
                        if let Err(err) = std::io::copy(&mut response, &mut file) {
                            eprintln!("Failed to download cover art: {err}");
                        }
                    },
                );
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
    if fg_file.exists() {
        let value = std::fs::read_to_string(fg_file).expect("Could not read foreground file");
        log::debug!("Foreground value read from cache: {value}");
        return value;
    }

    let color;
    if let Ok(image) = get_image(cover) {
        let (width, height) = image.dimensions();
        let resized_image = image.resize(
            width / 64,
            height / 64,
            image::imageops::FilterType::Lanczos3,
        );

        // put image data in a vec
        // `image` provides pixels as Rgba, but material-colors expects ARGB
        let mut pixels = vec![];
        for (_, _, p) in resized_image.pixels() {
            let c = Color {
                r: p[0],
                g: p[1],
                b: p[2],
                a: p[3],
            };
            pixels.push([c.a, c.r, c.g, c.b]);
        }

        // generate theme from image
        let theme = QuantizerCelebi::quantize(&mut QuantizerCelebi, &pixels, 1);
        let score = score(&theme);
        print_color(
            "score",
            score
                .first()
                .expect("Could not get score from image")
                .to_owned(),
        );

        theme
            .into_iter()
            .for_each(|(k, v)| log::info!("{k:?}{} = {v}", get_color(k)));

        // generate palette based on theme color
        let mut palette = palettes::core::CorePalette::new(
            score.first().expect("No colors were scored").to_owned(),
            true,
        );

        // generate dark scheme, we can use it for both light/dark situations
        let scheme = Scheme::dark_from_core_palette(&mut palette);

        // we take into account only the luminance of the viewport, aka the
        // part of the image where text is rendered
        let viewport = image.crop_imm(0, height / 3, width, height / 3);

        // use primary or on_primary based on luma value
        let luma = viewport.thumbnail(1, 1).to_luma8().into_raw()[0];
        color = if luma > 130 {
            log::info!("using on_primary color (luma is {luma})");
            scheme.on_primary
        } else {
            log::info!("using primary color (luma is {luma})");
            scheme.primary
        };

        let rgb = format!("rgb({},{},{})", color[1], color[2], color[3]);

        log::debug!("Writing generated {rgb} to cache");
        std::fs::write(fg_file, &rgb).expect("Could not write foreground file");

        // debuggin time
        print_color("Primary", scheme.primary);
        print_color("On primary", scheme.on_primary);
        print_color("Tertiary", scheme.tertiary);
        print_color("On tertiary", scheme.on_tertiary);

        return rgb;
    }
    String::new()
}

fn print_color(name: &str, c: [u8; 4]) {
    log::debug!("{name:12} \x1b[48;2;{};{};{}m  \x1b[0m", c[1], c[2], c[3]);
}
fn get_color(c: [u8; 4]) -> String {
    format!("\x1b[48;2;{};{};{}m  \x1b[0m", c[1], c[2], c[3])
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
            buf.write_all(header.as_bytes())
                .expect("Image header could not be written");

            for px in data_new {
                buf.write_all(&px)
                    .expect("File contents could not be written");
            }

            // write blurred file
            fs::write(&bg, &buf).expect("Background image could not be written");
        }
    }

    bg
}
