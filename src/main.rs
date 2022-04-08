// rShell
// rShell is a simple shell written in Rust.
// it fully supports the POSIX shell commands.
// it's also a scripting language.


// Import required libraries
use std::io::stdin;
use std::io::Write;
use std::process::Command;

// Main function
fn main() {
    loop {
        // Print the prompt
        print!("rShell> ");
        // Flush the output buffer
        std::io::stdout().flush();

        // Get user input
        let mut input = String::new();
        // Read user input
        stdin().read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        
        // Remove the newline character
        // Split the input into parts
        let mut parts = input.trim().split_whitespace();
        // Get the command
        let command = parts.next().unwrap();
        // Get the arguments
        let args = parts;


    
    
        // Run the command in the shell
        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .expect("Failed to run command");

        // do not acccept other commands until the current command is finished
        child.wait()
            .expect("Failed to wait for command");
    }

}