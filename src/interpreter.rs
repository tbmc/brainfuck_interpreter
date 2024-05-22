use std::fs;
use log::{debug, info};
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
    info!("Parsed ok.");

    let checked = syntax_check(ast);
    if checked.is_err() {
        return Err(checked.err().unwrap());
    }
    info!("Syntax check ok.");

    let runtime = &mut Runtime::new();
    info!("Program output:");
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

    let mut sub_index = 0i64;
    while (sub_index as usize) < sub_indexes.len() {
        let node_index = sub_indexes[sub_index as usize];
        let node = ast.data.get(node_index).unwrap();

        if node.is_leaf == false {
            // Branch opening
            if runtime.jump_to_next_bracket() {
                debug!("Jump to next");
                sub_index += 1;
            } else {
                // Execute inner loop
                let result = execute_code(runtime, ast, node_index);
                if result.is_err() {
                    return Err(result.err().unwrap());
                }
            }
        } else if node.char == ']' {
            // Branch closing
            if runtime.jump_to_previous_bracket() {
                // Go back to start of loop
                debug!("Jump to previous");
                sub_index -= 2;
            } else {
                // Do nothing
                debug!("Do not jump to previous");
            }
        } else {
            // Leaf
            let result = execute_leaf(runtime, ast, node_index);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
        }

        sub_index += 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_operators_3_1() {
        let code = "+++ > +";
        let parsed = parse_code(code);
        let ast = &parsed.unwrap();

        let runtime = &mut Runtime::new();
        let result = execute_code(runtime, ast, 0);

        assert!(result.is_ok());
        assert_eq!(1, runtime.ptr);
        assert_eq!([3, 1], &runtime.data[0..2]);
    }

    #[test]
    fn test_simple_operators_2_minus2_1() {
        let code = "+++ > -- < - >> +";
        let parsed = parse_code(code);
        let ast = &parsed.unwrap();

        let runtime = &mut Runtime::new();
        let result = execute_code(runtime, ast, 0);

        assert!(result.is_ok());
        assert_eq!(2, runtime.ptr);
        assert_eq!([2, -2, 1], &runtime.data[0..3]);
    }

    #[test]
    fn test_simple_loop() {
        let code = "+++[-]+";
        let parsed = parse_code(code);
        let ast = &parsed.unwrap();

        let runtime = &mut Runtime::new();
        let result = execute_code(runtime, ast, 0);
        assert!(result.is_ok());
        assert_eq!(0, runtime.ptr);
        assert_eq!([1], runtime.data[0..1]);
    }

    #[test]
    fn test_loop_in() {
        let code = "[-]+";
        let parsed = parse_code(code);
        let ast = &parsed.unwrap();

        let runtime = &mut Runtime::new();
        runtime.data[0] = 2;
        let result = execute_code(runtime, ast, 1);
        assert!(result.is_ok());
        assert_eq!(1, runtime.data[0]);
    }
}
