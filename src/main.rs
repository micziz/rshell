use std::io::stdin;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        // use `>` character as prompt
        // need to explicitly flush this to ensure prompt prints before read_line
        print!("> ");
        std::io::stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        // everything after the first whitespace character 
        //     is interpreted as args to the command
        // read_line leaves a trailing newline, which trim removes
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = std::env::set_current_dir(root) {
                    eprintln!("{}", e);
                }
            },
            command => { 
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .unwrap();

                child.wait();
            }
        }
    
    }

}
