use std::fs::{self};
use stdio_override::StdoutOverride;

use std::sync::Mutex;

lazy_static! {
    static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
}

use std::error::Error;

type RunFn = (dyn Fn(&[u8]) -> Result<(), Box<dyn Error>>);

pub fn test_run(run_func: &RunFn) -> Result<(), Box<dyn Error>> {
    let test_lock = TEST_MUTEX.lock().unwrap();

    let tmp_filename = "./test.tmp";

    // If the file exists, remove it
    if fs::metadata(tmp_filename).is_ok() {
        fs::remove_file(tmp_filename)?;
    }

    let hello_bf = "something something ++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+hello there.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.[-]";
    let guard = StdoutOverride::override_file(tmp_filename)?;
    let prog = hello_bf.as_bytes().to_vec();
    let run_res = run_func(&prog);
    let content = fs::read_to_string(tmp_filename);

    // cleanup
    drop(guard);
    match fs::remove_file(tmp_filename) {
        Ok(_) => {}
        Err(e) => {
            drop(test_lock);
            panic!("Error removing file: {}", e)
        }
    }
    drop(test_lock);

    match run_res {
        Ok(_) => {}
        Err(e) => {
            panic!("Error running program: {}", e)
        }
    };
    match content {
        Ok(c) => {
            assert_eq!(c, "Hello World!\n");
        }
        Err(e) => {
            panic!("Error reading file: {}", e)
        }
    };

    Ok(())
}

const HELLO_HELL_STR: &str = r#"[
    This routine is a demonstration of checking for the three cell sizes
    that are normal for Brainfuck. The demo code also checks for bugs
    that have been noted in various interpreters and compilers.

    It should print one of three slight variations of "Hello world" followed
    by an exclamation point then the maximum cell value (if it's less than a
    few thousand) and a newline.

    If the interpreter is broken in some way it can print a lot of other
    different strings and frequently causes the interpreter to crash.

    It does work correctly with 'bignum' cells.
]
+>>

	This code runs at pointer offset two and unknown bit width; don't
	assume you have more that eight bits

	======= DEMO CODE =======
	First just print "Hello"

	Notice that I reset the cells despite knowing that they are zero
	this is a test for proper functioning of the ability to skip over
	a loop that's never executed but isn't actually a comment loop

	Secondly there's a NOP movement between the two 'l' characters

	Also there's some commented out code afterwards

	>[-]<[-]++++++++[->+++++++++<]>.----[--<+++>]<-.+++++++.><.+++.
	[-][[-]>[-]+++++++++[<+++++>-]<+...--------------.>++++++++++[<+
	++++>-]<.+++.-------.>+++++++++[<----->-]<.-.>++++++++[<+++++++>
	-]<++.-----------.--.-----------.+++++++.----.++++++++++++++.>++
	++++++++[<----->-]<..[-]++++++++++.[-]+++++++[.,]-]

	===== END DEMO CODE =====
<<-

Calculate the value 256 and test if it's zero
If the interpreter errors on overflow this is where it'll happen
++++++++[>++++++++<-]>[<++++>-]
+<[>-<
Multiply by 256 again to get 65536
[>++++<-]>[<++++++++>-]<[>++++++++<-]
+>[>
	Cells should be 32bits at this point

	The pointer is at cell two and you can continue your code confident
	that there are big cells

	======= DEMO CODE =======
	This code rechecks that the test cells are in fact nonzero
	If the compiler notices the above is constant but doesn't
	properly wrap the values this will generate an incorrect
	string

	An optimisation barrier; unbalanced loops aren't easy
	>+[<]>-<

	Print a message
	++>[-]++++++[<+++++++>-]<.------------.[-]
	<[>+<[-]]>
	++++++++>[-]++++++++++[<+++++++++++>-]<.--------.+++.------.
	--------.[-]

	===== END DEMO CODE =====

<[-]<[-]>] <[>>
	Cells should be 16bits at this point

	The pointer is at cell two and you can continue your code confident
	that there are medium sized cells; you can use all the cells on the
	tape but it is recommended that you leave the first two alone

	If you need 32bit cells you'll have to use a BF doubler

	======= DEMO CODE =======
	Space
	++>[-]+++++[<++++++>-]<.[-]

	I'm rechecking that the cells are 16 bits
	this condition should always be true

	+>>++++[-<<[->++++<]>[-<+>]>]< + <[ >>

	    Print a message
	    >[-]++++++++++[<+++++++++++>-]<+++++++++.--------.
	    +++.------.--------.[-]

	<[-]<[-] ] >[> > Dead code here
	    This should never be executed because it's in an 8bit zone hidden
	    within a 16bit zone; a really good compiler should delete this
	    If you see this message you have dead code walking

	    Print a message
	    [-]>[-]+++++++++[<++++++++++>-]<.
	    >++++[<+++++>-]<+.--.-----------.+++++++.----.
	    [-]

	<<[-]]<
	===== END DEMO CODE =====

<<[-]] >[-]< ] >[>
	Cells should be 8bits at this point

	The pointer is at cell two but you only have 8 bits cells
	and it's time to use the really big and slow BF quad encoding

	======= DEMO CODE =======

	A broken wrapping check
	+++++[>++++<-]>[<+++++++++++++>-]<----[[-]>[-]+++++[<++++++>-]<++.
	>+++++[<+++++++>-]<.>++++++[<+++++++>-]<+++++.>++++[<---->-]<-.++.
	++++++++.------.-.[-]]

	Space
	++>[-]+++++[<++++++>-]<.[-]

	An exponent checker for github user btzy
	>++[>++<-]>[<<+>>[-<<[>++++<-]>[<++++>-]>]]<<[>++++[>---<++++]>++.
	[<++>+]<.[>+<------]>.+++.[<--->++]<--.[-]<[-]]

        Another dead code check
        [-]>[-]>[-]<++[>++++++++<-]>[<++++++++>-]<[>++++++++<-]>[<++++++++>-
        ]<[<++++++++>-]<[[-]>[-]+++++++++[<++++++++++>-]<.>++++[<+++++>-]<+.
        --.-----------.+++++++.----.>>[-]<+++++[>++++++<-]>++.<<[-]]

	Print a message
	[-] <[>+<[-]]> +++++>[-]+++++++++[<+++++++++>-]<.
	>++++[<++++++>-]<.+++.------.--------.
	[-]
	===== END DEMO CODE =====

<[-]]<

+[[>]<-]    Check unbalanced loops are ok

>>
	======= DEMO CODE =======
	Back out and print the last two characters

	[<[[<[[<[[<[,]]]<]<]<]<][ Deep nesting non-comment comment loop ]]

	Check that an offset of 128 will work
	+>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>-[+<-]

	And back
	+++[->++++++<]>[-<+++++++>]<[->>[>]+[<]<]>>[->]<<<<<<<<<<<<<<<<<<<<<
	<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

	And inside a loop
	--[>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>++<<<
	<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<+]+>----[++
	++>----]-[+<-]

	This is a simple multiply loop that looks like it goes off the
	start of the tape
	+[>]<- [-
	    <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	    <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	    <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	    <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	    <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	    <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
	    ++++
	    >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	    >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	    >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	    >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	    >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	    >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
	]

	[ Check there are enough cells. This takes 18569597 steps. ]
	[
	    >++++++[<+++>-]<+[>+++++++++<-]>+[[->+>+<<]>>
	    [-<<+>>]<[<[->>+<<]+>[->>+<<]+[>]<-]<-]<[-<]
	]

	This loop is a bug check for handling of nested loops; it goes
	round the outer loop twice and the inner loop is skipped on the
	first pass but run on the second

	BTW: It's unlikely that an optimiser will notice how this works

	>
	    +[>[
		Print the exclamation point
		[-]+++>
		[-]+++++ +#-
		[<+++2+++>-]<
		.

	    <[-]>[-]]+<]
	<

	Clean up any debris
	++++++++[[>]+[<]>-]>[>]<[[-]<]

	This is a hard optimisation barrier
	It contains several difficult to 'prove' constructions close together
	and is likely to prevent almost all forms of optimisation
	+[[>]<-[,]+[>]<-[]]

	This part finds the actual value that the cell wraps at; even
	if it's not one of the standard ones; but it gets bored after
	a few thousand: any higher and we print nothing

	This has a reasonably deep nested loop and a couple of loops
	that have unbalanced pointer movements

	Find maxint (if small)
	[-]>[-]>[-]>[-]>[-]>[-]>[-]>[-]<<<<<<<++++[->>++++>>++++>>++
	++<<<<<<]++++++++++++++>>>>+>>++<<<<<<[->>[->+>[->+>[->+>+[>
	>>+<<]>>[-<<+>]<-[<<<[-]<<[-]<<[-]<<[-]>>>[-]>>[-]>>[-]>->+]
	<<<]>[-<+>]<<<]>[-<+>]<<<]>[-<+>]<<<]>+>[[-]<->]<[->>>>>>>[-
	<<<<<<<<+>>>>>>>>]<<<<<<<]<

	The number is only printed if we found the actual maxint
	>+<[
	    Space
	    >[-]>[-]+++++[<++++++>-]<++.[-]<

	    Print the number
	    [[->>+<<]>>[-<++>[-<+>[-<+>[-<+>[-<+>[-<+>[-<+>[-<+>[-<+>[<[-]+>
	    ->+<[<-]]]]]]]]]]>]<<[>++++++[<++++++++>-]<-.[-]<]]

	]

	Check if we should have had a value but didn't
	>[
	    >[-]>[-]++++[<++++++++>-]<[<++++++++>-]>+++[<++++++++>-]<+++++++
	    [<-------->-]<------->+<[[-]>-<]>[>[-]<[-]++++[->++++++++<]>.+++
	    +++[-<++>]<.[-->+++<]>++.<++++[>----<-]>.[-]<]<

	    [-]>[-]++++++++[<++++++++>-]<[>++++<-]+>[<->[-]]<[>[-]<[-]++++[-
	    >++++++++<]>.---[-<+++>]<.---.--------------.[-->+<]>--.[-]<]
	]<

	Clean up any debris
	++++++++[[>]+[<]>-]>[>]<[[-]<]

	One last thing: an exclamation point is not a valid BF instruction!

	Print the newline
	[-]++++++++++.[-]
	[
	    Oh, and now that I can use "!" the string you see should be one of:
	    Hello World! 255
	    Hello world! 65535
	    Hello, world!

	    And it should be followed by a newline.
	]

	===== END DEMO CODE =====

<<  Finish at cell zero"#;

pub fn test_hell(run_func: &RunFn) -> Result<(), Box<dyn Error>> {
    let test_lock = TEST_MUTEX.lock().unwrap();

    let tmp_filename = "./test.tmp";

    // If the file exists, remove it
    if fs::metadata(tmp_filename).is_ok() {
        fs::remove_file(tmp_filename)?;
    }

    let guard = StdoutOverride::override_file(tmp_filename)?;
    let prog = HELLO_HELL_STR.as_bytes().to_vec();
    let run_res = run_func(&prog);
    let content = fs::read_to_string(tmp_filename);

    // cleanup
    drop(guard);
    match fs::remove_file(tmp_filename) {
        Ok(_) => {}
        Err(e) => {
            drop(test_lock);
            panic!("Error removing file: {}", e)
        }
    }
    drop(test_lock);

    match run_res {
        Ok(_) => {}
        Err(e) => {
            panic!("Error running program: {}", e)
        }
    };

    print!("Test passed\n");

    match content {
        Ok(c) => {
            // assert_eq!(c, "Hello World!\n");
            print!("{}", c);
            match c.as_str() {
                "Hello World! 255\n" => {
                    println!("Test passed with: Hello World! 255\n");
                }
                "Hello world! 65535\n" => {
                    println!("Test passed with: Hello world! 65535\n");
                }
                "Hello, world!\n" => {
                    println!("Test passed with: Hello, world!\n");
                }
                _ => {
                    panic!("Test failed, content: {}", c);
                }
            }
        }
        Err(e) => {
            panic!("Error reading file: {}", e)
        }
    };

    Ok(())
}
