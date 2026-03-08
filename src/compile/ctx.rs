use std::collections::HashMap;

use anyhow::{Result, bail};
use boa_ast::expression::Identifier;

pub struct CompileContext {
    pub usage: usize,
    pub callstack: Vec<Identifier>,
    pub var_map: Vec<(usize, HashMap<Identifier, usize>)>,
}
impl CompileContext {
    pub fn new() -> CompileContext {
        CompileContext {
            usage: 0,
            callstack: vec![],
            var_map: vec![],
        }
    }
    pub fn push(&mut self) {
        self.var_map.push((self.usage, HashMap::new()));
    }
    pub fn alloc(&mut self, id: Identifier) {
        self.var_map.last_mut().unwrap().1.insert(id, self.usage);
        self.usage += 1;
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
