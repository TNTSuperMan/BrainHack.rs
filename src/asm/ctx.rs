use crate::asm::{ADDR_PTR, FETCH_VAL_PTR, SEND_VAL_PTR, utils::repeat};

pub struct AssembleContext {
    pub pointer: usize,
    pub code: String,

    static_memory_size: usize,
    dynamic_memory_block_size: usize,
}

impl AssembleContext {
    pub fn new(static_memory_size: usize, dynamic_memory_block_size: usize) -> AssembleContext {
        AssembleContext {
            pointer: 0,
            code: String::new(),
            static_memory_size,
            dynamic_memory_block_size
        }
    }

    pub fn go(&mut self, at: usize) {
        let real_at = match at {
            ADDR_PTR => self.st() + self.dy() + 2,
            FETCH_VAL_PTR => self.st() + self.dy() + 2 + 1,
            SEND_VAL_PTR => self.st() + (self.dy() + 2) * 2 + 1,
            _ => at,
        };
        let delta = (real_at as isize) - (self.pointer as isize);
        self.push(&repeat(delta, ">", "<"));
        self.pointer = at;
    }
    pub fn add(&mut self, val: i8) {
        self.push(&repeat(val as isize, "+", "-"));
    }
    pub fn push(&mut self, str: &str) {
        self.code.push_str(str);
    }
    
    pub fn st(&self) -> usize {
        self.static_memory_size
    }
    pub fn dy(&self) -> usize {
        self.dynamic_memory_block_size
    }
}
