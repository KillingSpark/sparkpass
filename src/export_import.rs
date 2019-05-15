use csv;
use crate::util::{Options, add_entry, get_tree_from_path, show_entry, prepare_entry_path};
use crate::transform;
use crate::util::TreeNode;

pub fn cmd_import(opts: &Options, prefix: &std::path::Path ,enc_params: &transform::EncryptionParams)  {
    match opts.args[0].as_str() {
        "keepass_csv" => {
            let p = std::path::Path::new(opts.args[1].as_str());
            import_from_keepass_csv(prefix, p, enc_params).unwrap();
        }
        _ => {
            println!("Unknown import type: {}", opts.args[0]);
        }
    }
}

pub fn build_entry_list(tree: &TreeNode, prefix: String) -> Vec<String> {
     let mut v = Vec::new();

    match tree {
        TreeNode::Leaf(name) => {
            let mut entry = prefix.clone();
            entry.push('/');
            entry.push_str(name.as_str());
            v.push(entry);
        }
        TreeNode::Node(name, children) => {
            let mut entry = prefix.clone();
            entry.push('/');
            entry.push_str(name.as_str());

            for c in children {
                let mut res = build_entry_list(c, entry.clone());
                v.append(&mut res);            
            }
        }
    }

    return v;
}

pub fn cmd_export(opts: &Options, prefix: &std::path::Path ,enc_params: &transform::EncryptionParams) {
    if opts.args.len() > 2 {
        println!("Too many arguments. Want: 'path_to_dir'  Got: {}", opts.args.len());
        return;
    }

    let tree = get_tree_from_path(prefix, true, enc_params).unwrap();
    let renamed_tree = match tree {
        TreeNode::Node(_, children) => {
            TreeNode::Node("".to_owned(), children)
        },
        TreeNode::Leaf(_) => TreeNode::Leaf("".to_owned())
    };

    let list = build_entry_list(&renamed_tree, "".to_owned());

    match opts.args[0].as_str() {
        "keepass_csv" => {
            let p = std::path::Path::new(opts.args[1].as_str());
            export_to_csv(list, prefix, p, enc_params).unwrap();
        }
        _ => {
            println!("Unknown import type: {}", opts.args[0]);
        }
    }
}

fn export_to_csv(entries: Vec<String>, prefix: &std::path::Path ,p: &std::path::Path, enc_params: &transform::EncryptionParams) -> Result<(), String> {
    let mut w = csv::Writer::from_path(p).unwrap();

    w.write_record(&["name", "content"]).unwrap();

    for e in entries {
        let prep_entry = prepare_entry_path(e.as_str());
        let content = show_entry(prefix, std::path::Path::new(prep_entry), enc_params).unwrap();
        w.write_record(&[prep_entry, content.as_str()]).unwrap();
    }
    return Ok(());
}

fn import_from_keepass_csv(prefix: &std::path::Path, p: &std::path::Path, enc_params: &transform::EncryptionParams) -> Result<(), String> {
   let mut r = csv::Reader::from_path(p).unwrap();
   for rcrd in r.records() {
        let record = rcrd.unwrap();
        if record.len() != 5 {
            println!("Malformed entry: {:?}", record);
            return Err("Malformed entry".to_owned());
        }

        let acc = &record[0];
        let name = &record[1];
        let passwd = &record[2];
        let url = &record[3];
        let comment = &record[4];

        let mut content = String::new();
        content.push_str(passwd);
        content.push('\n');
        content.push_str(name);
        content.push('\n');
        content.push_str(url);
        content.push('\n');
        content.push_str(acc);
        content.push('\n');
        content.push_str(comment);
        content.push('\n');

        let mut entry = String::from(url);
        entry.push_str("/");
        entry.push_str(acc);

        match add_entry(prefix, std::path::Path::new(entry.as_str()), content.as_str(), false, enc_params) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
   }
   return Ok(());
}