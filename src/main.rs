use log::{info, LevelFilter};
use crate::consts::SCRIPT_FOLDER;
use crate::interpreter::interpret_script_file;

mod interpreter;
mod runtime;
mod abstract_syntax_tree;
mod syntax_checker;
mod consts;


fn main() {
    env_logger::Builder::new().filter_level(LevelFilter::Info).init();

    info!("Execute BrainFuck script!");
    let result = interpret_script_file(format!("{}/prime_2.bf", SCRIPT_FOLDER).as_str());
    match result {
        Ok(_) => {
            info!("Success!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
