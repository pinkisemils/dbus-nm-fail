
extern crate dbus;


use dbus::{BusType, Connection, Message, Path};
use dbus::arg::{RefArg, Variant};
use std::collections::HashMap;

fn main() {
    let c = Connection::get_private(BusType::System).expect("failed to get a DBus connection");
    let paths = get_device_path(&c);
    let (config, version, path) = get_device_config(&c, paths);
    println!("config signature - {}", config.signature());
    set_device_config(&c, &path, config, version);
    println!("success");
}

fn get_device_path(conn: &Connection) -> Vec<Path> {
    let msg = Message::new_method_call(
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        "org.freedesktop.NetworkManager",
        "GetDevices",
    )
    .expect("failed to construct a method call");

    let reply = conn
        .send_with_reply_and_block(msg, 5000)
        .expect("failed to send message");

    reply.get1().expect("failed to get device paths")
}

type DeviceConfig = HashMap<String, HashMap<String, Variant<Box<RefArg>>>>;
fn get_device_config(conn: &Connection, paths: Vec<Path>) -> (DeviceConfig, u64, Path<'static>) {

    for path in paths.into_iter() {
        let msg = Message::new_method_call(
            "org.freedesktop.NetworkManager",
            &path as &str,
            "org.freedesktop.NetworkManager.Device",
            "GetAppliedConnection",
        )
        .expect("failed to construct a method call")
        .append1(0u32);

        if let Ok(reply) = conn.send_with_reply_and_block(msg, 5000) {
            let (config, version) = reply.get2();
            let p: Path<'static> = path.into_static();
            return (
                config.expect("failed to get config"),
                version.expect("failed to get version"),
                p,
            );
        }
    }
    panic!("Failed to get config of a device");

}

fn set_device_config(conn: &Connection, path: &str, config: DeviceConfig, version: u64) {

    let msg = Message::new_method_call(
        "org.freedesktop.NetworkManager",
        path,
        "org.freedesktop.NetworkManager.Device",
        "Reapply",
    )
    .expect("failed to construct a method call")
    .append3(config, version, 0u32);

    conn.send_with_reply_and_block(msg, 5000).expect("This should work");
}

