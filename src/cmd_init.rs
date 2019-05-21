use crate::util::Options;

pub fn cmd_init(opts: &Options) {
    if opts.args.len() != 1 {
        println!("Too many arguments. Want: '[path_to_dir]'  Got: {}", opts.args.len());
        return;
    }

    let path = std::path::Path::new(opts.args[0].as_str()); 
    match std::fs::create_dir_all(path) {
        Ok(()) => {
            println!("Created: {}",  path.to_str().unwrap());
            println!("This command is not necessary with sparkpass. The repo would have been initialzied with your first add/insert command");
        },
        Err(e) => {
            println!("There was an error while creating the repo: {}, {}", path.to_str().unwrap(), e);
        }
    }
}