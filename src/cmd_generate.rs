use crate::util::{Options, prepare_entry_path, add_entry};
use crate::transform;
use crate::generate;

use std::path;

pub fn cmd_generate(opts: &Options, prefix: &path::Path , enc_params: &transform::EncryptionParams) {
    if opts.args.len() > 2 || opts.args.len() < 1 {
        println!("Incorrect number of arguments. Want: 'path, [length]'  Got: {}", opts.args.len());
        return;
    }

    let relative_path = prepare_entry_path(opts.args[0].as_str());
    
    let passwd = if opts.args.len() == 2 {
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