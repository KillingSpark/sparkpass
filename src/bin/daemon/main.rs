use dbus::{tree::Factory, tree::MethodErr, BusType, Connection, Message, NameFlag};

extern crate sparkpass;
use sparkpass::transform::{EncryptionParams, transform_path, DEFAULT_IV};
use sparkpass::util::{flatten_tree, get_tree_from_path, show_entry, TreeNode};

use openssl::sha::sha256;

struct Daemon {
    key: Option<Vec<u8>>,
    prefix: Box<std::path::Path>,
}

impl Daemon {
    fn handle_ls(&self, msg: &Message) -> Result<Vec<Message>, MethodErr> {
        let name: &str = msg.read1()?;

        let key = match &self.key {
            None => {
                return Err(MethodErr::failed(&"No key given".to_owned()));
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

        let trans_name = transform_path(&enc_params, name);
        full_path.push_str(trans_name.join("/").as_str());

        let tree = match get_tree_from_path(std::path::Path::new(full_path.as_str()), true, &enc_params) {
            Ok(t) => t,
            Err(_) => {
                return Err(MethodErr::failed(&"Error while reading entries".to_owned()));
            }
        };

        let renamed_tree = match tree {
            TreeNode::Node(_, children) => TreeNode::Node("".to_owned(), children),
            TreeNode::Leaf(_) => TreeNode::Leaf("".to_owned()),
        };

        let name_list = flatten_tree(&renamed_tree, "".to_owned());
        let result = name_list.join("\n");

        Ok(vec![msg.method_return().append1(result)])
    }

    fn handle_show(&self, msg: &Message) -> Result<Vec<Message>, MethodErr> {
        let name: &str = msg.read1()?;

        let key = match &self.key {
            None => {
                return Err(MethodErr::failed(&"No key given".to_owned()));
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
            Ok(c) => Ok(vec![msg.method_return().append1(c)]),
            Err(e) => {
                return Err(MethodErr::failed(&e.clone()));
            }
        }
    }

    fn handle_unlock(&mut self, msg: &Message) -> Result<Vec<Message>, MethodErr> {
        let n: &str = msg.read1()?;
        self.key = Some(Vec::from(n.as_bytes()));
        Ok(vec![msg.method_return()])
    }
}

fn run_daemon() -> Result<(), dbus::Error> {
    let c = Connection::get_private(BusType::Session)?;
    c.register_name("spark.pass", NameFlag::ReplaceExisting as u32)?;
    let f = Factory::new_fn::<()>();

    let dmn = Daemon {
        key: None,
        prefix: Box::from(std::path::Path::new("/home/moritz/.sparkpass/")),
    };

    let dmn_rc = std::sync::Arc::new(std::cell::RefCell::new(dmn));
    let dmn_ls = dmn_rc.clone();
    let dmn_ul = dmn_rc.clone();
    let dmn_shw = dmn_rc.clone();

    let tree = f.tree(()).add(
        f.object_path("/repo", ()).introspectable().add(
            f.interface("spark.pass", ())
                .add_m(
                    f.method("List", (), move |m| dmn_ls.borrow().handle_ls(&m.msg))
                        .inarg::<&str, _>("name")
                        .outarg::<&str, _>("reply"),
                )
                .add_m(
                    f.method("Unlock", (), move |m| {
                        dmn_ul.borrow_mut().handle_unlock(&m.msg)
                    })
                    .inarg::<&str, _>("key"),
                )
                .add_m(
                    f.method("Show", (), move |m| {
                        dmn_shw.borrow_mut().handle_show(&m.msg)
                    })
                    .inarg::<&str, _>("name")
                    .outarg::<&str, _>("reply"),
                ),
        ),
    );

    tree.set_registered(&c, true)?;
    c.add_handler(tree);

    loop {
        c.incoming(100000).next();
    }
}

fn main() {
    run_daemon().unwrap();
}
