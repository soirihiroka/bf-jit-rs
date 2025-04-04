use std::path::PathBuf;
use std::{error, fs};
#[macro_use]
extern crate lazy_static;
pub mod bf_types;
pub mod interpreter;
pub mod jit;
pub mod tests;
pub mod transpiler;

use clap::Parser;
use interpreter::interp::OptimizationLevel;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Set the running mode (int, src, jit)
    #[arg(short, long, default_value_t =String::from("jit"))]
    mode: String,

    /// Set an output location (required for src mode)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Set the brainfuck file to run
    #[arg(short, long)]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    let mode = cli.mode;

    let bf_file = cli.input;

    let prog: Vec<u8> = fs::read(bf_file)?;

    match mode.as_str() {
        "int" => {
            interpreter::interp::run(&prog, None)?;
        }
        "int1" => {
            interpreter::interp::run(&prog, Some(OptimizationLevel::Raw))?;
        }
        "int2" => {
            interpreter::interp::run(&prog, Some(OptimizationLevel::None))?;
        }
        "int3" => {
            interpreter::interp::run(&prog, Some(OptimizationLevel::Low))?;
        }
        "int4" => {
            interpreter::interp::run(&prog, Some(OptimizationLevel::Medium))?;
        }
        "int5" => {
            interpreter::interp::run(&prog, Some(OptimizationLevel::High))?;
        }
        "jit" => {
            jit::jit::run(&prog)?;
        }
        "bf2c" => {
            let output_file = cli.output.ok_or("Output file required for bf2c mode")?;
            transpiler::bf2c::transpile_to_file(
                &prog,
                output_file.to_str().ok_or("Invalid output file path")?,
            )
            .expect("Failed to transpile Brainfuck to C");
            println!("Transpiled to C: {:?}", output_file);
        }
        "bf2js" => {
            let output_file = cli.output.ok_or("Output file required for bf2js mode")?;
            transpiler::bf2js::transpile_to_file(
                &prog,
                output_file.to_str().ok_or("Invalid output file path")?,
            )
            .expect("Failed to transpile Brainfuck to JavaScript");
            println!("Transpiled to JavaScript: {:?}", output_file);
        }
        _ => panic!("Unknown mode: {}", mode),
    }

    // benchmarks(&prog)?;

    Ok(())
}
