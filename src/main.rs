mod arguments;
mod program;

use crate::program::Program;

use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        Program::from(command.as_str()).run();
    }
}
