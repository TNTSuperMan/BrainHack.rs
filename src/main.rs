mod asm;
mod compile;
mod ir;

use std::{env::args, path::Path, process::ExitCode};

use anyhow::Result;

use crate::{compile::compile, ir::parse_to_ir};

fn resulty_main(input: &str) -> Result<()> {
    let ir = parse_to_ir(&Path::new(input))?;
    let asm = compile(&ir)?;
    let bf = asm.assemble();
    println!("{bf}");
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("usage: {} [INPUT]", args[0]);
        ExitCode::FAILURE
    } else {
        if let Err(e) = resulty_main(&args[1]) {
            eprintln!("Error: {e:?}");
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }
}
