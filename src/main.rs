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
        println!("{}: command not found", command.trim());
    }
}
