use std::{fs::File, io::Write};

pub fn transpile_to_string(bf_src: &[u8]) -> String {
    // Initialize the JavaScript code with necessary setup
    let mut js_code = String::from(
        "const memory = new Uint8Array(30000);\n\
        let pointer = 0;\n\
        const input = [];\n\
        let output = '';\n\
        const readInput = () => input.shift().charCodeAt(0);\n\
        const writeOutput = (charCode) => { output += String.fromCharCode(charCode); };\n\
        // Brainfuck program start\n",
    );

    let mut comment_start = false;

    let mut comment_str;

    for &command in bf_src.iter() {
        let js_command: &str = match command {
            b'>' => {
                comment_start = false;
                "pointer++;\n"
            }
            b'<' => {
                comment_start = false;
                "pointer--;\n"
            }
            b'+' => {
                comment_start = false;
                "memory[pointer]++;\n"
            }
            b'-' => {
                comment_start = false;
                "memory[pointer]--;\n"
            }
            b'.' => {
                comment_start = false;
                "writeOutput(memory[pointer]);\n"
            }
            b',' => {
                comment_start = false;
                "memory[pointer] = readInput();\n"
            }
            b'[' => {
                comment_start = false;
                "while (memory[pointer] !== 0) {\n"
            }
            b']' => {
                comment_start = false;
                "}\n"
            }
            _ => match comment_start {
                true => {
                    comment_str = format!("{}", command as char);
                    &comment_str
                }
                false => {
                    comment_str = format!("//{}", command as char);
                    comment_start = true;
                    &comment_str
                }
            }, // Ignore any non-Brainfuck characters
        };
        js_code.push_str(js_command);
    }

    // Add return statement for output and close the function
    js_code.push_str("console.log(output);\n");

    // Return the transpiled JavaScript code
    js_code
}

pub fn transpile_to_file(bf_src: &[u8], filename: &str) -> std::io::Result<()> {
    let c_program = transpile_to_string(bf_src);
    let mut file = File::create(filename)?;
    file.write_all(c_program.as_bytes())?;
    Ok(())
}
