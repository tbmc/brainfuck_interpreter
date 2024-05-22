use log::{info, LevelFilter};
use crate::interpreter::interpret_script_file;

mod interpreter;
mod runtime;
mod abstract_syntax_tree;
mod syntax_checker;

fn main() {
    env_logger::Builder::new().filter_level(LevelFilter::Info).init();

    info!("Execute BrainFuck script!");
    let result = interpret_script_file("brain_fuck_scripts/prime_2.bf");
    match result {
        Ok(_) => {
            info!("Success!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
