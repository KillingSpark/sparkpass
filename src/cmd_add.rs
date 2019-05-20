extern crate rpassword;

use crate::transform;
use crate::util::{add_entry, prepare_entry_path, Options};

use std::io::{self, BufRead, BufReader, Read};

use std::path;

fn read_password_from_terminal() -> String {
    rpassword::read_password().unwrap().to_owned()
}

fn read_multiline<T: Read>(reader: T) -> String {
    BufReader::new(reader)
        .lines()
        .map(|x| match x {
            Ok(y) => y,
            Err(_) => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}
pub fn cmd_add(opts: &Options, prefix: &path::Path, enc_params: &transform::EncryptionParams) {
    if opts.args.len() < 1 || opts.args.len() > 2 {
        println!(
            "Incorrect number of arguments. Want: 'path_new, [content]'  Got: {}",
            opts.args.len()
        );
        return;
    }

    let relative_path = prepare_entry_path(opts.args[0].as_str());
    let pwd = if opts.args.len() == 1 {
        if !opts.interactive {
            println!("No key given and interactive mode deactivated");
            return;
        }

        if opts.verbose {
            println!("Only one argument was given, requesting the password via interactive input");
        }

        if opts.multiline {
            if opts.verbose {
                println!("Multiline is set, requesting multiline content");
            }
            println!("Enter multiline content for {}: ", opts.args[0].to_string());
            read_multiline(io::stdin())
        } else {
            print!("Enter content for {}: ", opts.args[0].to_string());
            read_password_from_terminal()
        }
    } else {
        opts.args[1].to_string()
    };

    if opts.verbose {
        println!("Adding Entry: {}", relative_path);
    }

    match add_entry(
        prefix,
        path::Path::new(relative_path),
        pwd.as_str(),
        opts.force,
        enc_params,
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("An error occurred while adding the entry: {}", e);
            return;
        }
    }
}
