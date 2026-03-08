use std::collections::HashMap;

use anyhow::{Result, anyhow, bail};
use boa_ast::expression::Identifier;

use crate::{asm::asm::AssemblyOp, compile::{ctx::CompileContext, expr::compile_expr}, ir::ir::{IRFunc, IRStmt}};

pub fn compile_stmts(ctx: &mut CompileContext, funcs: &HashMap<Identifier, IRFunc>, stmts: &[IRStmt]) -> Result<Vec<AssemblyOp>> {
    ctx.push();
    let mut asm: Vec<AssemblyOp> = vec![];

    for stmt in stmts {
        match stmt {
            IRStmt::Noop => {}
            IRStmt::VariableDefine { vars } => {
                for var in vars {
                    let ptr = ctx.alloc(var.id);
                    if let Some(init) = &var.init {
                        asm.append(&mut compile_expr(ctx, funcs, ptr, init)?);
                    }
                }
            }
            IRStmt::Assign { id, value } => {
                let val_p = ctx.alloc_noname();

                asm.append(&mut compile_expr(ctx, funcs, val_p, value)?);
                let ptr = ctx.get(*id)?;
                asm.push(AssemblyOp::Move(val_p, vec![(ptr, 1)]));
                
                ctx.free(val_p)?;
            }
            IRStmt::Call { id, args } => {
                let func = funcs.get(id).ok_or_else(|| anyhow!("Undefined function detected"))?;
                if func.args.len() != args.len() {
                    bail!("Function args length mismatch");
                }
                ctx.push();

                for (i, arg) in func.args.iter().enumerate() {
                    let ptr = ctx.alloc(*arg);
                    asm.append(&mut compile_expr(ctx, funcs, ptr, &args[i])?);
                }

                asm.append(&mut compile_stmts(ctx, funcs, &func.code)?);

                ctx.pop();
            }
            _ => bail!("todo"),
        }
    }

    ctx.pop();
    Ok(asm)
}
