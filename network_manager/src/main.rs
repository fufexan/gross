use std::error::Error;
use zbus::{dbus_proxy, zvariant::ObjectPath, Connection, Result};

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    #[dbus_proxy(property)]
    fn primary_connection(&self) -> Result<ObjectPath>;
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.Connection.Active",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager/ActiveConnection/0"
)]
trait ActiveConnection {
    #[dbus_proxy(property)]
    fn id(&self) -> Result<String>;
    #[dbus_proxy(property, name = "Type")]
    fn connection_type(&self) -> Result<String>;
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let connection = Connection::system().await?;

    let nm_proxy = NetworkManagerProxy::new(&connection).await?;

    let primary_connection = nm_proxy.primary_connection().await?;

    log::debug!("New primary connection: {}", primary_connection);

    let primary_connection_proxy = ActiveConnectionProxy::builder(&connection)
        .path(primary_connection)?
        .build()
        .await?;

    let id = primary_connection_proxy.id().await?;
    log::debug!("id: {:?}", id);

    let connection_type = primary_connection_proxy.connection_type().await?;
    log::debug!("type: {connection_type}");

    // event loop
    let mut nm_stream = nm_proxy.receive_primary_connection_changed().await;
    while let Some(e) = nm_stream.next().await {
        println!("{:?}", e.get().await);
    }

    Ok(())
}
