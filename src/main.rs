use std::io::stdin;
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();

        match command.trim() {
            "exit" => break,
            command if command.starts_with("echo") => {
                let message = &command[5..];
                println!("{}", message);
            }
            command => println!("{}: command not found", command),
        }
    }
}
