use dynasmrt::{AssemblyOffset, ExecutableBuffer};
use dynasmrt::{DynasmApi, DynasmLabelApi, dynasm};

use core::str;
use std::error;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::stdin;
use std::io::stdout;
use std::io::{BufRead, Read, Write};
use std::mem;
use std::slice;

use crate::bf_types::BF_MEMORY_SIZE;

#[derive(Debug)]
enum Ops {
    Left(usize),
    Right(usize),
    Add(u8),
    Sub(u8),
    Zero,
    LBrack,
    RBrack,
    Output,
    Input,
}

type OpSequence = Vec<Ops>;

fn parse(prog_src: &[u8]) -> Result<OpSequence, &'static str> {
    let mut prog_ops = vec![];
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
                prog_ops.push(Ops::LBrack);
            }
            ']' => {
                prog_ops.push(Ops::RBrack);
            }
            '.' => prog_ops.push(Ops::Output),
            ',' => prog_ops.push(Ops::Input),
            _ => (),
        }
        i += 1;
    }
    Ok(prog_ops)
}

struct State<'a> {
    pub input: Box<dyn BufRead + 'a>,
    pub output: Box<dyn Write + 'a>,
}

impl<'a> State<'a> {
    unsafe extern "win64" fn getchar(state: *mut State, cell: *mut u8) -> u8 {
        let state = &mut *state;
        (state
            .input
            .read_exact(slice::from_raw_parts_mut(cell, 1))
            .is_err()) as u8
    }

    unsafe extern "win64" fn putchar(state: *mut State, cell: *mut u8) -> u8 {
        let state = &mut *state;
        state
            .output
            .write_all(slice::from_raw_parts(cell, 1))
            .is_err() as u8
    }

    fn new(input: Box<dyn BufRead + 'a>, output: Box<dyn Write + 'a>) -> State<'a> {
        State {
            input: input,
            output: output,
        }
    }
}

macro_rules! x64_bf {
    ($ops:ident $($t:tt)*) => {
        dynasm!($ops
            ; .arch x64
            ; .alias a_state, rcx // 1st arg in win64
            ; .alias a_current, rdx // 2nd arg in win64
            ; .alias retval, rax
            $($t)*
        )
    }
}

macro_rules! call_extern {
    ($ops:ident, $addr:expr) => {x64_bf!($ops
        ; mov [rsp + 0x08], a_current
        ; mov retval, QWORD $addr as _
        ; call retval
        ; mov a_state, [rsp + 0x00]
        ; mov a_current, [rsp + 0x08]
    );};
}

fn compile(prog: &[u8]) -> Result<(ExecutableBuffer, AssemblyOffset), Box<dyn error::Error>> {
    let bf_ops: Vec<Ops> = parse(&prog)?;
    let mut ops: dynasmrt::Assembler<dynasmrt::x64::X64Relocation> =
        dynasmrt::x64::Assembler::new()?;
    let mut loop_stack: Vec<(dynasmrt::DynamicLabel, dynasmrt::DynamicLabel)> = vec![];

    let start = ops.offset();
    x64_bf!(ops
        ; sub rsp, 0x18
        ; mov [rsp + 0x00], a_state
    );

    for op in bf_ops {
        match op {
            Ops::Left(amount) => x64_bf!(ops;
                sub a_current, (amount) as _
            ),
            Ops::Right(amount) => x64_bf!(ops;
                add a_current, (amount) as _
            ),
            Ops::Add(amount) => x64_bf!(ops;
                add BYTE [a_current], amount as _
            ),
            Ops::Sub(amount) => x64_bf!(ops;
                sub BYTE [a_current], amount as _
            ),
            Ops::Zero => x64_bf!(ops;
                mov BYTE [a_current], 0
            ),
            Ops::LBrack => {
                let backward_label = ops.new_dynamic_label();
                let forward_label = ops.new_dynamic_label();
                loop_stack.push((backward_label, forward_label));
                x64_bf!(ops
                    ; cmp BYTE [a_current], 0
                    ; jz =>forward_label
                    ;=>backward_label
                );
            }
            Ops::RBrack => {
                if let Some((backward_label, forward_label)) = loop_stack.pop() {
                    x64_bf!(ops
                        ; cmp BYTE [a_current], 0
                        ; jnz =>backward_label
                        ;=>forward_label
                    );
                } else {
                    return Err("] without matching [".into());
                }
            }
            Ops::Output => x64_bf!(ops
                ;; call_extern!(ops, State::putchar)
                ; cmp al, 0
                ; jnz ->io_failure
            ),
            Ops::Input => x64_bf!(ops
                ;; call_extern!(ops, State::getchar)
                ; cmp al, 0
                ; jnz ->io_failure
            ),
        }
    }
    if loop_stack.len() != 0 {
        return Err("[ without matching ]".into());
    }

    x64_bf!(ops
        ; mov retval, 0
        ; add rsp, 0x18
        ; ret
        ;->io_failure:
        ; mov retval, 1
        ; add rsp, 0x18
        ; ret
    );

    let buffer = ops
        .finalize()
        .map_err(|e| format!("Assembler finalize error: {:?}", e))?;

    Ok((buffer, start))
}

pub fn run(prog: &[u8]) -> Result<(), Box<dyn error::Error>> {
    let (exe_buf, start) = compile(prog)?;
    let mut state = State::new(
        Box::new(BufReader::new(stdin())),
        Box::new(BufWriter::new(stdout())),
    );
    let mut cells: [u8; BF_MEMORY_SIZE] = [0; BF_MEMORY_SIZE];
    let cp = cells.as_mut_ptr();

    let f: extern "win64" fn(*mut State, *mut u8) -> u8 =
        unsafe { mem::transmute(exe_buf.ptr(start)) };

    let res = f(&mut state, cp);

    if res == 0 {
        Ok(())
    } else if res == 1 {
        Err("IO error".into())
    } else {
        Err(format!("Unknown Error: {res}").into())
    }
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
