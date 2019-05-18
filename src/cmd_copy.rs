use crate::util::{Options, prepare_entry_path};
use crate::transform;

use std::path;
use std::fs;

pub fn cmd_copy(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
    if opts.args.len() != 2 {
        println!("Incorrect number of arguments. Want: 'old_path, new_path'  Got: {}", opts.args.len());
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