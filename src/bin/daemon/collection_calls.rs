use dbus::{tree::MethodErr, Message, MsgHandlerResult};

pub fn handle_collection_calls(
    coll: &crate::Collection,
    msg: &Message,
    interface: &str,
    member: &str,
) -> Option<MsgHandlerResult> {
    match interface {
        "org.freedesktop.Secrets.Collection" => match member {
            _ => {
                return Some(MsgHandlerResult {
                    done: false,
                    handled: true,
                    reply: vec![dbus::tree::MethodErr::failed(&"Unknown member").to_message(msg)],
                });
            }
        },
        "DBus.Properties" => match member {
            "Get" => {
                let (iface, propname): (String, String) = msg.read2().unwrap();
                if iface != "org.freedesktop.Secrets.Collection" {
                    return Some(MsgHandlerResult {
                    done: false,
                    handled: true,
                    reply: vec![dbus::tree::MethodErr::failed(&"Tried to get property of other interface than org.freedesktop.Secrets.Collection").to_message(msg)],
                });
                }
                match propname.as_str() {
                    "Items" => {
                        let return_msg = match coll.handle_ls() {
                            Ok(v) => (*msg).method_return().append1(v),
                            Err(e) => e.to_message(msg),
                        };

                        let result = MsgHandlerResult {
                            done: false,
                            handled: true,
                            //todo generate sessions
                            reply: vec![return_msg],
                        };
                        return Some(result);
                    }
                    "Locked" => {
                        return Some(MsgHandlerResult {
                            done: false,
                            handled: true,
                            //todo generate sessions
                            reply: vec![msg.method_return().append1(false)],
                        });
                    }
                    _ => {
                        return Some(MsgHandlerResult {
                            done: false,
                            handled: true,
                            reply: vec![MethodErr::failed(&"Collection interface not implemented")
                                .to_message(msg)],
                        })
                    }
                }
            }
            "Set" => {
                let (_iface, _propname): (String, String) = msg.read2().unwrap();

                return Some(MsgHandlerResult {
                    done: false,
                    handled: true,
                    reply: vec![
                        MethodErr::failed(&"Unimplemented: Setting collection properties")
                            .to_message(msg),
                    ],
                });
            }
            "GetAll" => {
                return Some(MsgHandlerResult {
                    done: false,
                    handled: true,
                    reply: vec![
                        MethodErr::failed(&"Getting list of properties for collection")
                            .to_message(msg),
                    ],
                });
            }
            _ => {
                return Some(MsgHandlerResult {
                    done: false,
                    handled: true,
                    reply: vec![MethodErr::failed(&"Unknown member").to_message(msg)],
                });
            }
        },
        _ => {
            return Some(MsgHandlerResult {
                done: false,
                handled: true,
                reply: vec![MethodErr::failed(&"Unsupported interface").to_message(msg)],
            });
        }
    }
}
