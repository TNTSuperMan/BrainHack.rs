use anyhow::{Result, bail};
use boa_ast::{Declaration, Expression, Statement, StatementListItem, declaration::Binding};
use boa_interner::Sym;

use crate::ir::{ctx::ParserContext, expr::parse_expr, ir::{IRExpr, IRFunc, IRStmt, IRVarInit}, stmt::parse_stmt};

pub fn parse_statement_item(ctx: &mut ParserContext, statement: &StatementListItem) -> Result<IRStmt> {
    match statement {
        StatementListItem::Statement(stmt) => parse_stmt(ctx, stmt.as_ref()),
        StatementListItem::Declaration(decr) => {
            match decr.as_ref() {
                Declaration::Lexical(lex) => {
                    let mut vars: Vec<IRVarInit> = vec![];
                    for var in lex.variable_list().as_ref() {
                        if let Binding::Identifier(id) = var.binding() {
                            if let Some(Expression::ArrayLiteral(..)) = var.init() {
                                ctx.arrays.push(id.sym());
                            } else {
                                vars.push(IRVarInit {
                                    id: id.sym(),
                                    init: if let Some(i) = var.init() {
                                        Some(parse_expr(&mut ParserContext {
                                            interner: ctx.interner,
                                            funcs: None,
                                            arrays: ctx.arrays,
                                        }, i)?)
                                    } else {
                                        None
                                    },
                                });
                            }
                        } else {
                            bail!("Unsupported variable declaration detected");
                        }
                    }
                    Ok(IRStmt::VariableDefine { vars })
                }
                Declaration::FunctionDeclaration(func) => {
                    if let Some(funcs) = &mut ctx.funcs {
                        let mut body = func.body().statements().to_vec();
                        let mut result: Option<IRExpr> = None;

                        if let Some(StatementListItem::Statement(s)) = func.body().statements().last() {
                            if let Statement::Return(ret) = s.as_ref() {
                                if let Some(r) = ret.target() {
                                    result = Some(parse_expr(&mut ParserContext {
                                        interner: ctx.interner,
                                        funcs: None,
                                        arrays: ctx.arrays,
                                    }, r)?);
                                    body.pop();
                                }
                            }
                        }
                        funcs.insert(func.name().sym(), IRFunc {
                            args: func.parameters().as_ref().iter().map(|p| {
                                if p.is_rest_param() {
                                    bail!("Unsupported spread argument detected");
                                }
                                if p.init().is_some() {
                                    bail!("Unimplemented argument init detected");
                                }
                                if let Binding::Identifier(id) = p.variable().binding() {
                                    Ok(id.sym())
                                } else {
                                    bail!("Unsupported argument detected");
                                }
                            }).collect::<Result<Vec<Sym>>>()?,
                            code: body.iter().map(|statement| {
                                parse_statement_item(&mut ParserContext {
                                    interner: ctx.interner,
                                    funcs: None,
                                    arrays: ctx.arrays,
                                }, statement)
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
