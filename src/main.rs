mod transform;
mod generate;

extern crate shellexpand;
extern crate argparse;

use std::path;
use std::fs;
use std::str;
use std::io;


use argparse::{ArgumentParser, Store, StoreTrue, Collect};

use openssl::sha::sha256;

struct Options {
    args: Vec<String>,
    key: String,
    repo: String,
    verbose: bool,
    recursive: bool,
    force: bool,
}

fn read_key_from_terminal() -> String {
    println!("Enter key to repo (it is recommended to use SPARKPASS_KEY instead of interactive entering): ");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read line");
    return key.trim_end().to_owned();
}

fn main() {
    let mut options = Options {
        args: std::vec::Vec::new(),
        key: String::new(),
        repo: String::new(),
        verbose: false,
        recursive: false,
        force: false,
    };

    let mut command = String::new();

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Save and retrieve passwords.");

        ap.refer(&mut command)
            .add_argument("command", Store,
            "Command to run in the repo (see 'man pass' for a list. Some are not implemented)");

        ap.refer(&mut options.args)
            .add_argument("args", Collect,
            "arguments depending on the command");

         ap.refer(&mut options.verbose)
            .add_option(&["--verbose", "-v"], StoreTrue,
            "More print outs");

        ap.refer(&mut options.force)
            .add_option(&["--force", "-f"], StoreTrue,
            "Force overwrites for copy/move/generate/add");

        ap.refer(&mut options.recursive)
            .add_option(&["--recursive", "-r"], StoreTrue,
            "Remove contents of directories");

        ap.refer(&mut options.repo)
            .add_option(&["--repo", "-p"], Store,
            "Path to the repo where your keys are");

        ap.refer(&mut options.key)
            .add_option(&["--key", "-k"], Store,
            "Your master key");
        ap.parse_args_or_exit();
    }

    if options.key == "" || options.repo == "" {
        //search for env variabales if not given by options
        for (var, val) in std::env::vars() {
            match var.as_str() {
                "SPARKPASS_KEY" => {
                    if options.key == "" {
                        options.key = val;
                    }else{
                        //ignore
                    }
                },
                "SPARKPASS_REPO" => {
                    if options.repo == "" {
                        options.repo = val;
                    }else{
                        //ignore
                    }
                },
                _ => {},
            }
        }
    }

    if options.key == "" {
        if options.verbose {
            println!("Need a key to retrieve passwords. Instead of interactive entering you can use either the --key/-k options or the SPARKPASS_KEY environment variable");
        }
        options.key = read_key_from_terminal();
    }

    if options.repo == "" {
        let home = std::env::var("HOME").unwrap();
        options.repo = path::Path::new(home.as_str()).join(".sparkpass/".to_owned()).to_str().unwrap().to_owned();
        if options.verbose {
            println!("Repo not specified (use either SPARKPASS_REPO or --repo/-r), falling back to default {}", options.repo)
        }
    }

    let keyhash = sha256(options.key.as_bytes());
    let enc_params = transform::EncryptionParams{
        key: &keyhash,
        iv:  b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07",
    };

    let repopath = path::Path::new(options.repo.as_str());

    match &(command)[..] {
        "ls" => {
            cmd_list_tree(&options, repopath, &enc_params);
        },
        "list" => {
            cmd_list_tree(&options, repopath, &enc_params);
        }

        "grep" => {
            println!("This command is currently not supported. Sorry");
            return;
        },

        "find" =>  {
            cmd_search(&options, repopath, &enc_params);
        },
        "search" => {
            cmd_search(&options, repopath, &enc_params);
        },
        
        "show" => {
            cmd_show(&options, repopath, &enc_params);
        },

        "add" => {
            cmd_add(&options, repopath, &enc_params);
        },
        "insert" => {
            cmd_add(&options, repopath, &enc_params);
        },

        "edit" => {
            println!("This command is currently not supported. Sorry");
            return;
        },

        "generate" => {
            cmd_generate(&options, repopath, &enc_params);
        },

        "rm" => {
            cmd_remove(&options, repopath, &enc_params);
        },
        "remove" => {
            cmd_remove(&options, repopath, &enc_params);
        },
        "delete" => {
            cmd_remove(&options, repopath, &enc_params);
        },

        "mv" => {
            cmd_move(&options, repopath, &enc_params);
        },
        "rename" => {
            cmd_move(&options, repopath, &enc_params);
        },

        "cp" => {
            cmd_copy(&options, repopath, &enc_params);
        },
        "copy" => {
            cmd_copy(&options, repopath, &enc_params);
        },

        "git" => {
            println!("#####################################");
            println!("######         No.         ##########");
            println!("#####################################");
            return;
        }

        _ => {
            if options.args.len() == 0 {
                //no command was given and cmd collected the path to show/list
                options.args.push(command);
                cmd_show(&options,repopath,  &enc_params);
            }else{
                println!("Not implemented command: {}", command)
            }
        }, 
    };
}

//remove slashes at the start and end
fn prepare_entry_path(path: &str) -> &str {
    let mut tmp = path.trim_start_matches("/");
    tmp = tmp.trim_end_matches("/");

    return tmp;
}

fn cmd_move(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
    if opts.args.len() != 2 {
        println!("Too many arguments. Want: 'old_path, new_path'  Got: {}", opts.args.len());
        return;
    }

    let relative_path_old = prepare_entry_path(opts.args[0].as_str());
    let trans_path_old = transform::transform_path(enc_params, relative_path_old);
    let full_path_old = prefix.join(trans_path_old.join("/"));

    let relative_path_new = prepare_entry_path(opts.args[1].as_str());
    let trans_path_new = transform::transform_path(enc_params, relative_path_new);
    let full_path_new = prefix.join(trans_path_new.join("/"));


    if opts.verbose {println!("Moving Entry: {}, To: {}", relative_path_old, relative_path_new);}

    if full_path_new.exists() && !opts.force {
        println!("Target exists already!");
        return;
    }

    match fs::rename(full_path_old, full_path_new) {
        Ok(_) => {},
        Err(e) => println!("An error occured while moving: {}", e),
    }
}

fn cmd_copy(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
    if opts.args.len() != 2 {
        println!("Too many arguments. Want: 'old_path, new_path'  Got: {}", opts.args.len());
        return;
    }

    let relative_path_old = prepare_entry_path(opts.args[0].as_str());
    let trans_path_old = transform::transform_path(enc_params, relative_path_old);
    let full_path_old = prefix.join(trans_path_old.join("/"));

    let relative_path_new = prepare_entry_path(opts.args[1].as_str());
    let trans_path_new = transform::transform_path(enc_params, relative_path_new);
    let full_path_new = prefix.join(trans_path_new.join("/"));


    if opts.verbose {println!("Copying Entry: {}, To: {}", relative_path_old, relative_path_new);}

    if full_path_new.exists() && !opts.force {
        println!("Target exists already!");
        return;
    }

    if full_path_old.is_dir() {
        println!("Copying dirs is not yet supported");
    }else{
        match full_path_new.parent() {
            Some(p) => {
                match fs::create_dir_all(p){
                    Ok(_) => {},
                        Err(e) => {println!("An error occured while creating the needed directories: {}", e);
                        return;
                    },
                }
            },
            None => {},
        };

        match fs::copy(full_path_old, full_path_new){
            Ok(_) => {},
            Err(e) => {
                println!("An error occured while copying to new location: {}", e);
                return;
            },
        };
    }
}

fn cmd_remove(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
    if opts.args.len() != 1 {
        println!("Too many arguments. Want: 'path'  Got: {}", opts.args.len());
        return;
    }

    let relative_path = prepare_entry_path(opts.args[0].as_str());
    if opts.verbose {println!("Removing Entry: {}", relative_path);}

    let trans_path = transform::transform_path(enc_params, relative_path);
    let full_path = prefix.join(trans_path.join("/"));

    if full_path.is_file() {
        match fs::remove_file(full_path) {
            Ok(_) => {},
            Err(e) => {
                println!("An error occured while removing: {}", e);
                return;
            },
        }
    } else { 
        if full_path.is_dir() {
            if opts.recursive {
                match fs::remove_dir_all(full_path) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("An error occured while removing: {}", e);
                        return;
                    },
                }
            } else {
                println!("Tried to remove directory without recursive flag set");
            }
        }
    }
}

fn cmd_generate(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
    if opts.args.len() > 2 {
        println!("Too many arguments. Want: 'path, [length]'  Got: {}", opts.args.len());
        return;
    }

    let relative_path = prepare_entry_path(opts.args[0].as_str());
    
    let passwd = if opts.args.len() >= 2 {
        let length = match opts.args[1].trim().parse() {
            Ok(i) => i,
            Err(e) => {
                println!("Error while converting argument to number: {}", e);
                return;
            },
        };
        generate::generate_passwd(length)
    }else{
        generate::generate_passwd(64)
    };

    match add_entry(prefix, path::Path::new(relative_path), passwd.as_str(), opts.force, enc_params) {
        Ok(_) => {},
        Err(e) => {
            println!("An error occured while adding the entry: {}", e);
            return;
        },
    }
}

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
                        prefix_new.push_str("   ");
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

fn cmd_list_tree(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
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

fn cmd_list(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
    if opts.args.len() > 1 {
        println!("Too many arguments. Want: 'path_to_dir'  Got: {}", opts.args.len());
        return;
    }

    //check if any path needs to be appended to the prefix
    let pp = if opts.args.len() > 0 && opts.args[0].len() > 0 && opts.args[0] != "/" {
        let relative_path = prepare_entry_path(opts.args[0].as_str());

        let trans_path_tmp = transform::transform_path(enc_params, relative_path);
        prefix.join(trans_path_tmp.join("/"))
    } else{
        prefix.to_path_buf()
    };

    let full_path = pp.as_path();
    
    if opts.verbose {println!("Listing in: {}", full_path.to_str().unwrap());}

    let entries = match get_all_entries_in_path(full_path){
        Ok(vec) => vec,
        Err(err) => {
            println!("An error occured while listing entries: {}", err);
            return;
        },
    };

    if entries.len() == 0 {
        if opts.verbose {println!("No entries found")};
        return;
    }

    if opts.verbose {println!("Found entries:");}
    for (e, dir) in entries {
        let clear_entry = transform::retransform_entry(enc_params, &(e[..]));
        if dir {
            print!("DIR ")
        }else{
            print!("ENT ")
        }
        println!("{}", clear_entry);
    }
}

fn cmd_show(opts: &Options, prefix: &path::Path, enc_params: &transform::EncryptionParams) {
    if opts.args.len() != 1 {
        println!("Too many arguments. Want: 'path_to_entry'  Got: {}", opts.args.len());
        return;
    }

    let relative_path = prepare_entry_path(opts.args[0].as_str());
    let content = match show_entry(prefix, path::Path::new(relative_path), enc_params) {
        Ok(c) => c,
        Err(_) => {
            //entry doesnt exist. Search for it instead
            cmd_search(opts, prefix, enc_params);
            return;
        },
    };

    if opts.verbose {println!("Showing entry: {}", relative_path);}
    if opts.verbose {
        println!("Content: {}", content);
    }else{
        print!("{}", content);
    }
}

fn cmd_search(opts: &Options, prefix: &path::Path, enc_params: &transform::EncryptionParams) {
    if opts.args.len() > 1 {
        println!("Too many arguments. Want: 'pattern'  Got: {}", opts.args.len());
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
            println!("An error occured while listing entries: {}", err);
            return;
        },
    };


    let mut filtered = Vec::new();
    for (e, dir) in entries {
        let clear_entry = transform::retransform_entry(enc_params, &(e[..]));
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

    for (e, dir) in filtered {
    if dir {
            print!("DIR ")
        }else{
            print!("ENT ")
        }
        println!("{}", e);
    }
}

fn cmd_add(opts: &Options, prefix: &path::Path, enc_params: &transform::EncryptionParams) {
    if opts.args.len() != 2 {
        println!("Too many arguments. Want: 'path_new, content'  Got: {}", opts.args.len());
        return;
    }

    let relative_path = prepare_entry_path(opts.args[0].as_str());
    if opts.verbose {println!("Adding Entry: {}, Content: {}", relative_path, opts.args[1]);}


    match add_entry(prefix, path::Path::new(relative_path), opts.args[1].as_str(), opts.force, enc_params){
        Ok(_) => {},
        Err(e) => {
            println!("An error occured while adding the entry: {}", e);
            return;
        },
    }
}

fn add_entry(prefix : &path::Path, p: &path::Path, content: &str, overwrite: bool, enc_params: &transform::EncryptionParams) -> Result<(), String> {
    let trans_path = transform::transform_path(enc_params, p.to_str().unwrap()).join("/");
    let full_path = prefix.join(trans_path.clone());

    let exists = match fs::metadata(full_path.clone()) {
        Ok(_) => true,
        Err(_) => false,
    };

    if exists && !overwrite {
        return Err("Entry exists already".to_owned())
    }else{
        let full_path_dir = full_path.as_path().parent().unwrap();
        match fs::create_dir_all(full_path_dir) {
            Ok(_) => {},
            Err(_) => {
                return Err("An error occured while creating necessary parent directories".to_owned());
            }
        }
    }

    let trans_content = transform::transform_entry(enc_params, content);
    match fs::write(full_path, trans_content) {
        Ok(_) => {},
        Err(_) => {
            return Err("An error occured while writing the content to the file".to_owned());
        }
    }

    return Ok(())
}

fn show_entry(prefix: &path::Path, p: &path::Path, enc_params: &transform::EncryptionParams) -> Result<String, String> {
    let trans_path = transform::transform_path(enc_params, p.to_str().unwrap()).join("/");
    let full_path = prefix.join(trans_path);

    if full_path.is_dir() {
        return Err("Is dir".to_owned());
    }

    let exists = match fs::metadata(full_path.as_path()) {
        Ok(_) => true,
        Err(_) => false,
    };

    if !exists {
        return Err("Entry does not exist".to_owned());
    }

    let res = match fs::read(full_path) {
        Ok(r) => r,
        Err(_) => {
            return Err("An error occured while reading the entry from the file".to_owned());
        }
    };


    let content = str::from_utf8(res.as_slice()).unwrap().to_owned();
    let clear_content = transform::retransform_entry(enc_params, content.as_str());

    return Ok(clear_content);
}

enum TreeNode {
    Node(String, Vec<TreeNode>),
    Leaf(String),
}

fn get_tree_from_path(p: &path::Path, is_clear: bool, enc_params: &transform::EncryptionParams) -> Result<TreeNode, String> {
    if p.is_file() {
        let filename = p.file_name().unwrap().to_str().unwrap();
        return Ok(TreeNode::Leaf(transform::retransform_entry(enc_params, filename)));
    }

    let it = match fs::read_dir(p) {
        Ok(iter) => iter,
        Err(_) => return Err("Couldnt read directory".to_owned()),
    };

    let mut result = Vec::new();

    for entry in it {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => return Err("Conversion error. Not UTF-8?".to_owned()),
        };

    
        let entryp = &entry.path();
        
        match get_tree_from_path(entryp, false, enc_params) {
            Ok(node) => {
                result.push(node);
                },
            Err(e) => {
                return Err(e);
            },
        }
    }

    let filename = p.file_name().unwrap().to_str().unwrap();

    let dirname = if !is_clear {
        transform::retransform_entry(enc_params, filename)
    }else{
        filename.to_owned()
    };
    return Ok(TreeNode::Node(dirname.to_owned(), result));
}

fn get_all_entries_in_path(p: &path::Path) -> Result<Vec<(String,bool)>, String> {
    let it = match fs::read_dir(p) {
        Ok(iter) => iter,
        Err(_) => return Err("Couldnt read directory".to_owned()),
    };

    let mut result = Vec::new();

    for entry in it {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => return Err("Conversion error. Not UTF-8?".to_owned()),
        };

        let dirp = &entry.path();
        let x = match path::Path::new(dirp).file_name(){
            Some(s) => match s.to_owned().to_str() {
                Some(s) => s.to_owned(),
                None => return Err("Conversion error. Not UTF-8?".to_owned()),
            },
            None => return Err("No Filename? Empty path?".to_owned()),
        };
        result.push((x, entry.path().is_dir()));
    }

    return Ok(result);
}