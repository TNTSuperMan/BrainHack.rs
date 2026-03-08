use std::collections::HashMap;

use anyhow::{Result, bail};
use boa_ast::expression::Identifier;

use crate::{asm::asm::AssemblyOp, compile::ctx::CompileContext, ir::ir::{IRExpr, IRFunc}};

pub fn compile_expr(ctx: &mut CompileContext, funcs: &HashMap<Identifier, IRFunc>, target: usize, expr: &IRExpr) -> Result<Vec<AssemblyOp>> {
    let mut asm: Vec<AssemblyOp> = vec![];
    ctx.push();
    
    match expr {
        IRExpr::Const(c) => {
            asm.push(AssemblyOp::Set(target, *c));
        }
        IRExpr::Add(l, r) => {
            asm.append(&mut compile_expr(ctx, funcs, target, l.as_ref())?);
            
            let rp = ctx.alloc_noname();
            asm.append(&mut compile_expr(ctx, funcs, rp, r.as_ref())?);

            asm.push(AssemblyOp::Move(rp, vec![(target, 1)]));
            ctx.free(rp)?;
        }
        IRExpr::Sub(l, r) => {
            asm.append(&mut compile_expr(ctx, funcs, target, l.as_ref())?);
            
            let rp = ctx.alloc_noname();
            asm.append(&mut compile_expr(ctx, funcs, rp, r.as_ref())?);

            asm.push(AssemblyOp::Move(rp, vec![(target, -1)]));
            ctx.free(rp)?;
        }

        IRExpr::Id { id, last_use: _ } => {
            let ptr = ctx.get(*id)?;
            let tmp = ctx.alloc_noname();
            asm.push(AssemblyOp::Move(ptr, vec![(tmp, 1)]));
            asm.push(AssemblyOp::Move(tmp, vec![(ptr, 1), (target, 1)]));
            ctx.free(tmp)?;
        }

        _ => bail!("todo")
    }
    
    ctx.pop();
    Ok(asm)
}
