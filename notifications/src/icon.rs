use std::{fs, path::PathBuf};

use image::{self, DynamicImage, RgbImage, RgbaImage};
use serde::Deserialize;
use zbus::{export::serde::Serialize, zvariant::Type, zvariant::Value};

#[derive(Debug, Default, Clone, Deserialize, Serialize, Value, Type, PartialEq)]
pub struct Icon {
    // image width
    pub width: i32,
    // image height
    pub height: i32,
    // distance in bytes between row starts
    pub rowstride: i32,
    // whether the image has an alpha channel
    pub has_alpha: bool,
    // must always be 8 (why?)
    pub bits_per_sample: i32,
    // if has_alpha, must be 4, otherwise 3
    pub channels: i32,
    // RGB byte order data
    pub data: Vec<u8>,
}

impl Icon {
    pub fn into_image(self: &Self) -> DynamicImage {
        match self.has_alpha {
            true => DynamicImage::ImageRgba8(
                RgbaImage::from_raw(self.width as u32, self.height as u32, self.clone().data)
                    .unwrap(),
            ),
            false => DynamicImage::ImageRgb8(
                RgbImage::from_raw(self.width as u32, self.height as u32, self.clone().data)
                    .unwrap(),
            ),
        }
    }

    pub fn write_image(
        self: &Self,
        app_name: &String,
        id: u32,
    ) -> Result<String, image::ImageError> {
        let image = self.into_image();

        let path =
            cache_entry(&PathBuf::from("notify-receive")).join(format!("{app_name}_{id}.png"));

        let imgres = image.save(&path);
        let imgpath = path
            .clone()
            .into_os_string()
            .into_string()
            .unwrap_or_default();

        match imgres {
            Ok(_) => Ok(imgpath),
            Err(e) => Err(e),
        }
    }
}

pub fn cache_entry(subdir: &PathBuf) -> PathBuf {
    let cache_dir = dirs::cache_dir().unwrap_or_default();

    let parent = cache_dir.join(subdir);
    if let Err(err) = fs::create_dir_all(&parent) {
        eprintln!("{parent:?} could not be created: {err}");
    } else {
        println!("{parent:?} created");
    };

    parent
}
