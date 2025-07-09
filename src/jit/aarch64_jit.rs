use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
use dynasmrt::{AssemblyOffset, ExecutableBuffer};

use itertools::multipeek;
use itertools::Itertools;

use core::str;
use std::error;
use std::io::stdin;
use std::io::stdout;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::{BufRead, Read, Write};
use std::mem;
use std::slice;
use std::u8;

use crate::bf_types;
use crate::bf_types::BF_MEMORY_SIZE;

macro_rules! arm64_bf {
    ($ops:ident $($t:tt)*) => {
        dynasm!($ops
            ; .arch aarch64
            ; .alias a_state, x0
            ; .alias a_current, x1
            ; .alias a_begin, x2
            ; .alias a_end, x3
            ; .alias retval, x0
            $($t)*
        );
    }
}

macro_rules! prologue {
    ($ops:ident) => {{
        let start = $ops.offset();
        arm64_bf!($ops
            ; str x30, [sp, #-16]!
            ; stp x0, x1, [sp, #-16]!
            ; stp x2, x3, [sp, #-16]!
        );
        start
    }};
}

macro_rules! epilogue {
    ($ops:ident, $e:expr) => {arm64_bf!($ops
        ; mov x0, $e
        ; add sp, sp, #32
        ; ldr x30, [sp], #16
        ; ret
    );};
}

macro_rules! call_extern {
    ($ops:ident, $addr:ident) => {arm64_bf!($ops
        ; str x1, [sp, #24]
        ; ldr x9, ->$addr
        ; blr x9
        ; mov x9, x0
        ; ldp x0, x1, [sp, #16]
        ; ldp x2, x3, [sp]
    );};
}

/// BrainFuck AST node
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

fn parse(prog_src: &bf_types::BfSrc) -> Result<OpSequence, &'static str> {
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
    tape: [u8; BF_MEMORY_SIZE],
}

fn compile(prog: &bf_types::BfSrc) -> Result<(ExecutableBuffer, AssemblyOffset), &'static str> {
    let bf_ops = parse(&prog)?;
    let mut ops=
        dynasmrt::aarch64::Assembler::new().unwrap();
    let mut loop_stack = vec![];
    

    // literal pool
    dynasm!(ops
        ; ->getchar:
        ; .qword State::getchar as _
        ; ->putchar:
        ; .qword State::putchar as _
    );

    let start = prologue!(ops);

    for c in bf_ops {
        match c {
            Ops::Left(amount) => {
                arm64_bf!(ops
                    ; sub a_current, a_current, (amount) as u32 & 0xFFF
                    ; sub a_current, a_current, (amount) as u32 >> 12, LSL #12
                );
            }
            Ops::Right(amount) => {
                arm64_bf!(ops
                    ; add a_current, a_current, (amount) as u32 & 0xFFF
                    ; add a_current, a_current, (amount) as u32 >> 12, LSL #12
                );
            }
            Ops::Add(amount) => {
                arm64_bf!(ops
                    ; ldrb w9, [a_current]
                    ; add w9, w9, amount as u32
                    ; strb w9, [a_current]
                );
            }
            Ops::Sub(amount) => {
                arm64_bf!(ops
                    ; ldrb w9, [a_current]
                    ; sub w9, w9, amount as u32
                    ; strb w9, [a_current]
                );
            }
            Ops::Zero => {
                arm64_bf!(ops
                    ; strb wzr, [a_current]
                );
            }
            Ops::LBrack => {
                let backward_label = ops.new_dynamic_label();
                let forward_label = ops.new_dynamic_label();
                loop_stack.push((backward_label, forward_label));
                arm64_bf!(ops
                    ; ldrb w9, [a_current]
                    ; cbz w9, =>forward_label
                    ;=>backward_label
                );
            }
            Ops::RBrack => {
                if let Some((backward_label, forward_label)) = loop_stack.pop() {
                    arm64_bf!(ops
                        ; ldrb w9, [a_current]
                        ; cbnz w9, =>backward_label
                        ;=>forward_label
                    );
                } else {
                    return Err("] without matching [");
                }
            }
            Ops::Output => {
                arm64_bf!(ops
                    ;; call_extern!(ops, putchar)
                    ; cbnz x9, ->io_failure
                );
            }
            Ops::Input => {
                arm64_bf!(ops
                    ;; call_extern!(ops, getchar)
                    ; cbnz x9, ->io_failure
                );
            }
        }
    }
    if loop_stack.len() != 0 {
        return Err("[ without matching ]");
    }

    arm64_bf!(ops
        ;; epilogue!(ops, 0)
        // ;->outbound:
        // ;; epilogue!(ops, 1)
        ;->io_failure:
        ;; epilogue!(ops, 2)
    );

    Ok((ops.finalize().unwrap(), start))
}

impl<'a> State<'a> {
    unsafe extern "C" fn getchar(state: *mut State, cell: *mut u8) -> u8 {
        let state = &mut *state;
        let err = state.output.flush().is_err();
        (state.input.read_exact(slice::from_raw_parts_mut(cell, 1)).is_err() || err) as u8
    }

    unsafe extern "C" fn putchar(state: *mut State, cell: *mut u8) -> u8 {
        let state = &mut *state;
        state.output.write_all(slice::from_raw_parts(cell, 1)).is_err() as u8
    }

    fn new(input: Box<dyn BufRead + 'a>, output: Box<dyn Write + 'a>) -> State<'a> {
        State {
            input: input,
            output: output,
            tape: [0; BF_MEMORY_SIZE],
        }
    }
}


pub fn run(prog: &bf_types::BfSrc) -> Result<(), Box<dyn error::Error>> {
    let (exe_buf, start) = compile(prog).unwrap();
    let mut state = State::new(
        Box::new(BufReader::new(stdin())),
        Box::new(BufWriter::new(stdout())),
    );

    let f: extern "C" fn(*mut State, *mut u8, *mut u8, *const u8) -> u8 =
        unsafe { mem::transmute(exe_buf.ptr(start)) };

    let start = state.tape.as_mut_ptr();
    let end = unsafe { start.offset(BF_MEMORY_SIZE as isize) };
    let res = f(&mut state, start, start, end);

    if res == 0 {
        Ok(())
    } else if res == 1 {
        Err(Box::<dyn error::Error>::from("Memory Error"))
    } else if res == 2 {
        Err(Box::<dyn error::Error>::from("IO error"))
    } else {
        panic!("Unknown error code");
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
