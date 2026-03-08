pub mod ir;
mod stmt;
mod expr;

use std::{collections::HashMap, path::Path};

use anyhow::{Result, bail};
use boa_ast::{Declaration, Statement, StatementListItem, declaration::Binding, expression::Identifier};
use boa_parser::{Parser, Source};

use crate::ir::{expr::parse_expr, ir::{IR, IRExpr, IRFunc, IRStmt}, stmt::parse_stmt};

pub fn parse_to_ir(fpath: &Path) -> Result<IR> {
    let mut parser = Parser::new(Source::from_filepath(fpath)?);
    let mut interner = Default::default();
    let mut scope = Default::default();
    let script = parser.parse_script(&mut scope, &mut interner)?;
    println!("{script:?}");

    let mut ir = IR {
        main: vec![],
        funcs: HashMap::new(),
    };

    for statement in script.statements().iter() {
        match statement {
            StatementListItem::Statement(stmt) => {
                ir.main.push(parse_stmt(stmt)?);
            }
            StatementListItem::Declaration(decr) => {
                if let Declaration::FunctionDeclaration(func) = decr.as_ref() {
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
                    ir.funcs.insert(func.name(), IRFunc {
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
                            if let StatementListItem::Statement(s) = statement {
                                parse_stmt(s.as_ref())
                            } else {
                                bail!("Unsupported declaration detected in function");
                            }
                        }).collect::<Result<Vec<IRStmt>>>()?,
                        result,
                    });
                } else {
                    bail!("Unsupported declaration detected");
                }
            }
        }
    }
    
    Ok(ir)
}
