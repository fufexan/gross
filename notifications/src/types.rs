use crate::icon::Icon;
use serde::Serialize;
use zbus::zvariant::{DeserializeDict, Optional, SerializeDict, Type};

#[derive(Debug, Default, Clone, DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "dict")]
pub struct Hints {
    pub action_icons: Optional<bool>,
    pub category: Optional<String>,
    #[zvariant(rename = "desktop-entry")]
    pub desktop_entry: Optional<String>,
    pub image_data: Optional<Icon>,
    #[zvariant(rename = "image-data")]
    pub image_data_: Optional<Icon>,
    #[zvariant(rename = "image-path")]
    pub image_path: Optional<String>,
    pub image_path_: Optional<String>,
    pub icon_data: Optional<Icon>,
    pub resident: Optional<bool>,
    #[zvariant(rename = "sound-file")]
    pub sound_file: Optional<String>,
    #[zvariant(rename = "sound-name")]
    pub sound_name: Optional<String>,
    #[zvariant(rename = "suppress-sound")]
    pub suppress_sound: Optional<bool>,
    pub transient: Optional<bool>,
    pub x: Optional<i32>,
    pub y: Optional<i32>,
    pub urgency: u8,
}

#[derive(Serialize)]
pub struct Notifications<'a> {
    pub notifications: Vec<&'a Notification>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub time: String,
    pub image: String,
    pub timeout: i32,
    pub urgency: u8,
    pub actions: Vec<String>,
    pub visible: bool,
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum Action {
//     Close,
//     Clear,
//     Notify(Notification),
// }
