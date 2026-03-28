mod arguments;
mod program;

use crate::{arguments::check_for_balanced_quotes, program::Program};

use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        loop {
            io::stdin().read_line(&mut command).unwrap();
            // break;
            if check_for_balanced_quotes(&command) {
                break;
            }
            // let quote_count = command.chars().filter(|char| *char == '\'').count();
            // if quote_count.is_multiple_of(2) {
            //     break;
            // }
        }

        Program::from(command.as_str()).run();
    }
}

// fn has_balanced_quotes(input: &str) -> bool {
//     input.find(['\'', '"']);
// }
