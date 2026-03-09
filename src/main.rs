#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let command = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line
    };

    print!("$ ");
    println!("{}: command not found", command.trim());
    io::stdout().flush().unwrap();
}
