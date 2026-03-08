use std::collections::HashMap;

use anyhow::{Result, bail};
use boa_ast::expression::Identifier;

use crate::{asm::asm::AssemblyOp, compile::ctx::CompileContext, ir::ir::{IRExpr, IRFunc}};

pub fn compile_expr(ctx: &mut CompileContext, funcs: &HashMap<Identifier, IRFunc>, target: usize, expr: &IRExpr) -> Result<Vec<AssemblyOp>> {
    let mut asm: Vec<AssemblyOp> = vec![];
    ctx.push();
    // TODO
    bail!("unimplemented");
    
    ctx.pop();
    Ok(asm)
}
