use anyhow::{Result, bail};
use boa_ast::{Expression, Statement, StatementListItem, declaration::Binding, expression::operator::{assign::{AssignOp, AssignTarget}, update::{UpdateOp, UpdateTarget}}};

use crate::ir::{expr::parse_expr, ir::{IRExpr, IRStmt, IRVarInit}};

fn parse_block(statement: &Statement) -> Result<Vec<IRStmt>> {
    if let Statement::Block(block) = statement {
        block.statement_list().iter().map(parse_stmt).collect()
    } else {
        Ok(vec![parse_stmt(&StatementListItem::Statement(Box::new(statement.clone())))?])
    }
}

pub fn parse_stmt(statement: &StatementListItem) -> Result<IRStmt> {
    match statement {
        StatementListItem::Statement(stmt) => {
            match stmt.as_ref() {
                Statement::Var(var) => {
                    let vars: Result<Vec<IRVarInit>> = var.0.as_ref().iter().map(|d| {
                        if let Binding::Identifier(id) = d.binding() {
                            Ok(IRVarInit {
                                id: *id,
                                init: if let Some(i) = d.init() {
                                    Some(parse_expr(i)?)
                                } else {
                                    None
                                },
                            })
                        } else {
                            bail!("unsupport");
                        }
                    }).collect();
                    Ok(IRStmt::VariableDefine { vars: vars? })
                }
                Statement::Expression(expr) => {
                    match expr {
                        Expression::Assign(assign) => {
                            if let AssignTarget::Identifier(id) = assign.lhs() {
                                let val = parse_expr(assign.rhs())?;
                                Ok(match assign.op() {
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
                        Expression::Call(call) => {
                            if let Expression::Identifier(id) = call.function() {
                                Ok(IRStmt::Call {
                                    id: *id,
                                    args: call.args().iter().map(|e| parse_expr(e)).collect::<Result<Vec<IRExpr>>>()?,
                                })
                            } else {
                                bail!("unsupport");
                            }
                        }
                        Expression::Update(upd) => {
                            if let UpdateTarget::Identifier(id) = upd.target() {
                                let id_expr = Box::new(IRExpr::Id(*id));
                                let one_expr = Box::new(IRExpr::Const(1));
                                Ok(match upd.op() {
                                    UpdateOp::IncrementPost |
                                    UpdateOp::IncrementPre => IRStmt::Assign { id: *id, value: IRExpr::Add(id_expr, one_expr) },
                                    UpdateOp::DecrementPost |
                                    UpdateOp::DecrementPre => IRStmt::Assign { id: *id, value: IRExpr::Sub(id_expr, one_expr) },
                                })
                            } else {
                                bail!("unsupport");
                            }
                        }
                        _ => bail!("unimp | unsupport"),
                    }
                }
                Statement::WhileLoop(whileloop) => {
                    Ok(IRStmt::While {
                        condition: parse_expr(whileloop.condition())?,
                        body: parse_block(whileloop.body())?,
                    })
                }
                Statement::If(if_ast) => {
                    Ok(IRStmt::If {
                        condition: parse_expr(if_ast.cond())?,
                        body: parse_block(if_ast.body())?,
                        else_body: if let Some(s) = if_ast.else_node() {
                            Some(parse_block(s)?)
                        } else {
                            None
                        },
                    })
                }
                Statement::Block(_) => {
                    Ok(IRStmt::Block { body: parse_block(stmt)? })
                }
                _ => bail!("unimp")
            }
        }
        _ => bail!("err"),
    }
}
