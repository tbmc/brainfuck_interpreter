use log::info;
use crate::interpreter::interpret_script_file;

mod interpreter;
mod runtime;
mod abstract_syntax_tree;
mod syntax_checker;

fn main() {
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
