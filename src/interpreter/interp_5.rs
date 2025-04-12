use std::{
    error,
    io::{self, Read, Write},
};

use crate::bf_types;
/// BrainFuck AST node
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

type OpSequence = Vec<Ops>;

fn parse(prog_src: &[u8]) -> Result<OpSequence, &'static str> {
    let mut prog_ops = vec![];
    let mut bracket_index_stack = vec![];
    let mut i = 0;
    while i < prog_src.len() {
        match prog_src[i] as char {
            '<' => {
                let mut count = 1;
                let mut j = i + 1;
                while j < prog_src.len() && prog_src[j] as char == '<' {
                    count += 1;
                    j += 1;
                }
                prog_ops.push(Ops::Left(count));
                i = j - 1;
            }
            '>' => {
                let mut count = 1;
                let mut j = i + 1;
                while j < prog_src.len() && prog_src[j] as char == '>' {
                    count += 1;
                    j += 1;
                }
                prog_ops.push(Ops::Right(count));
                i = j - 1;
            }
            '-' => {
                let mut count = 1;
                let mut j = i + 1;
                while j < prog_src.len() && prog_src[j] as char == '-' {
                    count += 1;
                    j += 1;
                }
                prog_ops.push(Ops::Sub(count));
                i = j - 1;
            }
            '+' => {
                let mut count = 1;
                let mut j = i + 1;
                while j < prog_src.len() && prog_src[j] as char == '+' {
                    count += 1;
                    j += 1;
                }
                prog_ops.push(Ops::Add(count));
                i = j - 1;
            }
            '[' => {
                // Check if it is [-]
                if i + 2 < prog_src.len()
                    && prog_src[i + 1] as char == '-'
                    && prog_src[i + 2] as char == ']'
                {
                    prog_ops.push(Ops::Zero);
                    i += 3;
                    continue;
                }
                bracket_index_stack.push(prog_ops.len());
                prog_ops.push(Ops::LBrack(usize::max_value()));
            }
            ']' => {
                let open_bracket = bracket_index_stack.pop().unwrap();
                prog_ops[open_bracket] = Ops::LBrack(prog_ops.len() - 1);
                prog_ops.push(Ops::RBrack(open_bracket));
            }
            '.' => prog_ops.push(Ops::Output),
            ',' => prog_ops.push(Ops::Input),
            _ => (),
        }
        i += 1;
    }

    Ok(prog_ops)
    // TODO: Return error if program is not valid
}

pub fn run(prog: &[u8]) -> Result<(), Box<dyn error::Error>> {
    let prog_ops = parse(prog)?;

    let mut cells = vec![0u8; bf_types::BF_MEMORY_SIZE];
    let mut cc = 0usize;
    let mut pc = 0;
    while pc < prog_ops.len() {
        match prog_ops[pc] {
            Ops::Left(v) => cc -= v,
            Ops::Right(v) => cc += v,
            Ops::Add(v) => cells[cc] = cells[cc].wrapping_add(v),
            Ops::Sub(v) => cells[cc] = cells[cc].wrapping_sub(v),
            Ops::Zero => cells[cc] = 0,
            Ops::LBrack(jump) if cells[cc] == 0 => pc = jump,
            Ops::RBrack(jump) if cells[cc] != 0 => pc = jump,
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
