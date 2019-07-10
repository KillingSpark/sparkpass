use dbus::{Message, MsgHandlerResult};

pub fn handle_collection_calls(
    coll: &crate::Collection,
    msg: &Message,
    interface: &str,
    member: &str,
) -> Option<MsgHandlerResult> {
    match interface {
        "org.freedesktop.Secrets.Collection" => match member {
            _ => {
                panic!("Unknown command");
            }
        },
        "DBus.Properties" => match member {
            "Get" => {
                let (iface, propname): (String, String) = msg.read2().unwrap();
                if iface != "org.freedesktop.Secrets.Collection" {
                    panic!("Tried to get property of other interface than org.freedesktop.Secrets.Collection");
                }
                match propname.as_str() {
                    "Items" => {
                        let return_msg = (*msg)
                            .method_return()
                            .append1(coll.handle_ls());

                        let result = MsgHandlerResult {
                            done: false,
                            handled: true,
                            //todo generate sessions
                            reply: vec![return_msg],
                        };
                        return Some(result);
                    }
                    _ => unimplemented!("Collection interface"),
                }
            }
            "Set" => {
                let (iface, propname): (String, String) = msg.read2().unwrap();
                println!("{}, {}", iface, propname);
                unimplemented!("Setting collection properties");
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
