use dbus::{Message, MsgHandlerResult};

pub fn handle_item_calls(
    coll: &crate::Collection,
    msg: &Message,
    interface: &str,
    member: &str,
    route: &[&str],
) -> Option<MsgHandlerResult> {
    match interface {
        "org.freedesktop.Secrets.Item" => match member {
            _ => {
                panic!("Unknown command");
            }
        },
        "DBus.Properties" => match member {
            "Get" => {
                let (iface, propname): (String, String) = msg.read2().unwrap();
                if iface != "org.freedesktop.Secrets.Item" {
                    return Some(MsgHandlerResult {
                        done: false,
                        handled: true,
                        reply: vec![dbus::tree::MethodErr::failed(
                            &"Tried to get property of other interface than org.freedesktop.Secrets.Item",
                        )
                        .to_message(msg)],
                    });
                }
                let name = route[2..].join("/");
                println!("{}, {}, {}", iface, propname, name);
                let return_msg = (*msg)
                    .method_return()
                    .append1(coll.handle_show(name.as_str()).unwrap());

                let result = MsgHandlerResult {
                    done: false,
                    handled: true,
                    //TODO generate sessions
                    reply: vec![return_msg],
                };
                return Some(result);
            }
            "Set" => {
                let (_iface, _propname): (String, String) = msg.read2().unwrap();
                return Some(MsgHandlerResult {
                    done: false,
                    handled: true,
                    reply: vec![dbus::tree::MethodErr::failed(
                        &"Setting properties is not supported",
                    )
                    .to_message(msg)],
                });
            }
            "GetAll" => {
                return Some(MsgHandlerResult {
                    done: false,
                    handled: true,
                    reply: vec![dbus::tree::MethodErr::failed(
                        &"Getting a list of properties is not supported",
                    )
                    .to_message(msg)],
                });
            }
            _ => {
                return Some(MsgHandlerResult {
                    done: false,
                    handled: true,
                    reply: vec![dbus::tree::MethodErr::failed(&"Unknown member").to_message(msg)],
                });
            }
        },
        _ => {
            return Some(MsgHandlerResult {
                done: false,
                handled: true,
                reply: vec![dbus::tree::MethodErr::failed(&"Unknown member").to_message(msg)],
            });
        }
    }
}
