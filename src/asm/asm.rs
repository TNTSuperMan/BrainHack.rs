use crate::asm::{block::assemble_block, ctx::AssembleContext};

type Pointer = usize;

pub enum AssemblyOp {
    Move(Pointer, Vec<(Pointer, i8)>),
    Set(Pointer, i8),
    Add(Pointer, i8),
    Out(Pointer),
    In(Pointer),
    Loop(Pointer, Vec<AssemblyOp>),
    Fetch(usize),
    Send(usize),
}

pub struct AssemblyProgram {
    pub static_memory_size: usize,
    pub dynamic_memory_block_size: usize,
    pub code: Vec<AssemblyOp>,
}

impl AssemblyProgram {
    pub fn assemble(&self) -> String {
        let mut ctx = AssembleContext::new(self.static_memory_size, self.dynamic_memory_block_size);

        assemble_block(&mut ctx, &self.code);

        ctx.code
    }
}
