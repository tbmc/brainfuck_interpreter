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

    let stdin = &mut io::stdin().lock();
    let stdout = &mut io::stdout();
    let runtime = &mut Runtime::new(Box::new(stdin), Box::new(stdout));
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

        let stdin = &mut io::stdin().lock();
        let stdout = &mut io::stdout();
        let runtime = &mut Runtime::new(Box::new(stdin), Box::new(stdout));
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

        let stdin = &mut io::stdin().lock();
        let stdout = &mut io::stdout();
        let runtime = &mut Runtime::new(Box::new(stdin), Box::new(stdout));
        let result = execute_code(runtime, ast, 0);

        assert!(result.is_ok());
        assert_eq!(2, runtime.ptr);
        assert_eq!([2, 254, 1], &runtime.data[0..3]);
    }

    #[test]
    fn test_simple_loop() {
        let code = "+++[-]+";
        let parsed = parse_code(code);
        let ast = &parsed.unwrap();

        let stdin = &mut io::stdin().lock();
        let stdout = &mut io::stdout();
        let runtime = &mut Runtime::new(Box::new(stdin), Box::new(stdout));
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

        let stdin = &mut io::stdin().lock();
        let stdout = &mut io::stdout();
        let runtime = &mut Runtime::new(Box::new(stdin), Box::new(stdout));
        runtime.data[0] = 2;
        let result = execute_code(runtime, ast, 1);
        assert!(result.is_ok());
        assert_eq!(1, runtime.data[0]);
    }
}

#[cfg(test)]
mod test_scripts {
    use std::{fs, io};
    use log::LevelFilter;
    use crate::abstract_syntax_tree::parse_code;
    use crate::interpreter::execute_code;
    use crate::runtime::Runtime;
    use crate::SCRIPT_FOLDER;
    use crate::syntax_checker::syntax_check;

    static mut ALREADY_INITIALIZED: bool = false;

    fn init() {
        unsafe {
            if ALREADY_INITIALIZED {
                return;
            }

            ALREADY_INITIALIZED = true;
        }

        env_logger::Builder::new().filter_level(LevelFilter::Debug).init();
    }

    fn execute_code_for_test(runtime: &mut Runtime, script_path: &str) {
        let file_content = fs::read_to_string(script_path).unwrap();
        let ast = &(parse_code(file_content.as_str()).unwrap());
        syntax_check(ast).unwrap();
        execute_code(runtime, ast, 0).unwrap();
    }


    #[test]
    fn test_copy() {
        init();

        let stdin = &mut io::stdin().lock();
        let mut stdout: Vec<u8> = Vec::new();
        let runtime = &mut Runtime::new(Box::new(stdin), Box::new(&mut stdout));
        let script_path = format!("{}/test/copy.bf", SCRIPT_FOLDER);

        execute_code_for_test(runtime, script_path.as_str());

        let str = stdout.iter().map(|x| x.clone() as char).collect::<String>();
        assert_eq!(str, "@");
    }

    #[test]
    fn test_inside_loop() {
        init();

        let stdin = &mut io::stdin().lock();
        let mut stdout: Vec<u8> = Vec::new();
        let runtime = &mut Runtime::new(Box::new(stdin), Box::new(&mut stdout));
        let script_path = format!("{}/test/inside_loop.bf", SCRIPT_FOLDER);

        execute_code_for_test(runtime, script_path.as_str());

        let str = stdout.iter().map(|x| x.clone() as char).collect::<String>();
        assert_eq!(str, "@");
    }

    #[test]
    fn test_hello_world() {
        init();

        let stdin = &mut io::stdin().lock();
        let mut stdout: Vec<u8> = Vec::new();
        let runtime = &mut Runtime::new(Box::new(stdin), Box::new(&mut stdout));
        let script_path = format!("{}/hello_world.bf", SCRIPT_FOLDER);

        execute_code_for_test(runtime, script_path.as_str());

        let str = stdout.iter().map(|x| x.clone() as char).collect::<String>();
        assert_eq!(str, "Hello World!\n");
    }

    #[test]
    fn test_cell_size() {
        init();

        let stdin = &mut io::stdin().lock();
        let mut stdout: Vec<u8> = Vec::new();
        let runtime = &mut Runtime::new(Box::new(stdin), Box::new(&mut stdout));
        let script_path = format!("{}/cell_size.bf", SCRIPT_FOLDER);

        execute_code_for_test(runtime, script_path.as_str());

        let str = stdout.iter().map(|x| x.clone() as char).collect::<String>();
        assert_eq!(str, "8 bit cells\n");
    }

    #[test]
    fn test_fibonacci() {
        init();

        let stdin = &mut io::stdin().lock();
        let mut stdout: Vec<u8> = Vec::new();
        let runtime = &mut Runtime::new(Box::new(stdin), Box::new(&mut stdout));
        let script_path = format!("{}/fibonacci.bf", SCRIPT_FOLDER);

        execute_code_for_test(runtime, script_path.as_str());

        let str = stdout.iter().map(|x| x.clone() as char).collect::<String>();
        assert_eq!(str, "1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89");
    }

    #[test]
    fn test_read_print() {
        init();
        
        let stdin = &mut io::Cursor::new(b"This is a test!\n");
        let mut stdout: Vec<u8> = Vec::new();
        let runtime = &mut Runtime::new(Box::new(stdin), Box::new(&mut stdout));
        let script_path = format!("{}/test/read_print.bf", SCRIPT_FOLDER);

        execute_code_for_test(runtime, script_path.as_str());

        let str = stdout.iter().map(|x| x.clone() as char).collect::<String>();
        assert_eq!(str, "This is a test!\n");
    }
    
    #[test]
    fn test_prime_1() {
        init();
        
        let stdin = &mut io::stdin().lock();
        let mut stdout: Vec<u8> = Vec::new();
        let runtime = &mut Runtime::new(Box::new(stdin), Box::new(&mut stdout));
        let script_path = format!("{}/prime.bf", SCRIPT_FOLDER);

        execute_code_for_test(runtime, script_path.as_str());

        let str = stdout.iter().map(|x| x.clone() as char).collect::<String>();
        assert_eq!(str, "29, 23, 19, 17, 13, 11, 7, 5, 3, 2, 1, ");
    }
}
