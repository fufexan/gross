use dirs;
use std::{fs, path::PathBuf, time::Duration};

pub fn get_time(duration: Duration) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = 60 * MINUTE;
    let mut time = String::new();

    let secs = duration.as_secs();
    let whole_hours = secs / HOUR;

    if whole_hours > 0 {
        time.push_str(&format!("{whole_hours:02}:"));
    }

    let secs = secs - whole_hours * HOUR;
    let whole_minutes = secs / MINUTE;
    let secs = secs - whole_minutes * MINUTE;

    time.push_str(&format!("{whole_minutes:02}:{secs:02}"));

    time
}

/// Convert an array slice to a 3-dimensional vector (r,g,b)
pub fn unflatten(data: &[u8]) -> Vec<[u8; 3]> {
    data.chunks(3).map(|rgb| [rgb[0], rgb[1], rgb[2]]).collect()
}

pub fn cache_entry(file: &PathBuf, parent: &str) -> PathBuf {
    let cache_dir = dirs::cache_dir().unwrap_or_default();
    let file_new = cache_dir
        .join(parent)
        .join(file.file_stem().unwrap_or_else(|| "cover".as_ref()));
    log::trace!("new cache entry at {file_new:?}");

    if let Err(err) = fs::create_dir_all(
        file_new
            .parent()
            .unwrap_or_else(|| panic!("Could not get parent of {file:?}")),
    ) {
        log::warn!("{parent} could not be created. {err}");
    } else {
        log::trace!("{parent} created");
    };
    file_new
}
