use std::error;

#[cfg(target_arch = "x86_64")]
use crate::jit::x64_jit;

#[cfg(target_arch = "aarch64")]
use crate::jit::aarch64_jit;

/// Run a bf program
///
/// # Examples
///
/// ```
/// let prog = fs::read(env::args().nth(1).unwrap())?;
/// inter_1::run(prog)?;
/// ```
///
/// This is a naive implementation, which we will optimize further in other implementations.
pub fn run(prog: &[u8]) -> Result<(), Box<dyn error::Error>> {
    // run
    #[cfg(target_arch = "x86_64")]
    {
        return x64_jit::run(prog);
    }
    #[cfg(target_arch = "aarch64")]
    {
        return aarch64_jit::run(prog);
    }
    println!("Architecture not supported!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tests::test_helper::test_run;

    use super::*;

    #[test]
    fn it_works() {
        assert!(test_run(&run).is_ok());
    }
}
