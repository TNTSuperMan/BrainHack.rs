pub mod ir;
mod stmt;
mod stmt_item;
mod expr;
mod ctx;

use std::{collections::HashMap, path::Path};

use anyhow::{Result};
use boa_interner::Sym;
use boa_parser::{Parser, Source};

use crate::ir::{ctx::ParserContext, ir::{IR, IRFunc, IRStmt}, stmt_item::parse_statement_item};

pub fn parse_to_ir(fpath: &Path) -> Result<IR> {
    let mut parser = Parser::new(Source::from_filepath(fpath)?);
    let mut interner = Default::default();
    let mut scope = Default::default();
    let script = parser.parse_script(&mut scope, &mut interner)?;

    interner.get_or_intern("out");
    interner.get_or_intern("input");

    let mut funcs = HashMap::<Sym, IRFunc>::new();
    
    Ok(IR {
        main: script.statements().iter().map(|s| {
            parse_statement_item(&mut ParserContext {
                interner: &interner,
                funcs: Some(&mut funcs),
            }, s)
        }).collect::<Result<Vec<IRStmt>>>()?,
        funcs,
    })
}
