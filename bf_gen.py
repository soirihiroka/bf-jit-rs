import sys

def string_to_brainfuck(s):
    brainfuck_code = ""
    current_value = 0
    
    for char in s:
        target_value = ord(char)
        diff = target_value - current_value

        if diff > 0:
            brainfuck_code += '+' * diff
        elif diff < 0:
            brainfuck_code += '-' * (-diff)

        brainfuck_code += '.'
        current_value = target_value
    
    # brainfuck_code += "[-]."
    return brainfuck_code

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <string>")
        sys.exit(1)

    input_string = sys.argv[1]
    input_string = eval(f"\"{input_string}\"")
    brainfuck_code = string_to_brainfuck(input_string)

    with open("res.bf", "w") as f:
        f.write(brainfuck_code)
    
    print(f"Brainfuck code written to res.bf")

