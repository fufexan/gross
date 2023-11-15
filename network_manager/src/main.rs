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

    println!("Primary connection: {}", primary_connection);

    let primary_connection_proxy = ActiveConnectionProxy::builder(&connection)
        .path(primary_connection)?
        .build()
        .await?;

    let id = primary_connection_proxy.id().await?;
    let connection_type = primary_connection_proxy.connection_type().await?;

    println!("id: {:?}", id);
    println!("type: {connection_type}");

    Ok(())
}
