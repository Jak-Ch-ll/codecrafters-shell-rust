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
            _ => (),
        }

        println!("{}: command not found", command.trim());
    }
}
