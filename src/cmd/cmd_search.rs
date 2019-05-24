use crate::util::{Options, prepare_entry_path, get_all_entries_in_path, get_tree_from_path, print_tree, flatten_tree, sort_tree_leveshtein};
use crate::transform;

use std::path;

pub fn cmd_search_fuzzy (opts: &Options, prefix: &path::Path, enc_params: &transform::EncryptionParams) {
    if opts.args.len() > 1 {
        println!("Too many arguments. Want: '[pattern]'  Got: {}", opts.args.len());
        return;
    }

    let relative_path = if opts.args.len() == 0 {
        ""
    } else {
        prepare_entry_path(opts.args[0].as_str())
    };
    let words: Vec<&str> = relative_path.split("/").collect();

    let tree = get_tree_from_path(prefix, true, enc_params).unwrap();

    let sorted_tree = sort_tree_leveshtein(&tree, words);

    if opts.show_tree {
        print_tree(&sorted_tree, "".to_owned(), false, 0); 
    }else{
        let vec = flatten_tree(&sorted_tree, "".to_owned());
        for e in vec {
            println!("{}", e);
        }
    }       
}

pub fn cmd_search(opts: &Options, prefix: &path::Path, enc_params: &transform::EncryptionParams) {
    if opts.args.len() > 1 {
        println!("Too many arguments. Want: '[pattern]'  Got: {}", opts.args.len());
        return;
    }

    let relative_path = if opts.args.len() == 0 {
        ""
    } else {
        prepare_entry_path(opts.args[0].as_str())
    };

    if opts.verbose {println!("Searching for: {}", relative_path);}
    let pp = path::Path::new(relative_path);

    //if no filename given match all names
    let last = match pp.file_name() {
        Some(n) => n.to_str().unwrap(),
        None => "",
    };

    //if no dir given, search repo root
    let dir = match pp.parent() {
        Some(n) => n.as_os_str().to_str().unwrap(),
        None => ""
    };

    let trans_path = if dir.len() > 0 {
        transform::transform_path(enc_params, dir)
    }else{
        Vec::new()
    };

    let trans_path_dir_str = trans_path.join("/");
    
    let trans_path_temp = prefix.join(&trans_path_dir_str);
    let trans_path_dir = trans_path_temp.as_path();

    if opts.verbose {println!("Searching in: {}", trans_path_dir.to_str().unwrap());}
    let entries = match get_all_entries_in_path(trans_path_dir){
        Ok(vec) => vec,
        Err(err) => {
            println!("An error occurred while listing entries: {}", err);
            return;
        },
    };


    let mut filtered = Vec::new();
    for (e, dir) in entries {
        let clear_entry = match transform::retransform_entry(enc_params, &(e[..])) {
            Ok(s) => s,
            Err(e) => {
                println!("Error occured while decrypting: {}", e); 
                return
            },
        };
        if !clear_entry.contains(last) && !(clear_entry == last) {
            continue;
        }
        filtered.push((clear_entry, dir));
    }

    if filtered.len() == 0 {
        println!("No matching entries found");
        return;
    }
    if opts.verbose {println!("Found Entries:");}

    if filtered.len() == 1 {
        let (_,dir) = & filtered[0];
        if *dir {
            crate::cmd::cmd_list::cmd_list_tree(opts, prefix, enc_params)
        }
        return;
    }

    for (e, dir) in filtered {
    if dir {
            print!("DIR ")
        }else{
            print!("ENT ")
        }
        println!("{}", e);
    }
}