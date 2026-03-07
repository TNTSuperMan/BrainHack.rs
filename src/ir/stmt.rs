use anyhow::{Result, bail};
use boa_ast::{Expression, Statement, StatementListItem, declaration::Binding, expression::operator::assign::{AssignOp, AssignTarget}};

use crate::ir::{expr::parse_expr, ir::{IRExpr, IRStmt}};

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
                    Statement::Expression(expr) => {
                        match expr {
                            Expression::Assign(assign) => {
                                if let AssignTarget::Identifier(id) = assign.lhs() {
                                    let val = parse_expr(assign.rhs())?;
                                    ir.push(match assign.op() {
                                        AssignOp::Assign => IRStmt::Assign { id: *id, value: val },
                                        AssignOp::Add => IRStmt::Assign { id: *id, value: IRExpr::Add(Box::new(IRExpr::Id(*id)), Box::new(val)) },
                                        AssignOp::Sub => IRStmt::Assign { id: *id, value: IRExpr::Sub(Box::new(IRExpr::Id(*id)), Box::new(val)) },
                                        AssignOp::Mul => IRStmt::Assign { id: *id, value: IRExpr::Mul(Box::new(IRExpr::Id(*id)), Box::new(val)) },
                                        AssignOp::Div => IRStmt::Assign { id: *id, value: IRExpr::Div(Box::new(IRExpr::Id(*id)), Box::new(val)) },
                                        _ => bail!("unsupport"),
                                    })
                                } else {
                                    bail!("unsupport");
                                }
                            }
                            _ => bail!("unimp | unsupport"),
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