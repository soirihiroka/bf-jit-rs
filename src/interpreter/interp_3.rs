use std::{
    error,
    io::{self, Read, Write},
};

use crate::bf_types;

#[derive(Debug)]
enum Ops {
    Left,
    Right,
    Add,
    Sub,
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}

type OpSequence = Vec<Ops>;

fn parse(prog: &[u8]) -> Result<OpSequence, &'static str> {
    let mut prog_ops = vec![];
    // a stack to keep track of the brackets
    let mut bracket_index_stack = vec![];
    let mut i: usize = 0;
    while i < prog.len() {
        match prog[i] as char {
            '<' => prog_ops.push(Ops::Left),
            '>' => prog_ops.push(Ops::Right),
            '+' => prog_ops.push(Ops::Add),
            '-' => prog_ops.push(Ops::Sub),
            '[' => {
                // Place holder
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
}

pub fn run(prog: &[u8]) -> Result<(), Box<dyn error::Error>> {
    let prog_ops = parse(prog).unwrap();
    let mut cells = vec![0u8; bf_types::BF_MEMORY_SIZE];
    let mut cp = 0;
    let mut pc = 0;
    while pc < prog_ops.len() {
        match prog_ops[pc] {
            Ops::Left => cp -= 1,
            Ops::Right => cp += 1,
            Ops::Add => cells[cp] = cells[cp].wrapping_add(1),
            Ops::Sub => cells[cp] = cells[cp].wrapping_sub(1),
            Ops::LBrack(jump) if cells[cp] == 0 => pc = jump,
            Ops::RBrack(jump) if cells[cp] != 0 => pc = jump,
            Ops::Output => io::stdout().write_all(&cells[cp..cp + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cp..cp + 1])?,
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
