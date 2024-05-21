use std::io::{Read};
use std::io;

const ARRAY_SIZE: usize = 30_000;

fn add(a: char, b: i32) -> char {
    return (a as i32 + b) as u8 as char;
}

pub struct Runtime {
    ptr: i32,
    data: [i64; ARRAY_SIZE],
}


impl Runtime {
    pub fn new() -> Self {
        return Runtime {
            ptr: 0,
            data: [0; ARRAY_SIZE],
        };
    }

    pub fn increment_ptr(&mut self) -> Result<(), String> {
        self.ptr += 1;
        if self.ptr >= ARRAY_SIZE as i32 {
            return Err("There is a buffer overflow".to_string());
        }
        return Ok(());
    }

    pub fn decrement_ptr(&mut self) -> Result<(), String> {
        self.ptr -= 1;
        if self.ptr < 0 {
            return Err("There is a buffer underflow".to_string());
        }
        return Ok(());
    }

    pub fn increment_value(&mut self) {
        self.data[self.ptr as usize] += 1;
    }

    pub fn decrement_value(&mut self) {
        self.data[self.ptr as usize] -= 1;
    }

    pub fn put_char(&mut self) {
        let i64 = self.data[self.ptr as usize];
        let u8 = i64 as u8;
        let char = u8 as char;
        println!("{}, {}, '{}'", i64, u8, char);
    }

    pub fn get_char(&mut self) -> Result<(), String> {
        let mut buffer = [0; 1];
        let read_result = io::stdin().read(&mut buffer);
        return match read_result {
            Err(e) => {
                Err(e.to_string())
            }
            Ok(red) => {
                if red == 0 {
                    return Err("Not enough bytes in stdin red".to_string());
                }
                self.data[self.ptr as usize] = buffer[0] as i64;
                Ok(())
            }
        };
    }

    pub fn jump_to_next_bracket(&mut self) -> bool {
        let current_value = self.data[self.ptr as usize];
        return current_value == 0;
    }

    pub fn jump_to_previous_bracket(&mut self) -> bool {
        let current_value = self.data[self.ptr as usize];
        return current_value != 0;
    }
}
