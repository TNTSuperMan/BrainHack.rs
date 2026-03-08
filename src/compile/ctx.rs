use std::collections::HashMap;

use anyhow::{Result, bail};
use boa_ast::expression::Identifier;

pub struct CompileContext {
    pub usage: usize,
    pub max_usage: usize,
    pub callstack: Vec<Identifier>,
    pub var_map: Vec<(usize, HashMap<Identifier, usize>)>,
}
impl CompileContext {
    pub fn new() -> CompileContext {
        CompileContext {
            usage: 0,
            max_usage: 0,
            callstack: vec![],
            var_map: vec![],
        }
    }
    pub fn push(&mut self) {
        self.var_map.push((self.usage, HashMap::new()));
    }
    pub fn alloc(&mut self, id: Identifier) -> usize {
        let ptr = self.alloc_noname();
        self.var_map.last_mut().unwrap().1.insert(id, ptr);
        ptr
    }
    pub fn alloc_noname(&mut self) -> usize {
        let ptr = self.usage;
        self.usage += 1;
        self.max_usage = self.max_usage.max(self.usage);
        ptr
    }
    pub fn free(&mut self, ptr: usize) -> Result<()> {
        if self.usage - 1 != ptr {
            bail!("Internal: Free mismatch");
        }
        self.usage -= 1;
        Ok(())
    }
    pub fn get(&self, id: Identifier) -> Result<usize> {
        for map in self.var_map.iter().rev() {
            if let Some(p) = map.1.get(&id) {
                return Ok(*p);
            }
        }
        bail!("Undefined variable detected");
    }
    pub fn pop(&mut self) {
        let (usage, _) = self.var_map.pop().unwrap();
        self.usage = usage;
    }
}
