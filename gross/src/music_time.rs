use mpris::{Player, PlayerFinder};
use serde_json::json;
use std::time::Duration;

use crate::music::utils;

pub fn main() {
    loop {
        let player = PlayerFinder::new()
            .expect("Failed to create PlayerFinder")
            .find_active();

        match player {
            Ok(player) => {
                monitor_player(&player);
            }
            Err(err) => {
                println!();
                eprintln!("Failed to find active player: {err}");
                // Wait for a while before searching for players again
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

fn get_position_data(player: &Player) -> serde_json::Value {
    let position;
    let position_percent;
    if let Some(length) = player
        .get_metadata()
        .expect("Could not get metadata")
        .length()
    {
        let pos = player.get_position().expect("Could not get position");
        position = utils::get_time(pos);
        position_percent = pos.as_secs_f64() * 100.0 / length.as_secs_f64();
    } else {
        position = String::new();
        position_percent = 0.0;
    };

    json!({
        "position": position,
        "position_percent": format!("{position_percent:.2}"),
    })
}

fn monitor_player(player: &Player) {
    let mut old_data = json!({});

    println!("{}", get_position_data(player));

    loop {
        let data = get_position_data(player);
        if data != old_data {
            println!("{data}");
            old_data = data;
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}
