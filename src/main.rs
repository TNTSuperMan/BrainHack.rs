mod asm;
mod ir;

use std::path::Path;

use crate::{asm::asm::{AssemblyOp, AssemblyProgram}, ir::parse_to_ir};

fn main() {
    //parse_to_ir(&Path::new("./box/example.js"));
    let program = AssemblyProgram {
        static_memory_size: 2,
        dynamic_memory_block_size: 1,
        code: vec![
            AssemblyOp::Add(5, 3),
            AssemblyOp::Add(6, 6),
            AssemblyOp::Send(0),
            AssemblyOp::Add(0, 0),
            AssemblyOp::Set(5, 3),
            AssemblyOp::Fetch(0),
            AssemblyOp::Out(3),
        ],
    };
    println!("{}", program.assemble());
}
