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
                    panic!("Tried to get property of other interface than org.freedesktop.Secrets.Item");
                }
                let name = route[2..].join("/");
                println!("{}, {}, {}", iface, propname, name);
                let return_msg = (*msg)
                    .method_return()
                    .append1(coll.handle_show(name.as_str()).unwrap());

                let result = MsgHandlerResult {
                    done: false,
                    handled: true,
                    //todo generate sessions
                    reply: vec![return_msg],
                };
                return Some(result);
            }
            "Set" => {
                let (iface, propname): (String, String) = msg.read2().unwrap();
                println!("{}, {}", iface, propname);
                unimplemented!("Setting secrtes");
            }
            "GetAll" => {
                unimplemented!("GetAll for properties");
            }
            _ => {
                panic!("Unknown command");
            }
        },
        _ => {
            panic!(
                "Called default collection with wrong interface: {}",
                interface
            );
        }
    }
}
