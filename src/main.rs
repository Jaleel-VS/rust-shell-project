use regex::Regex;
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let command = {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            line
        };
        let command = command.trim();

        if let Some(output) = command.strip_prefix("echo ") {
            println!("{}", output);
        } else if command == "exit" {
            return;
        } else {
            println!("{}: command not found", command);
        }
    }
}
