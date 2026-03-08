mod asm;
mod ir;
mod compile;

use std::{env::args, fs, path::Path, process::ExitCode};

use anyhow::Result;

use crate::{compile::compile, ir::parse_to_ir};

fn resulty_main(input: &str, output: &str) -> Result<()> {
    let ir = parse_to_ir(&Path::new(input))?;
    let asm = compile(&ir)?;
    fs::write(output, asm.assemble())?;
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();
    
    if args.len() < 3 {
        println!("usage: {} [INPUT] [OUTPUT]", args[0]);
        ExitCode::FAILURE
    } else {
        if let Err(e) = resulty_main(&args[1], &args[2]) {
            eprintln!("Error: {e:?}");
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }
}
