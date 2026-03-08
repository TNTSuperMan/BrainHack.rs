pub mod ir;
mod stmt;
mod stmt_item;
mod expr;

use std::{collections::HashMap, path::Path};

use anyhow::{Result};
use boa_ast::expression::Identifier;
use boa_parser::{Parser, Source};

use crate::ir::{ir::{IR, IRFunc, IRStmt}, stmt_item::parse_statement_item};

pub fn parse_to_ir(fpath: &Path) -> Result<IR> {
    let mut parser = Parser::new(Source::from_filepath(fpath)?);
    let mut interner = Default::default();
    let mut scope = Default::default();
    let script = parser.parse_script(&mut scope, &mut interner)?;

    let mut funcs = HashMap::<Identifier, IRFunc>::new();
    
    Ok(IR {
        main: script.statements().iter().map(|s| {
            parse_statement_item(s, Some(&mut funcs))
        }).collect::<Result<Vec<IRStmt>>>()?,
        funcs,
    })
}
