use dbus::{
    tree::MethodErr, BusType, Connection, Message, MessageType, MsgHandler, MsgHandlerResult,
    MsgHandlerType, NameFlag,
};

extern crate rpassword;
extern crate sparkpass;
use sparkpass::transform::{EncryptionParams, DEFAULT_IV};
use sparkpass::util::{flatten_tree, get_tree_from_path, show_entry, TreeNode};

use openssl::sha::sha256;
use std::collections::HashMap;

mod collection_calls;
mod item_calls;
mod service_calls;
mod session_calls;

pub struct Collection {
    key: Option<Vec<u8>>,
    prefix: Box<std::path::Path>,
}

pub struct Handler {
    default_coll: Collection,
}

impl MsgHandler for Handler {
    fn handler_type(&self) -> MsgHandlerType {
        MsgHandlerType::MsgType(MessageType::MethodCall)
    }

    fn handle_msg(&mut self, msg: &Message) -> Option<MsgHandlerResult> {
        let path = match msg.path() {
            Some(p) => p.clone(),
            None => return None,
        };
        let path_cstr = path.as_cstr();
        let path = String::from_utf8(path_cstr.to_bytes().to_vec()).unwrap();

        let member_cstr = msg.member().unwrap().as_cstr().to_bytes().to_vec();
        let member = String::from_utf8(member_cstr).unwrap();

        let interface_cstr = msg.interface().unwrap().as_cstr().to_bytes().to_vec();
        let interface = String::from_utf8(interface_cstr).unwrap();

        print!("Called ");
        print!("Function {}.{}", interface, member);
        print!(" on ");
        println!("Object: {}", path.as_str());

        if !path.starts_with("/org/freedesktop/Secrets") {
            panic!("invalid object path prefix");
        }

        let route: Vec<&str> = path.split("/").collect();
        let route = &route[4..];

        if route.len() == 0 {
            //main Service
            return service_calls::handle_service_calls(
                &self.default_coll,
                msg,
                interface.as_str(),
                member.as_str(),
            );
        } else {
            match route[0] {
                "default" => {
                    if route.len() == 1 {
                        return collection_calls::handle_collection_calls(
                            &self.default_coll,
                            msg,
                            interface.as_str(),
                            member.as_str(),
                        );
                    }
                    if route.len() >= 2 {
                        return item_calls::handle_item_calls(
                            &self.default_coll,
                            msg,
                            interface.as_str(),
                            member.as_str(),
                            route,
                        );
                    }
                }
                "session" => {
                    return session_calls::handle_session_calls(interface.as_str(), member.as_str());
                }
                "collection" => {
                    if route.len() == 2 {
                        return collection_calls::handle_collection_calls(
                            &self.default_coll,
                            msg,
                            interface.as_str(),
                            member.as_str(),
                        );
                    }
                    if route.len() >= 3 {
                        return item_calls::handle_item_calls(
                            &self.default_coll,
                            msg,
                            interface.as_str(),
                            member.as_str(),
                            &route[1..],
                        );
                    }
                }
                _ => panic!("unknown object"),
            }
        }

        None
    }
}

impl Collection {
    fn handle_ls(&self) -> Vec<String> {
        let key = match &self.key {
            None => {
                panic!("No key given");
            }
            Some(v) => v.clone(),
        };

        let keyhash = sha256(key.as_slice());
        let enc_params = EncryptionParams {
            key: &keyhash,
            //this iv is only used for encrypting the path. This must unfortunately be deterministic.
            iv: DEFAULT_IV,
        };

        let mut full_path = self.prefix.as_ref().to_str().unwrap().to_owned();
        full_path.push('/');

        let tree =
            match get_tree_from_path(std::path::Path::new(full_path.as_str()), true, &enc_params) {
                Ok(t) => t,
                Err(e) => {
                    panic!("Error reading entries: {}", e.to_string());
                }
            };

        let renamed_tree = match tree {
            TreeNode::Node(_, children) => TreeNode::Node("".to_owned(), children),
            TreeNode::Leaf(_) => TreeNode::Leaf("".to_owned()),
        };

        let name_list = flatten_tree(&renamed_tree, "".to_owned());
        let objectpath_list = name_list
            .iter()
            .map(|name| {
                let mut path = "/org/freedesktop/Secrets/collection/default/".to_owned();
                let trimmed_name = name.to_owned();
                path.push_str(trimmed_name.trim_matches('/'));
                path
            })
            .collect();

        objectpath_list
    }

    fn handle_show(&self, name: &str) -> Result<String, Box<std::error::Error>> {
        let key = match &self.key {
            None => {
                panic!("No key given".to_owned());
            }
            Some(v) => v.clone(),
        };

        let keyhash = sha256(key.as_slice());
        let enc_params = EncryptionParams {
            key: &keyhash,
            //this iv is only used for encrypting the path. This must unfortunately be deterministic.
            iv: DEFAULT_IV,
        };

        let content = show_entry(
            self.prefix.as_ref(),
            std::path::Path::new(name),
            &enc_params,
        );
        match content {
            Ok(c) => Ok(c.to_owned()),
            Err(e) => {
                panic!(e.as_str().to_owned());
            }
        }
    }

    fn handle_unlock(&mut self, msg: &Message) -> Result<Vec<Message>, MethodErr> {
        let n: &str = msg.read1()?;
        self.key = Some(Vec::from(n.as_bytes()));
        Ok(vec![msg.method_return()])
    }
}

fn run_default_coll() -> Result<(), dbus::Error> {
    let c = Connection::get_private(BusType::Session)?;
    c.register_name("spark.pass", NameFlag::ReplaceExisting as u32)?;

    println!("Enter key for repo");
    let pass = rpassword::read_password().unwrap();
    println!("Thanks");

    let home = std::env::var("HOME").unwrap();
    let repo = std::path::Path::new(home.as_str()).join(".sparkpass/".to_owned());
    let repo = repo.as_path();

    let handler = Handler {
        default_coll: Collection {
            key: Some(pass.as_bytes().to_vec()),
            prefix: Box::from(repo),
        },
    };

    c.add_handler(handler);
    let mut old_cb = c.replace_message_callback(None).unwrap();
    c.replace_message_callback(Some(Box::new(move |conn, m| {
        let my_b = match &m.headers() {
            (_, path, _, _) => match path {
                None => false,
                Some(path) => path.starts_with("/org/freedesktop/Secrets"),
            },
        };
        let b = old_cb(conn, m);

        return my_b || b;
    })));

    loop {
        c.incoming(100000).next();
    }
}

fn main() {
    run_default_coll().unwrap();
}
