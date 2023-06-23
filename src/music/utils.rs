use dirs;
use std::{fs, path::PathBuf, time::Duration};

pub fn get_time(duration: Duration) -> String {
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

/// Convert an array slice to a 3-dimensional vector (r,g,b)
pub fn unflatten(data: &[u8]) -> Vec<[u8; 3]> {
    data.chunks(3).map(|rgb| [rgb[0], rgb[1], rgb[2]]).collect()
}

pub fn cache_entry(file: &PathBuf, parent: &str) -> PathBuf {
    let cache_dir = dirs::cache_dir().unwrap();
    let file = cache_dir.join(parent).join(file.file_stem().unwrap());
    fs::create_dir_all(
        file.parent()
            .expect(format!("Could not get parent of {:?}", file).as_str()),
    )
    .expect(format!("{} could not be created", parent).as_str());

    return file;
}
