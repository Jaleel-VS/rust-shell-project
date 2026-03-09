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

        if command.starts_with("echo") {
            let words: Vec<&str> = Regex::new("^echo\\s").unwrap().split(command).collect();
            if words.len() == 1 {
                continue;
            }
            match words.get(1) {
                Some(output) => {
                    println!("{}", output);
                    continue;
                }
                None => print!("Invalid use of command"),
            }
        } else if command == "exit" {
            return;
        }

        println!("{}: command not found", command);
    }
}
