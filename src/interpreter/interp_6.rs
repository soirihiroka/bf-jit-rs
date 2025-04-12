use std::{
    error, ffi::CString, io::{self, Read}
};

use libc::c_int;

use crate::bf_types;
#[derive(Debug)]
enum Ops {
    Left(usize),
    Right(usize),
    Add(u8),
    Sub(u8),
    Zero,
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}
pub fn run(prog: &[u8]) -> Result<(), Box<dyn error::Error>> {
    let mut prog_ops = vec![];
    let bytes = prog;
    let mut i = 0;
    while i < prog.len() {
        match bytes[i] as char {
            '<' => {
                let mut count = 1;
                let mut j = i + 1;
                while j < bytes.len() && bytes[j] as char == '<' {
                    count += 1;
                    j += 1;
                }
                prog_ops.push(Ops::Left(count));
                i = j - 1;
            }
            '>' => {
                let mut count = 1;
                let mut j = i + 1;
                while j < bytes.len() && bytes[j] as char == '>' {
                    count += 1;
                    j += 1;
                }
                prog_ops.push(Ops::Right(count));
                i = j - 1;
            }
            '-' => {
                let mut count = 1;
                let mut j = i + 1;
                while j < bytes.len() && bytes[j] as char == '-' {
                    count += 1;
                    j += 1;
                }
                prog_ops.push(Ops::Sub(count));
                i = j - 1;
            }
            '+' => {
                let mut count = 1;
                let mut j = i + 1;
                while j < bytes.len() && bytes[j] as char == '+' {
                    count += 1;
                    j += 1;
                }
                prog_ops.push(Ops::Add(count));
                i = j - 1;
            }
            '[' => prog_ops.push(Ops::LBrack(usize::max_value())),
            ']' => prog_ops.push(Ops::RBrack(usize::max_value())),
            '.' => prog_ops.push(Ops::Output),
            ',' => prog_ops.push(Ops::Input),
            _ => (),
        }
        i += 1;
    }

    // Use indices to iterate over prog_ops to avoid immutable borrow
    let mut i = 0;
    let mut bracket_index_stack = vec![];
    while i < prog_ops.len() {
        match prog_ops[i] {
            Ops::LBrack(_) => {
                bracket_index_stack.push(i);
            }
            Ops::RBrack(_) => {
                let open_bracket = bracket_index_stack.pop().unwrap();
                // Safe to mutate prog_ops because we're not borrowing it as immutable
                prog_ops[open_bracket] = Ops::LBrack(i);
                prog_ops[i] = Ops::RBrack(open_bracket);
            }
            _ => (),
        }
        i += 1;
    }

    let mut i = 0;
    while i < prog_ops.len() - 4 {
        match prog_ops[i..i + 3] {
            [Ops::LBrack(_), Ops::Sub(_), Ops::RBrack(_)] => {
                prog_ops[i] = Ops::Zero;
                prog_ops[i + 1] = Ops::Zero;
                prog_ops[i + 2] = Ops::Zero;
                i += 3;
            }
            _ => {
                i += 1;
            }
        }
    }

    let mut cells = vec![0u8; bf_types::BF_MEMORY_SIZE];
    let mut cc = 0usize;
    let mut pc = 0;
    while pc < prog_ops.len() {
        match prog_ops[pc] {
            // _ => todo!("Copy interp5, but update Ops::Sub, Ops::Left, Ops::Right instruction."),
            Ops::Left(v) => {
                cc -= v;
            }
            Ops::Right(v) => {
                cc += v;
            }
            Ops::Add(v) => {
                cells[cc] = cells[cc].wrapping_add(v);
            }
            Ops::Sub(v) => {
                cells[cc] = cells[cc].wrapping_sub(v);
            }
            Ops::Zero => {
                cells[cc] = 0;
            }
            Ops::LBrack(jump) if cells[cc] == 0 => {
                pc = jump;
            }
            Ops::RBrack(jump) if cells[cc] != 0 => {
                pc = jump;
            }
            Ops::Output => unsafe {
                let _ = libc::putchar(cells[cc] as c_int);
            },
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (),
        }
        pc += 1;
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use crate::tests::test_helper::test_run;

//     use super::*;

//     #[test]
//     fn it_works() {
//         assert!(test_run(run).is_ok());
//     }
// }
