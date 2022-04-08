// rShell
// rShell is a simple shell written in Rust.
// it fully supports the POSIX shell commands.
// it's also a scripting language.


// Import required libraries
use std::io::stdin;
use std::io::Write;
use std::process::Command;
use std::path::Path;

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

        // Built in commands
        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = std::env::set_current_dir(root) {
                    println!("{}", e);
                }
            },
            "exit" => return,
            command => {
                // Run the command in the shell
                let mut child = Command::new(command)
                    .args(args)
                    .spawn();
                
                // gracefully handle malformed commands
                match child {
                    Ok(mut child) => { child.wait(); }, 
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    
    

        // do not acccept other commands until the current command is finished
        // and gracefully exit if the command is not found

    }

}