use crate::asm::utils::repeat;

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
        let delta = (at as isize) - (self.pointer as isize);
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
