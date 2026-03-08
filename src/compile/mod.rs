mod ctx;

use anyhow::{Result, bail};

use crate::{asm::asm::AssemblyProgram, ir::ir::IR};

pub fn compile(ir: &IR) -> Result<AssemblyProgram> {
    bail!("unimplemented");
}
