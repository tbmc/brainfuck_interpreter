use std::env;
use std::path::Path;
use log::{info, LevelFilter};
use crate::consts::SCRIPT_FOLDER;
use crate::interpreter::interpret_script_file;

mod interpreter;
mod runtime;
mod abstract_syntax_tree;
mod syntax_checker;
mod consts;


fn main() {
    let args: Vec<String> = env::args().collect();

    env_logger::Builder::new().filter_level(LevelFilter::Info).init();

    let default_script_path = format!("{}/prime_2.bf", SCRIPT_FOLDER);
    let program_path = if args.len() > 1 {
        Path::new(args[1].as_str())
    } else {
        Path::new(default_script_path.as_str())
    };

    info!("Execute BrainFuck script!");
    let result = interpret_script_file(program_path.to_str().unwrap());
    match result {
        Ok(_) => {
            info!("Success!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
