use crate::util::{Options, prepare_entry_path};
use crate::transform;

use std::path;
use std::fs;

pub fn cmd_move(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
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