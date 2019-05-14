use crate::util::{Options, prepare_entry_path, add_entry};
use crate::transform;

use std::path;

pub fn cmd_add(opts: &Options, prefix: &path::Path, enc_params: &transform::EncryptionParams) {
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