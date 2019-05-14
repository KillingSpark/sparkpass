use crate::util::{TreeNode, Options, prepare_entry_path,  get_tree_from_path};
use crate::transform;

use std::path;


fn print_tree(tree: &TreeNode, prefix: String, last: bool, level: i32) {
     match tree {
        TreeNode::Leaf(name) => {
            if level > 0 {
                print!("{}", prefix);

                if last {
                    println!("└── {}", name);
                }else{
                    println!("├── {}", name);
                }
            }else{
                println!("{}", name);
            }
        },
        TreeNode::Node(name, children) => {
            if level != 0 { 
                print!("{}", prefix);
                if last {
                    println!("└── {}", name);
                }else{
                    println!("├── {}", name);
                }
            }else {
                println!("{}", name);
            }

            let mut i = 0;
            for c in children {
                i+=1;
                let mut prefix_new = prefix.clone();
                if level > 0 {
                    if !last {
                        prefix_new.push_str("│   ");
                    }else{
                        prefix_new.push_str("    ");
                    }
                }

                if i != children.len() {
                    print_tree(c, prefix_new, false, level+1);
                }else{
                    print_tree(c, prefix_new, true, level+1);
                }
            }
        }
    }
}

pub fn cmd_list_tree(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
    if opts.args.len() > 1 {
        println!("Too many arguments. Want: 'path_to_dir'  Got: {}", opts.args.len());
        return;
    }

    let mut is_root = false;

    //check if any path needs to be appended to the prefix
    let pp = if opts.args.len() > 0 && opts.args[0].len() > 0 && opts.args[0] != "/" {
        let relative_path = prepare_entry_path(opts.args[0].as_str());

        let trans_path_tmp = transform::transform_path(enc_params, relative_path);
        prefix.join(trans_path_tmp.join("/"))
    } else{
        is_root = true;
        prefix.to_path_buf()
    };

    let full_path = pp.as_path();
    
    if opts.verbose {println!("Listing in: {}", full_path.to_str().unwrap());}

    let tree = match get_tree_from_path(full_path, is_root, enc_params){
        Ok(t) => t,
        Err(err) => {
            println!("An error occured while listing entries: {}", err);
            return;
        },
    };

    let renamed_tree = if is_root {
        match tree {
            TreeNode::Node(_, children) => {
                TreeNode::Node("repo".to_owned(), children)
            },
            TreeNode::Leaf(_) => TreeNode::Leaf("repo".to_owned())
        }
    }else{
        tree
    };

    print_tree(&renamed_tree, "".to_owned(), false, 0);
}

//fn cmd_list(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
//    if opts.args.len() > 1 {
//        println!("Too many arguments. Want: 'path_to_dir'  Got: {}", opts.args.len());
//        return;
//    }
//
//    //check if any path needs to be appended to the prefix
//    let pp = if opts.args.len() > 0 && opts.args[0].len() > 0 && opts.args[0] != "/" {
//        let relative_path = prepare_entry_path(opts.args[0].as_str());
//
//        let trans_path_tmp = transform::transform_path(enc_params, relative_path);
//        prefix.join(trans_path_tmp.join("/"))
//    } else{
//        prefix.to_path_buf()
//    };
//
//    let full_path = pp.as_path();
//    
//    if opts.verbose {println!("Listing in: {}", full_path.to_str().unwrap());}
//
//    let entries = match get_all_entries_in_path(full_path){
//        Ok(vec) => vec,
//        Err(err) => {
//            println!("An error occured while listing entries: {}", err);
//            return;
//        },
//    };
//
//    if entries.len() == 0 {
//        if opts.verbose {println!("No entries found")};
//        return;
//    }
//
//    if opts.verbose {println!("Found entries:");}
//    for (e, dir) in entries {
//        let clear_entry = transform::retransform_entry(enc_params, &(e[..]));
//        if dir {
//            print!("DIR ")
//        }else{
//            print!("ENT ")
//        }
//        println!("{}", clear_entry);
//    }
//}