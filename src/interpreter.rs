use std::fs;
use crate::abstract_syntax_tree::{Ast, parse_code};
use crate::runtime::Runtime;
use crate::syntax_checker::syntax_check;

pub fn interpret_script_file(filename: &str) -> Result<(), String> {
    let content = fs::read_to_string(filename).expect("This should not fail! May be file does not exist!");
    return interpret_code(content.as_str());
}

pub fn interpret_code(script: &str) -> Result<(), String> {
    let parsed = parse_code(script);
    if parsed.is_err() {
        return Err(parsed.err().unwrap());
    }
    let ast = &parsed.unwrap();
    let checked = syntax_check(ast);
    if checked.is_err() {
        return Err(checked.err().unwrap());
    }

    let runtime = &mut Runtime::new();
    let result = execute_code(runtime, ast, 0);

    match result {
        Err(e) => {
            Err(e)
        }
        _ => {
            Ok(())
        }
    }
}

fn execute_code(runtime: &mut Runtime, ast: &Ast, parent_index: usize) -> Result<(), String> {
    let sub_indexes: Vec<usize>;
    {
        let parent_node = ast.data.get(parent_index).unwrap();
        sub_indexes = parent_node.sub_assets_indexes.clone();
    }

    for sub_index in sub_indexes {
        let node = ast.data.get(sub_index).unwrap();

        match node.is_leaf {
            true => {
                let result = execute_leaf(runtime, ast, sub_index);
                if result.is_err() {
                    return Err(result.err().unwrap());
                }
            }
            false => {
                let result = execute_code(runtime, ast, sub_index);
                if result.is_err() {
                    return Err(result.err().unwrap());
                }
            }
        }
    }

    Ok(())
}

fn execute_leaf(runtime: &mut Runtime, ast: &Ast, index: usize) -> Result<(), String> {
    let node = ast.data.get(index).unwrap();
    match node.char {
        '>' => {
            let result = runtime.increment_ptr();
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }
        '<' => {
            let result = runtime.decrement_ptr();
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }
        '+' => {
            runtime.increment_value();
        }
        '-' => {
            runtime.decrement_value();
        }
        '.' => {
            runtime.put_char();
        }
        ',' => {
            let result = runtime.get_char();
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }
        _ => {
            // Char is a comment, so it is ignored
        }
    }
    return Ok(());
}
