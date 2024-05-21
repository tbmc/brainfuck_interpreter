use std::fs;
use crate::runtime::Runtime;

pub fn interpret_script_file(filename: &str) {
    let content = fs::read_to_string(filename).expect("This should not fail! May be file does not exist!");
    interpret_code(content.as_str());
}

pub fn interpret_code(script: &str) {
    let mut interpreter = Runtime::new();
}
