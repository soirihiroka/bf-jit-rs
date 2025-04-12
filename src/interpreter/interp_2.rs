use std::{
    error,
    io::{self, Read, Write},
};

use crate::bf_types;

/// Operations for BF
#[derive(Debug)]
enum Ops {
    Left,
    Right,
    Add,
    Sub,
    LBrack,
    RBrack,
    Output,
    Input,
}

type OpSequence = Vec<Ops>;

fn parse(prog: &[u8]) -> Result<OpSequence, &'static str> {
    let mut prog_ops = vec![];
    /* First parse the program into a sequence of opcodes */
    for &b in prog {
        match b as char {
            '<' => prog_ops.push(Ops::Left),
            '>' => prog_ops.push(Ops::Right),
            '+' => prog_ops.push(Ops::Add),
            '-' => prog_ops.push(Ops::Sub),
            '[' => prog_ops.push(Ops::LBrack),
            ']' => prog_ops.push(Ops::RBrack),
            '.' => prog_ops.push(Ops::Output),
            ',' => prog_ops.push(Ops::Input),
            _ => (),
        }
    }
    Ok(prog_ops)
}

pub fn run(prog: &[u8]) -> Result<(), Box<dyn error::Error>> {
    /* Notice: prog is now a vec of OpCodes, not a string */
    let prog_ops = parse(prog)?;

    let mut pc = 0;

    let mut cells = vec![0u8; bf_types::BF_MEMORY_SIZE];
    let mut cc = 0;
    while pc < prog_ops.len() {
        match prog_ops[pc] {
            Ops::Left => {
                cc -= 1;
            }
            Ops::Right => {
                cc += 1;
            }
            Ops::Add => {
                cells[cc] = cells[cc].wrapping_add(1);
            }
            Ops::Sub => {
                cells[cc] = cells[cc].wrapping_sub(1);
            }
            Ops::LBrack if cells[cc] == 0 => {
                let mut level = 1;
                while level > 0 {
                    pc += 1;
                    match prog_ops[pc] {
                        Ops::LBrack => {
                            level += 1;
                        }
                        Ops::RBrack => {
                            level -= 1;
                        }
                        _ => (),
                    }
                }
            }
            Ops::RBrack if cells[cc] != 0 => {
                let mut level = 1;
                while level > 0 {
                    pc -= 1;
                    match prog_ops[pc] {
                        Ops::LBrack => {
                            level -= 1;
                        }
                        Ops::RBrack => {
                            level += 1;
                        }
                        _ => (),
                    }
                }
            }
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
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
