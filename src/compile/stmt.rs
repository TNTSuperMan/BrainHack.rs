use std::collections::HashMap;

use anyhow::{Result, anyhow, bail};
use boa_interner::Sym;

use crate::{asm::asm::AssemblyOp, compile::{ctx::CompileContext, expr::compile_expr}, ir::ir::{IRFunc, IRStmt}};

pub fn compile_stmts(ctx: &mut CompileContext, funcs: &HashMap<Sym, IRFunc>, stmts: &[IRStmt]) -> Result<Vec<AssemblyOp>> {
    let mut asm: Vec<AssemblyOp> = vec![];

    macro_rules! expr {
        ($target: expr, $expr: expr) => {
            asm.append(&mut compile_expr(ctx, funcs, $target, $expr)?);
        };
    }

    for stmt in stmts {
        match stmt {
            IRStmt::Noop => {}
            IRStmt::VariableDefine { vars } => {
                for var in vars {
                    let ptr = ctx.alloc(var.id);
                    if let Some(init) = &var.init {
                        expr!(ptr, init);
                    }
                }
            }
            IRStmt::Assign { id, value } => {
                let val_p = ctx.alloc_noname();

                expr!(val_p, value);
                let ptr = ctx.get(*id)?;
                asm.push(AssemblyOp::Move(val_p, vec![(ptr, 1)]));
                
                ctx.free(val_p)?;
            }
            IRStmt::Call { id, args } => {
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

                ctx.pop();
            }
            IRStmt::Out { val } => {
                let val_p = ctx.alloc_noname();
                asm.append(&mut compile_expr(ctx, funcs, val_p, val)?);
                asm.push(AssemblyOp::Out(val_p));
                ctx.free(val_p)?;
            }
            IRStmt::While { condition, body } => {
                let cond_p = ctx.alloc_noname();

                expr!(cond_p, condition);

                ctx.push();
                let mut asm_body = compile_stmts(ctx, funcs, body)?;
                ctx.pop();
                asm_body.append(&mut compile_expr(ctx, funcs, cond_p, condition)?);
                asm.push(AssemblyOp::Loop(cond_p, asm_body));

                ctx.free(cond_p)?;
            }
            IRStmt::If { condition, body, else_body } => {
                if let Some(else_b) = else_body {
                    let cond_p = ctx.alloc_noname();
                    let else_p = ctx.alloc_noname();

                    asm.push(AssemblyOp::Set(else_p, 1));
                    expr!(cond_p, condition);
                    ctx.push();
                    let mut asm_body = compile_stmts(ctx, funcs, body)?;
                    ctx.pop();
                    asm_body.push(AssemblyOp::Set(cond_p, 0));
                    asm_body.push(AssemblyOp::Set(else_p, 0));
                    asm.push(AssemblyOp::Loop(cond_p, asm_body));

                    ctx.push();
                    let mut asm_body = compile_stmts(ctx, funcs, else_b)?;
                    ctx.pop();
                    asm_body.push(AssemblyOp::Set(else_p, 0));
                    asm.push(AssemblyOp::Loop(else_p, asm_body));

                    ctx.free(else_p)?;
                    ctx.free(cond_p)?;
                } else {
                    let cond_p = ctx.alloc_noname();

                    expr!(cond_p, condition);
                    ctx.push();
                    let mut asm_body = compile_stmts(ctx, funcs, body)?;
                    ctx.pop();
                    asm_body.push(AssemblyOp::Set(cond_p, 0));
                    asm.push(AssemblyOp::Loop(cond_p, asm_body));

                    ctx.free(cond_p)?;
                }
            }
            IRStmt::Block { body } => {
                ctx.push();
                asm.append(&mut compile_stmts(ctx, funcs, body)?);
                ctx.pop();
            }
        }
    }

    Ok(asm)
}
