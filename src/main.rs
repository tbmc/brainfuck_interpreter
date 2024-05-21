use crate::interpreter::interpret_script_file;

mod interpreter;
mod runtime;
mod abstract_syntax_tree;
mod syntax_checker;

fn main() {
    println!("Execute BrainFuck script!");
    let result = interpret_script_file("hello_world.bf");
    match result {
        Ok(_) => {
            println!("Success!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
