use std::collections::HashMap;

use anyhow::{Result, anyhow, bail};
use boa_interner::Sym;

use crate::{asm::asm::AsmOp, compile::{ctx::CompileContext, stmt::compile_stmts}, ir::ir::{IRExpr, IRFunc}};

pub fn compile_expr(ctx: &mut CompileContext, funcs: &HashMap<Sym, IRFunc>, target: usize, expr: &IRExpr) -> Result<Vec<AsmOp>> {
    let mut asm: Vec<AsmOp> = vec![];
    ctx.push();

    macro_rules! expr {
        ($target: expr, $expr: expr) => {
            asm.append(&mut compile_expr(ctx, funcs, $target, $expr)?);
        };
    }
    
    match expr {
        IRExpr::Const(c) => {
            asm.push(AsmOp::Set(target, *c));
        }
        IRExpr::Add(l, r) => {
            expr!(target, l.as_ref());
            
            let rp = ctx.alloc_noname();
            expr!(rp, r.as_ref());

            asm.push(AsmOp::Move(rp, vec![(target, 1)]));
            ctx.free(rp)?;
        }
        IRExpr::Sub(l, r) => {
            expr!(target, l.as_ref());
            
            let rp = ctx.alloc_noname();
            expr!(rp, r.as_ref());

            asm.push(AsmOp::Move(rp, vec![(target, -1)]));
            ctx.free(rp)?;
        }
        IRExpr::Mul(l, r) => {
            match (l.as_ref(), r.as_ref()) {
                (IRExpr::Const(nl), IRExpr::Const(nr)) => {
                    asm.push(AsmOp::Set(target, nl.wrapping_mul(*nr)))
                }
                (IRExpr::Const(n), exp) |
                (exp, IRExpr::Const(n)) => {
                    let tmp = ctx.alloc_noname();
                    asm.append(&mut compile_expr(ctx, funcs, tmp, exp)?);
                    asm.push(AsmOp::Move(tmp, vec![(target, *n)]));
                    ctx.free(tmp)?;
                }
                (expl, expr) => {
                    let tmp_l = ctx.alloc_noname();
                    let tmp_r = ctx.alloc_noname();
                    let tmp = ctx.alloc_noname();

                    asm.push(AsmOp::Set(target, 0));
                    expr!(tmp_l, expl);
                    expr!(tmp_r, expr);
                    asm.push(AsmOp::Set(tmp, 0));

                    let mut loop_body: Vec<AsmOp> = vec![];
                    loop_body.push(AsmOp::Move(tmp_l, vec![(tmp, 1)]));
                    loop_body.push(AsmOp::Move(tmp, vec![(tmp_l, 1), (target, 1)]));
                    loop_body.push(AsmOp::Add(tmp_r, -1));

                    asm.push(AsmOp::Loop(tmp_r, loop_body));
                }
            }
        }

        IRExpr::Id { id, last_use: _ } => {
            let ptr = ctx.get(*id)?;
            let tmp = ctx.alloc_noname();
            asm.push(AsmOp::Move(ptr, vec![(tmp, 1)]));
            asm.push(AsmOp::Move(tmp, vec![(ptr, 1), (target, 1)]));
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
            ctx.callstack.pop();
        }
        
        IRExpr::Input => {
            asm.push(AsmOp::In(target));
        }
        
        IRExpr::BoolNot(exp) => {
            let tmp = ctx.alloc_noname();
            asm.push(AsmOp::Set(target, 1));
            expr!(tmp, exp.as_ref());
            asm.push(AsmOp::Loop(tmp, vec![
                AsmOp::Set(target, 0),
                AsmOp::Set(tmp, 0),
            ]));
            ctx.free(tmp)?;
        }
        IRExpr::Boolify(exp) => {
            let tmp = ctx.alloc_noname();
            asm.push(AsmOp::Set(target, 0));
            expr!(tmp, exp.as_ref());
            asm.push(AsmOp::Loop(tmp, vec![
                AsmOp::Set(target, 1),
                AsmOp::Set(tmp, 0),
            ]));
            ctx.free(tmp)?;
        }
        IRExpr::Gt(expl, expr) => {
            let l = ctx.alloc_noname();
            let r = ctx.alloc_noname();
            let tmp0 = ctx.alloc_noname();
            let tmp1 = ctx.alloc_noname();

            expr!(l, expl.as_ref());
            expr!(r, expr.as_ref());
            asm.push(AsmOp::Set(tmp0, 0));
            asm.push(AsmOp::Set(tmp1, 0));
            asm.push(AsmOp::Set(target, 0));
            asm.push(AsmOp::Loop(l, vec![
                AsmOp::Add(tmp0, 1),
                AsmOp::Loop(r, vec![
                    AsmOp::Set(tmp0, 0),
                    AsmOp::Add(tmp1, 1),
                    AsmOp::Add(r, -1),
                ]),
                AsmOp::Move(tmp0, vec![(target, 1)]),
                AsmOp::Move(tmp1, vec![(r, 1)]),
                AsmOp::Add(r, -1),
                AsmOp::Add(l, -1),
            ]));

            ctx.free(tmp1)?;
            ctx.free(tmp0)?;
            ctx.free(r)?;
            ctx.free(l)?;
        }

        _ => bail!("todo")
    }
    
    ctx.pop();
    Ok(asm)
}
