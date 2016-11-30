extern crate dbus;

use dbus::{Connection, BusType, Message};
use dbus::obj::ObjectPath;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_list_connections() {
//         let connection_items = list_connection_items();
//         assert_eq!(vec!["tan", "stand", "at", "yo"], connection_items);
//     }
// }

pub fn get_connection_settings(s: &str) -> Vec<dbus::MessageItem> {
    let c = Connection::get_private(BusType::System).unwrap();
    let m = Message::new_method_call("org.freedesktop.NetworkManager",
                                     s,
                                     "org.freedesktop.NetworkManager.Settings.Connection",
                                     "GetSettings")
        .unwrap();
    c.send_with_reply_and_block(m, 2000).unwrap().get_items()
}

pub fn get_connection_list() {
    let c = Connection::get_private(BusType::System);
    assert_eq!(c.is_ok(), true);
    let c = c.unwrap();

    let m = Message::new_method_call("org.freedesktop.NetworkManager",
                                     "/org/freedesktop/NetworkManager/Settings",
                                     "org.freedesktop.NetworkManager.Settings",
                                     "ListConnections")
        .unwrap();
    let r = c.send_with_reply_and_block(m, 2000);
    assert_eq!(r.is_ok(), true);
    let r = r.unwrap();

    // println!("{:?}", r);

    let mut i = r.iter_init();
    println!("{:?}", i.get());


    // println!("{:?}", i.next());


    println!("{:?}", i);
    // println!("{:?}", r.get1());






    // for name in arr {
    //     println!("{}", name);
    // }

}





// get_items(&self) -> Vec<MessageItem>

// iter_init<'a>(&'a self) -> Iter<'a>

// List devices