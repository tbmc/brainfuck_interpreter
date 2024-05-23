use std::io;

const ARRAY_SIZE: usize = 30_000;

type BoxType = u8;

pub struct Runtime<'a> {
    pub(crate) ptr: i32,
    pub(crate) data: [BoxType; ARRAY_SIZE],
    pub(crate) max_ptr: usize,
    pub(crate) instruction_counter: usize,

    pub(crate) read_buffer: Vec<u8>,
    pub(crate) read_cursor: usize,

    pub(crate) stdin: &'a mut dyn BufRead,
    pub(crate) stdout: &'a mut dyn Write,
}


impl<'a> Runtime<'a> {
    pub fn new(stdin: &'a mut dyn BufRead, stdout: &'a mut dyn Write) -> Self {
        Runtime {
            ptr: 0,
            data: [0; ARRAY_SIZE],
            max_ptr: 0,
            instruction_counter: 0,

            read_buffer: Vec::new(),
            read_cursor: 1,
            stdin,
            stdout,
        }
    }

    pub fn extract_data(&self) -> Vec<BoxType> {
        let mut vec = Vec::with_capacity(self.max_ptr + 1);
        for i in 0..self.max_ptr {
            vec.push(self.data[i]);
        }
        vec
    }

    pub fn dump_data(&self, exit: bool) {
        let data: String = self.extract_data().iter().map(|x| format!("{} ", x)).collect();
        println!("Data:\nCurrent pointer: {}\n{}\n", self.ptr, data);
        if exit {
            std::process::exit(0);
        }
    }

    fn increment_instruction_counter(&mut self) {
        self.instruction_counter += 1;

        if self.instruction_counter > MAX_INSTRUCTIONS {
            self.dump_data(true);
            panic!("May be there is an infinite loop. Max of {} instructions exceeded.", MAX_INSTRUCTIONS);
        }
    }

    pub fn increment_ptr(&mut self) -> Result<(), String> {
        self.ptr += 1;

        if self.ptr >= ARRAY_SIZE as i32 {
            return Err("There is a buffer overflow".to_string());
        }

        if self.max_ptr < self.ptr as usize {
            self.max_ptr = self.ptr as usize;
        }

        self.increment_instruction_counter();
        Ok(())
    }

    pub fn decrement_ptr(&mut self) -> Result<(), String> {
        self.ptr -= 1;
        if self.ptr < 0 {
            return Err("There is a buffer underflow".to_string());
        }

        self.increment_instruction_counter();
        Ok(())
    }

    pub fn increment_value(&mut self) {
        self.data[self.ptr as usize] = self.data[self.ptr as usize].wrapping_add(1);
        self.increment_instruction_counter();
    }

    pub fn decrement_value(&mut self) {
        self.data[self.ptr as usize] = self.data[self.ptr as usize].wrapping_sub(1);
        self.increment_instruction_counter();
    }

    pub fn put_char(&mut self) {
        let char = self.data[self.ptr as usize] as char;
        write!(&mut self.stdout, "{}", char).unwrap();
        self.stdout.flush().unwrap();
        self.increment_instruction_counter();
    }

    pub fn get_char(&mut self) -> Result<(), String> {
        if self.read_cursor > self.read_buffer.len() {
            let mut buffer = String::new();
            
            let result = self.stdin.read_line(&mut buffer);
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
        self.data[self.ptr as usize] = char as BoxType;

        self.increment_instruction_counter();
        Ok(())
    }

    pub fn jump_to_next_bracket(&mut self) -> bool {
        self.increment_instruction_counter();
        let current_value = self.data[self.ptr as usize];
        current_value == 0
    }

    pub fn jump_to_previous_bracket(&mut self) -> bool {
        self.increment_instruction_counter();
        let current_value = self.data[self.ptr as usize];
        current_value != 0
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use crate::runtime::Runtime;

    #[test]
    fn increment_value() {
        let stdin = &mut io::stdin().lock();
        let stdout = &mut io::stdout();
        let runtime = &mut Runtime::new(stdin, stdout);
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
        let stdin = &mut io::stdin().lock();
        let stdout = &mut io::stdout();
        let runtime = &mut Runtime::new(stdin, stdout);
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
        assert_eq!([2, 1, 255], runtime.data[0..runtime.max_ptr + 1]);
    }

    #[test]
    fn all_without_put_get() {
        let stdin = &mut io::stdin().lock();
        let stdout = &mut io::stdout();
        let runtime = &mut Runtime::new(stdin, stdout);

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
        assert_eq!([2, 255, 254, 4], runtime.data[0..runtime.max_ptr + 1]);
    }
}
