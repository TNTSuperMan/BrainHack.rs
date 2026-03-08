mod ctx;
mod stmt;
mod expr;

use anyhow::Result;

use crate::{asm::asm::AssemblyProgram, compile::{ctx::CompileContext, stmt::compile_stmts}, ir::ir::IR};

pub fn compile(ir: &IR) -> Result<AssemblyProgram> {
    let mut ctx = CompileContext::new();

    let code = compile_stmts(&mut ctx, &ir.funcs, &ir.main)?;

    Ok(AssemblyProgram {
        static_memory_size: ctx.max_usage,
        dynamic_memory_block_size: 0,
        code,
    })
}
