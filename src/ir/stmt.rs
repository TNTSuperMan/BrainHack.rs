use anyhow::{Result, bail};
use boa_ast::{Expression, Statement, declaration::{Binding, VarDeclaration}, expression::operator::{assign::{AssignOp, AssignTarget}, update::{UpdateOp, UpdateTarget}}, statement::iteration::ForLoopInitializer};

use crate::ir::{expr::parse_expr, ir::{IRExpr, IRStmt, IRVarInit}, stmt_item::parse_statement_item};

fn parse_block(statement: &Statement) -> Result<Vec<IRStmt>> {
    if let Statement::Block(block) = statement {
        block.statement_list().iter().map(|s| {
            parse_statement_item(s, None)
        }).collect()
    } else {
        parse_stmt(statement).map(|stmt| {
            vec![stmt]
        })
    }
}

pub fn parse_stmt(statement: &Statement) -> Result<IRStmt> {
    match statement {
        Statement::Var(var) => {
            var.0.as_ref().iter().map(|d| {
                if let Binding::Identifier(id) = d.binding() {
                    Ok(IRVarInit {
                        id: id.sym(),
                        init: if let Some(i) = d.init() {
                            Some(parse_expr(i)?)
                        } else {
                            None
                        },
                    })
                } else {
                    bail!("Unsupported variable declaration detected");
                }
            }).collect::<Result<Vec<IRVarInit>>>().map(|vars| {
                IRStmt::VariableDefine { vars }
            })
        }
        Statement::Expression(expr) => {
            match expr {
                Expression::Assign(assign) => {
                    if let AssignTarget::Identifier(id) = assign.lhs() {
                        let val = parse_expr(assign.rhs())?;
                        Ok(match assign.op() {
                            AssignOp::Assign => IRStmt::Assign { id: id.sym(), value: val },
                            AssignOp::Add => IRStmt::Assign { id: id.sym(), value: IRExpr::Add(Box::new(IRExpr::Id { id: id.sym(), last_use: false }), Box::new(val)) },
                            AssignOp::Sub => IRStmt::Assign { id: id.sym(), value: IRExpr::Sub(Box::new(IRExpr::Id { id: id.sym(), last_use: false }), Box::new(val)) },
                            AssignOp::Mul => IRStmt::Assign { id: id.sym(), value: IRExpr::Mul(Box::new(IRExpr::Id { id: id.sym(), last_use: false }), Box::new(val)) },
                            AssignOp::Div => IRStmt::Assign { id: id.sym(), value: IRExpr::Div(Box::new(IRExpr::Id { id: id.sym(), last_use: false }), Box::new(val)) },
                            _ => bail!("Unsupported assignment detected"),
                        })
                    } else {
                        bail!("Unsupported assignment target detected");
                    }
                }
                Expression::Call(call) => {
                    if let Expression::Identifier(id) = call.function() {
                        call.args().iter().map(|e| parse_expr(e)).collect::<Result<Vec<IRExpr>>>().map(|args| {
                            IRStmt::Call {
                                id: id.sym(),
                                args,
                            }
                        })
                    } else {
                        bail!("Unsupported callee detected");
                    }
                }
                Expression::Update(upd) => {
                    if let UpdateTarget::Identifier(id) = upd.target() {
                        let id_expr = Box::new(IRExpr::Id { id: id.sym(), last_use: false });
                        let one_expr = Box::new(IRExpr::Const(1));
                        Ok(match upd.op() {
                            UpdateOp::IncrementPost |
                            UpdateOp::IncrementPre => IRStmt::Assign { id: id.sym(), value: IRExpr::Add(id_expr, one_expr) },
                            UpdateOp::DecrementPost |
                            UpdateOp::DecrementPre => IRStmt::Assign { id: id.sym(), value: IRExpr::Sub(id_expr, one_expr) },
                        })
                    } else {
                        bail!("Unsupported update target detected");
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
            Ok(IRStmt::Block { body: parse_block(statement)? })
        }
        Statement::ForLoop(for_ast) => {
            let mut body: Vec<IRStmt> = vec![];

            if let Some(init) = for_ast.init() {
                body.push(match init {
                    ForLoopInitializer::Var(var) => parse_stmt(&Statement::Var(var.clone()))?,
                    ForLoopInitializer::Lexical(lex) => parse_stmt(&Statement::Var(VarDeclaration(lex.declaration().variable_list().clone())))?,
                    ForLoopInitializer::Expression(expr) => parse_stmt(&Statement::Expression(expr.clone()))?,
                });
            }

            let mut loop_body = parse_block(for_ast.body())?;

            if let Some(upd) = for_ast.final_expr() {
                loop_body.push(parse_stmt(&Statement::Expression(upd.clone()))?);
            }

            body.push(IRStmt::While {
                condition: for_ast.condition().map_or(
                    Ok(IRExpr::Const(1)),
                    parse_expr
                )?,
                body: loop_body,
            });

            Ok(IRStmt::Block { body })
        }
        Statement::Empty => Ok(IRStmt::Noop),
        _ => bail!("unimp | unsupport")
    }
}
