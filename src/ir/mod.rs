pub mod ir;

use std::path::Path;

use anyhow::Result;
use boa_parser::{Parser, Source};

use crate::ir::ir::IR;

pub fn parse_to_ir(fpath: &Path) -> Result<IR> {
    let mut parser = Parser::new(Source::from_filepath(fpath)?);
    let mut interner = Default::default();
    let mut scope = Default::default();
    let script = parser.parse_script(&mut scope, &mut interner)?;
    println!("{script:?}");
    
    unimplemented!();
}
