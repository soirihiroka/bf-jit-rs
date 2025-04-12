import sys

def clean_brainfuck(file_path):
    # Define the set of valid Brainfuck commands
    valid_commands = {'>', '<', '+', '-', '.', ',', '[', ']'}
    
    try:
        # Read the content of the file
        with open(file_path, 'r') as file:
            content = file.read()
        
        # Filter out non-command characters
        cleaned_content = ''.join(char for char in content if char in valid_commands)
        
        # Print the cleaned Brainfuck code
        print(cleaned_content)
    
    except FileNotFoundError:
        print(f"Error: The file '{file_path}' was not found.")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == '__main__':
    # Check if the script received exactly one argument
    if len(sys.argv) != 2:
        print("Usage: python clean_brainfuck.py <file_path>")
    else:
        # Get the file path from the argument
        file_path = sys.argv[1]
        # Clean the Brainfuck source code
        clean_brainfuck(file_path)
