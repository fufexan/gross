extern crate futures;
extern crate upower_dbus;

use futures::stream::StreamExt;
use upower_dbus::UPowerProxy;

pub fn main() -> zbus::Result<()> {
    futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;

        let upower = UPowerProxy::new(&connection).await?;

        println!("On Battery: {:?}", upower.on_battery().await);

        let mut stream = upower.receive_on_battery_changed().await;

        while let Some(event) = stream.next().await {
            println!("On Battery: {:?}", event.get().await);
        }

        Ok(())
    })
}
