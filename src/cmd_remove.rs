use crate::util::{Options, prepare_entry_path};
use crate::transform;

use std::path;
use std::fs;

pub fn cmd_remove(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
    if opts.args.len() != 1 {
        println!("Incorrect number of arguments. Want: 'path'  Got: {}", opts.args.len());
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