use std::io;

const ARRAY_SIZE: usize = 30_000;

pub struct Runtime {
    pub(crate) ptr: i32,
    pub(crate) data: [i64; ARRAY_SIZE],
    pub(crate) max_ptr: usize,

    pub(crate) read_buffer: Vec<u8>,
    pub(crate) read_cursor: usize,
}


impl<'a> Runtime {
    pub fn new() -> Self {
        return Runtime {
            ptr: 0,
            data: [0; ARRAY_SIZE],
            max_ptr: 0,
            read_buffer: Vec::new(),
            read_cursor: 1,
        };
    }

    // pub fn slice(self) -> &'a [i64] {
    //     return &(self.data[0..self.max_ptr]);
    // }

    pub fn increment_ptr(&mut self) -> Result<(), String> {
        self.ptr += 1;

        if self.ptr >= ARRAY_SIZE as i32 {
            return Err("There is a buffer overflow".to_string());
        }

        if self.max_ptr < self.ptr as usize {
            self.max_ptr = self.ptr as usize;
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
        print!("{}", self.data[self.ptr as usize] as u8 as char);
    }

    pub fn get_char(&mut self) -> Result<(), String> {
        if self.read_cursor > self.read_buffer.len() {
            let mut buffer = String::new();
            let result = io::stdin().read_line(&mut buffer);
            if result.is_err() {
                return Err(result.err().unwrap().to_string());
            }
            let mut vec = buffer.as_bytes().to_vec();
            vec.push(0);
            self.read_buffer = vec;
            self.read_cursor = 0;
        }
        
        let char = self.read_buffer[self.read_cursor];
        self.read_cursor += 1;
        self.data[self.ptr as usize] = char as i64;
        
        return Ok(());
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

#[cfg(test)]
mod tests {
    use crate::runtime::Runtime;

    #[test]
    fn increment_value() {
        let runtime = &mut Runtime::new();
        runtime.increment_value();
        runtime.increment_value();
        runtime.increment_value();

        let result_inc = runtime.increment_ptr();
        assert!(result_inc.is_ok());

        runtime.increment_value();

        assert_eq!(1, runtime.ptr);
        assert_eq!([3, 1], runtime.data[0..runtime.max_ptr + 1]);
    }

    #[test]
    fn increment_decrement_value() {
        let runtime = &mut Runtime::new();
        runtime.increment_value();
        runtime.increment_value();
        runtime.increment_value();
        runtime.decrement_value();

        let result_inc = runtime.increment_ptr();
        assert!(result_inc.is_ok());

        runtime.increment_value();

        let result_inc = runtime.increment_ptr();
        assert!(result_inc.is_ok());

        runtime.decrement_value();

        assert_eq!(2, runtime.ptr);
        assert_eq!([2, 1, -1], runtime.data[0..runtime.max_ptr + 1]);
    }

    #[test]
    fn all_without_put_get() {
        let runtime = &mut Runtime::new();

        runtime.increment_value();
        runtime.increment_value();
        runtime.increment_value();

        assert!(runtime.increment_ptr().is_ok());
        assert!(runtime.increment_ptr().is_ok());

        runtime.decrement_value();
        runtime.decrement_value();

        // [3, 0, -2]
        //        ^

        assert!(runtime.decrement_ptr().is_ok());
        runtime.decrement_value();

        // [3, -1, -2]
        //     ^

        assert!(runtime.decrement_ptr().is_ok());
        runtime.decrement_value();

        // [2, -1, -2]
        //  ^

        assert!(runtime.increment_ptr().is_ok());
        assert!(runtime.increment_ptr().is_ok());
        assert!(runtime.increment_ptr().is_ok());

        // [2, -1, -2, 0]
        //             ^

        runtime.increment_value();
        runtime.increment_value();
        runtime.increment_value();
        runtime.increment_value();

        // [2, -1, -2, 4]
        //             ^

        assert!(runtime.decrement_ptr().is_ok());
        assert!(runtime.decrement_ptr().is_ok());

        assert_eq!(1, runtime.ptr);
        assert_eq!([2, -1, -2, 4], runtime.data[0..runtime.max_ptr + 1]);
    }
}
