use std::io::{self, Write};

fn main() {
    let builtins = ["echo", "exit", "type", ""];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let command = {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            line
        };
        let command = command.trim();

        if let Some(arg) = command.strip_prefix("echo ") {
            println!("{}", arg);
        } else if command == "exit" {
            return;
        } else if let Some(arg) = command.strip_prefix("type ") {
            if builtins.contains(&arg) {
                println!("{} is a shell builtin", arg)
            } else {
                println!("{}: not found", arg)
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}
