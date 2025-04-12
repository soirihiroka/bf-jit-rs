// pub mod inter_1 {
use std::{
    error,
    io::{self, Read, Write},
};

use crate::bf_types;

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
    let mut pc = 0; /* Program counter tracks location in the code */
    let mut cells = vec![0u8; bf_types::BF_MEMORY_SIZE]; /* memory */
    let mut cc = 0; /* Cell counter (data pointer) points to active location in memory*/
    while pc < prog.len() {
        match prog[pc] as char {
            '<' => {
                cc -= 1;
            }
            '>' => {
                cc += 1;
            }
            '+' => {
                cells[cc] = cells[cc].wrapping_add(1);
            }
            '-' => {
                cells[cc] = cells[cc].wrapping_sub(1);
            }
            '[' if cells[cc] == 0 => {
                let mut level = 1;
                while level > 0 {
                    pc += 1;
                    match prog[pc] as char {
                        '[' => {
                            level += 1;
                        }
                        ']' => {
                            level -= 1;
                        }
                        _ => (),
                    }
                }
            }
            ']' if cells[cc] != 0 => {
                let mut level = 1;
                while level > 0 {
                    pc -= 1;
                    match prog[pc] as char {
                        '[' => {
                            level -= 1;
                        }
                        ']' => {
                            level += 1;
                        }
                        _ => (),
                    }
                }
            }
            '.' => io::stdout().write_all(&cells[cc..cc + 1])?,
            ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (),
        }
        pc += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tests::test_helper::{test_hell, test_run};

    use super::*;

    #[test]
    fn hello() {
        assert!(test_run(&run).is_ok());
    }
    #[test]
    fn hello_hell() {
        assert!(test_hell(&run).is_ok());
    }
}
