use std::io::stdin;
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut command = String::new();
    stdin().read_line(&mut command).unwrap();

    println!("{}: command not found\n", command.trim());
}
