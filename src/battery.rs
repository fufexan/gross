use battery::{Battery, Manager, State};
use serde_json::json;

pub fn main() {
    let manager = Manager::new().expect("Could not create battery manager");
    let mut bat = manager
        .batteries()
        .expect("Could not get batteries")
        .next()
        .expect("Could not get battery")
        .expect("Could not get battery");

    let mut old_status = json!({});

    loop {
        manager
            .refresh(&mut bat)
            .expect("Could not refresh battery");
        let status = generator(&bat);

        if status != old_status {
            println!("{status}");
            old_status = status;
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

fn generator(battery: &Battery) -> serde_json::Value {
    let energy_rate = battery.energy_rate().value;

    let rate = if energy_rate == 0.0
        || (battery.state() != State::Charging && battery.state() != State::Discharging)
    {
        String::new()
    } else {
        format!("{energy_rate:.1} W")
    };

    let status = match battery.state() {
        State::Charging => {
            #[allow(clippy::cast_possible_truncation)]
            let ttf = battery
                .time_to_full()
                .expect("Could not get time to full")
                .value as i64;

            let time = seconds_to_string(ttf);

            if energy_rate > 0.0 {
                format!("Charging, {time}")
            } else {
                String::from("Plugged in")
            }
        }
        State::Discharging => {
            #[allow(clippy::cast_possible_truncation)]
            let tte = battery
                .time_to_empty()
                .expect("Could not get time to empty")
                .value as i64;

            let time = seconds_to_string(tte);

            if energy_rate > 0.0 {
                time
            } else {
                String::from("Discharging")
            }
        }
        State::Full => String::from("Fully charged"),
        _ => String::new(),
    };

    json!({
        "rate": rate,
        "status": status
    })
}

fn seconds_to_string(seconds: i64) -> String {
    const MINUTE: i64 = 60;
    const HOUR: i64 = 60 * MINUTE;
    const DAY: i64 = 24 * HOUR;

    let mut time_string = String::new();

    let days = seconds / (DAY);
    if days > 0 {
        time_string += &format!("{days}d");
    }

    let hours = (seconds % DAY) / HOUR;
    if hours > 0 {
        time_string += &format!(" {hours}h");
    }

    let minutes = (seconds % HOUR) / MINUTE;
    if minutes > 0 {
        time_string += &format!(" {minutes}m");
    }

    time_string.push_str(" left");
    time_string.trim_start().to_string()
}
