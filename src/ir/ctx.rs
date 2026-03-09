use std::collections::HashMap;

use boa_interner::{Interner, Sym};

use crate::ir::ir::IRFunc;

pub struct ParserContext<'a, 'b, 'c> {
    pub interner: &'a Interner,
    pub funcs: Option<&'b mut HashMap<Sym, IRFunc>>,
    pub arrays: &'c mut Vec<Sym>,
}
