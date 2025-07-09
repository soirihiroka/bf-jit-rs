use std::{fs::File, io::Write};

pub fn transpile_to_string(bf_src: &[u8]) -> String {
    let mut c_program = String::new();

    // C program header
    c_program.push_str("#include <stdio.h>\n");
    c_program.push_str("int main() {\n");
    c_program.push_str("    char array[30000] = {0};\n");
    c_program.push_str("    char *ptr = array;\n");

    // Convert Brainfuck instructions to C
    for &command in bf_src {
        match command {
            b'>' => c_program.push_str("    ++ptr;\n"),
            b'<' => c_program.push_str("    --ptr;\n"),
            b'+' => c_program.push_str("    ++*ptr;\n"),
            b'-' => c_program.push_str("    --*ptr;\n"),
            b'.' => c_program.push_str("    putchar(*ptr);\n"),
            b',' => c_program.push_str("    *ptr = getchar();\n"),
            b'[' => c_program.push_str("    while (*ptr) {\n"),
            b']' => c_program.push_str("    }\n"),
            _ => (), // Ignore other characters (comments or invalid)
        }
    }

    // C program footer
    c_program.push_str("    return 0;\n");
    c_program.push_str("}\n");

    c_program
}

pub fn transpile_to_file(bf_src: &[u8], filename: &str) -> std::io::Result<()> {
    let c_program = transpile_to_string(bf_src);
    let mut file = File::create(filename)?;
    file.write_all(c_program.as_bytes())?;
    Ok(())
}
