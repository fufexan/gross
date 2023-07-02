use futures::stream::StreamExt;
use serde_json::json;
use std::time::Duration;
use tokio::time::timeout;
use upower_dbus::{BatteryState, DeviceProxy, UPowerProxy};

#[tokio::main]
pub async fn main() -> zbus::Result<()> {
    futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;
        let proxy = UPowerProxy::new(&connection).await?;
        let display = proxy.get_display_device().await?;

        let device = DeviceProxy::builder(&connection)
            .path(display.path())
            .unwrap()
            .build()
            .await
            .unwrap();

        let mut stream = proxy.receive_on_battery_changed().await;

        loop {
            // Wait for an event with a timeout of 2 seconds
            match timeout(Duration::from_secs(2), stream.next()).await {
                Ok(Some(event)) => {
                    // display.refresh().await?;
                    println!("{}", event.name());
                    println!("{}", generator(&device).await);
                }
                Ok(None) => {
                    // No events received within the timeout period
                    println!("No events received within 2 seconds. Re-running loop.");
                    continue;
                }
                Err(_) => {
                    // Timeout occurred
                    println!("Timeout occurred. Re-running loop.");
                    continue;
                }
            }
        }
    })
}

async fn generator(device: &DeviceProxy<'_>) -> serde_json::Value {
    let state = device.state().await.unwrap_or(BatteryState::Unknown);

    let energy_rate: f64 = device.get_property("EnergyRate").await.unwrap_or(0.0);

    let rate = if energy_rate == 0.0 {
        String::new()
    } else {
        format!("{:.1}", energy_rate)
    };

    println!("{} {:?}", rate, state);

    match state {
        BatteryState::Charging | BatteryState::PendingCharge => {
            let time = get_time(device, "TimeToFull").await;

            let status = if energy_rate > 0.0 {
                format!("Charging, {time}")
            } else {
                String::from("Plugged in")
            };
            json!({
                "status": status,
                "rate": rate,
            })
        }
        BatteryState::Discharging | BatteryState::PendingDischarge => {
            let time = get_time(device, "TimeToEmpty").await;

            let status = if energy_rate > 0.0 {
                time
            } else {
                String::from("Discharging")
            };

            json!({
                "status": status,
                "rate": rate,
            })
        }
        BatteryState::FullyCharged => json!({"status": "Fully charged", "rate": ""}),
        _ => json!({"status": "", "rate": ""}),
    }
}

async fn get_time(display: &DeviceProxy<'_>, property: &str) -> String {
    let time: Result<i64, zbus::Error> = display.get_property(property).await;
    if let Ok(value) = time {
        format!("{} left", seconds_to_string(value as u64))
    } else {
        String::new()
    }
}

fn seconds_to_string(seconds: u64) -> String {
    const DAY: u64 = 24 * 60 * 60;
    const HOUR: u64 = 60 * 60;
    const MINUTE: u64 = 60;

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
    time_string.trim_start().to_string()
}
