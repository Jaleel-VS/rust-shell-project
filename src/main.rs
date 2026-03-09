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

        if command.trim() == "exit" {
            return;
        }

        println!("{}: command not found", command.trim());
    }
}
