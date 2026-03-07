mod asm;
mod ir;

use std::path::Path;

use crate::{asm::asm::{AssemblyOp, AssemblyProgram}, ir::parse_to_ir};

fn main() {
    parse_to_ir(&Path::new("./box/example.js"));
}
