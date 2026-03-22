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
            command if command.starts_with("echo") => println!("{}", &command[5..]),
            command if command.starts_with("type") => match &command[5..] {
                c @ ("echo" | "type" | "exit") => println!("{} is a shell builtin", c),
                c => println!("{}: not found", c),
            },
            command => println!("{}: command not found", command),
        }
    }
}
