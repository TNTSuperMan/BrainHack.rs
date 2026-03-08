use std::collections::HashMap;

use anyhow::{Result, bail};
use boa_ast::{Declaration, Statement, StatementListItem, declaration::{Binding, VarDeclaration}, expression::Identifier};

use crate::ir::{expr::parse_expr, ir::{IRExpr, IRFunc, IRStmt}, stmt::parse_stmt};

pub fn parse_statement_item(statement: &StatementListItem, func_map: Option<&mut HashMap<Identifier, IRFunc>>) -> Result<IRStmt> {
    match statement {
        StatementListItem::Statement(stmt) => parse_stmt(stmt.as_ref()),
        StatementListItem::Declaration(decr) => {
            match decr.as_ref() {
                Declaration::Lexical(lex) => {
                    parse_stmt(&Statement::Var(VarDeclaration(lex.variable_list().clone())))
                }
                Declaration::FunctionDeclaration(func) => {
                    if let Some(funcs) = func_map {
                        let mut body = func.body().statements().to_vec();
                        let mut result: Option<IRExpr> = None;

                        if let Some(StatementListItem::Statement(s)) = func.body().statements().last() {
                            if let Statement::Return(ret) = s.as_ref() {
                                if let Some(r) = ret.target() {
                                    result = Some(parse_expr(r)?);
                                    body.pop();
                                }
                            }
                        }
                        funcs.insert(func.name(), IRFunc {
                            args: func.parameters().as_ref().iter().map(|p| {
                                if p.is_rest_param() {
                                    bail!("Unsupported spread argument detected");
                                }
                                if p.init().is_some() {
                                    bail!("Unimplemented argument init detected");
                                }
                                if let Binding::Identifier(id) = p.variable().binding() {
                                    Ok(*id)
                                } else {
                                    bail!("Unsupported argument detected");
                                }
                            }).collect::<Result<Vec<Identifier>>>()?,
                            code: body.iter().map(|statement| {
                                parse_statement_item(statement, None)
                            }).collect::<Result<Vec<IRStmt>>>()?,
                            result,
                        });
                        Ok(IRStmt::Noop)
                    } else {
                        bail!("Unsupported function declaration detected");
                    }
                }
                _ => bail!("Unsupported statement detected"),
            }
        }
    }
}
