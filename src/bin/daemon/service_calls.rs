use dbus::{Message, MsgHandlerResult};
use std::collections::HashMap;

pub fn handle_service_calls(
    coll: &crate::Collection,
    msg: &Message,
    interface: &str,
    member: &str,
) -> Option<MsgHandlerResult> {
    if interface != "org.freedesktop.Secrets.Service" {
        panic!("Called service with wrong interface: {}", interface);
    }
    match member {
        "OpenSession" => {
            let return_msg = (*msg).method_return();
            let result = MsgHandlerResult {
                done: false,
                handled: true,
                //todo generate sessions
                reply: vec![return_msg.append1("/org/freedesktop/Secrets/Session/abcde")],
            };
            return Some(result);
        }
        "SearchCollections" => {
            let search_dict: HashMap<String, String> = msg.read1().unwrap();

            let return_msg = (*msg)
                .method_return()
                .append1(coll.handle_ls())
                .append1(Vec::<String>::new());

            let result = MsgHandlerResult {
                done: false,
                handled: true,
                //todo generate sessions
                reply: vec![return_msg],
            };
            return Some(result);
        }
        "RetrieveSecrets" => {
            let paths: Vec<String> = msg.read1().unwrap();
            let mut secrets = Vec::new();
            for p in paths {
                println!("Wanna get: {}", p);
                let item = p
                    .trim_start_matches("/org/freedesktop/Secrets/collection/default/")
                    .to_owned();
                secrets.push(coll.handle_show(item.as_str()).unwrap());
            }
            let return_msg = (*msg).method_return().append1(secrets);

            let result = MsgHandlerResult {
                done: false,
                handled: true,
                //todo generate sessions
                reply: vec![return_msg],
            };
            return Some(result);
        }
        _ => unimplemented!("Service interface"),
    }
}
