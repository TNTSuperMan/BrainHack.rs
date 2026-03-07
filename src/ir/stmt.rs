use anyhow::{Result, bail};
use boa_ast::{Statement, StatementListItem, declaration::Binding};

use crate::ir::{expr::parse_expr, ir::IRStmt};

pub fn parse_stmts(statements: &[StatementListItem]) -> Result<Vec<IRStmt>> {
    let mut ir: Vec<IRStmt> = vec![];
    for statement in statements {
        match statement {
            StatementListItem::Statement(stmt) => {
                match stmt.as_ref() {
                    Statement::Var(var) => {
                        for d in var.0.as_ref() {
                            if let Binding::Identifier(id) = d.binding() {
                                ir.push(IRStmt::VariableDefine {
                                    id: *id,
                                    init: if let Some(i) = d.init() {
                                        Some(parse_expr(i)?)
                                    } else {
                                        None
                                    },
                                });
                            } else {
                                bail!("unsupport");
                            }
                        }
                    }
                    _ => bail!("unimp")
                }
            }
            _ => bail!("err"),
        }
    }

    Ok(ir)
}