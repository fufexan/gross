use crate::icon::Icon;
use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Debug, Default, Clone, Deserialize, Serialize, Type)]
pub struct Hints {
    pub action_icons: bool,
    pub category: String,
    pub desktop_entry: String,
    pub image_data: Icon,
    pub image_path: String,
    pub icon_data: Icon,
    pub resident: bool,
    pub sound_file: String,
    pub sound_name: String,
    pub suppress_sound: bool,
    pub transient: bool,
    pub x: i32,
    pub y: i32,
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
