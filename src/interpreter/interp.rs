use std::error;

use crate::interpreter::*;

pub enum OptimizationLevel {
    Raw,
    None,
    Low,
    Medium,
    High,
}

pub fn run(
    prog: &[u8],
    optimization_level: Option<OptimizationLevel>,
) -> Result<(), Box<dyn error::Error>> {
    let ol = optimization_level.unwrap_or(OptimizationLevel::High);

    match ol {
        OptimizationLevel::Raw => interp_1::run(prog),
        OptimizationLevel::None => interp_2::run(prog),
        OptimizationLevel::Low => interp_3::run(prog),
        OptimizationLevel::Medium => interp_4::run(prog),
        OptimizationLevel::High => interp_5::run(prog),
    }
}
