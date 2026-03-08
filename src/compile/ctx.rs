use std::collections::HashMap;

use anyhow::{Result, bail};
use boa_ast::expression::Identifier;

use crate::ir::ir::IRFunc;

pub struct CompileContext<'a> {
    pub usage: usize,
    pub max_usage: usize,
    pub callstack: Vec<Identifier>,
    pub var_map: Vec<(usize, HashMap<Identifier, usize>)>,
    pub funcs: &'a HashMap<Identifier, IRFunc>,
}
impl<'a> CompileContext<'a> {
    pub fn new(funcs: &'a HashMap<Identifier, IRFunc>) -> CompileContext<'a> {
        CompileContext {
            usage: 0,
            max_usage: 0,
            callstack: vec![],
            var_map: vec![],
            funcs,
        }
    }
    pub fn push(&mut self) {
        self.var_map.push((self.usage, HashMap::new()));
    }
    pub fn alloc(&mut self, id: Identifier) {
        self.var_map.last_mut().unwrap().1.insert(id, self.usage);
        self.usage += 1;
        self.max_usage = self.max_usage.max(self.usage);
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
