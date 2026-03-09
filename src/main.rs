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

        if command == "exit" {
            return;
        }

        println!("{}: command not found", command);
    }
}
