use std::io::{self, Write};

fn main() {
    let commands = vec!["echo", "exit", "type", ""];
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
        } else if let Some(output) = command.strip_prefix("type ") {
            if commands.contains(&output) {
                println!("{}: is a shell builtin", output)
            } else {
                println!("{}: not found", output)
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}
