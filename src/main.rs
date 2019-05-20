
//main.rs deals with preparing the options and arguments to pass to the cmd_* functions

extern crate shellexpand;
extern crate argparse;
use argparse::{ArgumentParser, Store, StoreTrue, StoreFalse, Collect};
extern crate rpassword;

use std::path;
use openssl::sha::sha256;

// internal imports
mod transform;
mod generate;
mod util;

mod cmd_add;
use cmd_add::cmd_add;
mod cmd_copy;
use cmd_copy::cmd_copy;
mod cmd_generate;
use cmd_generate::cmd_generate;
mod cmd_list;
use cmd_list::cmd_list_tree;
mod cmd_move;
use cmd_move::cmd_move;
mod cmd_remove;
use cmd_remove::cmd_remove;
mod cmd_search;
use cmd_search::cmd_search_fuzzy;
mod cmd_show;
use cmd_show::cmd_show;

mod export_import;
use export_import::{cmd_import, cmd_export};

use util::Options;



fn read_key_from_terminal() -> String {
    println!("Enter key to repo (it is recommended to use SPARKPASS_KEY instead of interactive entering): ");
    let pass = rpassword::read_password().unwrap();
    return pass.to_owned();
}

fn main() {
    let mut options = Options {
        args: std::vec::Vec::new(),
        key: String::new(),
        repo: String::new(),
        verbose: false,
        recursive: false,
        force: false,
        line: 0,
        show_tree: true,
        interactive: true,
        multiline: false,
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

        ap.refer(&mut options.interactive)
            .add_option(&["--interactiveoff", "-i"], StoreFalse,
            "Don't ask for key if not found in argument/SPAKRPASS_KEY");

         ap.refer(&mut options.show_tree)
            .add_option(&["--treeoff", "-t"], StoreFalse,
            "Show output of search/list as flat list of entries");

        ap.refer(&mut options.recursive)
            .add_option(&["--recursive", "-r"], StoreTrue,
            "Remove contents of directories");

        ap.refer(&mut options.repo)
            .add_option(&["--repo", "-p"], Store,
            "Path to the repo where your keys are");

        ap.refer(&mut options.line)
            .add_option(&["--line", "-l"], Store,
            "Specify which line of multiline file you want to show. If set to -1 all lines will be printed. Default is line 0.");

        ap.refer(&mut options.multiline)
            .add_option(&["--multiline", "-m"], StoreTrue,
            "Add a new multiline content");

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
        if !options.interactive {
            println!("No key given and interactive mode deactivated");
            return;
        }
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
            cmd_search_fuzzy(&options, repopath, &enc_params);
        },
        "search" => {
            cmd_search_fuzzy(&options, repopath, &enc_params);
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

        ////// Commands special to sparkpass
        
        "import" => {
            cmd_import(&options, repopath, &enc_params);
        }

        "export" => {
            cmd_export(&options, repopath, &enc_params);
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

