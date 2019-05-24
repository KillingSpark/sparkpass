use crate::util::{Options, prepare_entry_path, show_entry};
use crate::transform;
use crate::cmd::cmd_search::cmd_search;

use std::path;

pub fn cmd_show(opts: &Options, prefix: &path::Path, enc_params: &transform::EncryptionParams) {
    if opts.args.len() != 1 {
        println!("Incorrect number of arguments. Want: 'path_to_entry'  Got: {}", opts.args.len());
        return;
    }

    let relative_path = prepare_entry_path(opts.args[0].as_str());
    let mut content = match show_entry(prefix, path::Path::new(relative_path), enc_params) {
        Ok(c) => c,
        Err(_) => {
            //entry doesn't exist. Search for it instead
            cmd_search(opts, prefix, enc_params);
            return;
        },
    };

    let lines: Vec<&str> = content.split("\n").collect();
    let idx: usize = opts.line as usize;
    if opts.line >= 0 && idx >= lines.len() {
        println!("Line too big. Given: {}, max line in entry: {}", idx, lines.len());
        return;
    }
    if opts.line >= 0 {
        content = lines[idx].to_owned();
    }

    if opts.verbose {println!("Showing entry: {}", relative_path);}
    if opts.verbose {
        println!("Content: {}", content);
    }else{
        print!("{}", content);
    }
}