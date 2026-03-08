use std::collections::HashMap;

use anyhow::{Result, anyhow, bail};
use boa_interner::Sym;

use crate::{asm::asm::AssemblyOp, compile::{ctx::CompileContext, stmt::compile_stmts}, ir::ir::{IRExpr, IRFunc}};

pub fn compile_expr(ctx: &mut CompileContext, funcs: &HashMap<Sym, IRFunc>, target: usize, expr: &IRExpr) -> Result<Vec<AssemblyOp>> {
    let mut asm: Vec<AssemblyOp> = vec![];
    ctx.push();

    macro_rules! expr {
        ($target: expr, $expr: expr) => {
            asm.append(&mut compile_expr(ctx, funcs, $target, $expr)?);
        };
    }
    
    match expr {
        IRExpr::Const(c) => {
            asm.push(AssemblyOp::Set(target, *c));
        }
        IRExpr::Add(l, r) => {
            expr!(target, l.as_ref());
            
            let rp = ctx.alloc_noname();
            expr!(rp, r.as_ref());

            asm.push(AssemblyOp::Move(rp, vec![(target, 1)]));
            ctx.free(rp)?;
        }
        IRExpr::Sub(l, r) => {
            expr!(target, l.as_ref());
            
            let rp = ctx.alloc_noname();
            expr!(rp, r.as_ref());

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
        IRExpr::Call { id, args } => {
            if ctx.callstack.contains(id) {
                bail!("Recursive call detected");
            }
            ctx.callstack.push(*id);
            let func = funcs.get(id).ok_or_else(|| anyhow!("Undefined function detected"))?;
            if func.args.len() != args.len() {
                bail!("Function args length mismatch");
            }
            ctx.push();

            for (i, arg) in func.args.iter().enumerate() {
                let ptr = ctx.alloc(*arg);
                expr!(ptr, &args[i]);
            }

            asm.append(&mut compile_stmts(ctx, funcs, &func.code)?);

            expr!(target, func.result.as_ref().unwrap_or_else(|| &IRExpr::Const(0)));

            ctx.pop();
        }
        
        IRExpr::Input => {
            asm.push(AssemblyOp::In(target));
        }

        _ => bail!("todo")
    }
    
    ctx.pop();
    Ok(asm)
}
