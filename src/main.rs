use std::fs::DirEntry;
use std::os::unix::fs::MetadataExt;
use std::{
    env,
    ffi::OsString,
    fmt::Display,
    fs,
    io::{self, Write},
};

enum Command {
    Exit,
    Echo(String),
    Type(String),
    Unknown(String),
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        let (command, arguments) = value.trim().split_once(' ').unwrap_or((value.trim(), ""));

        match command {
            "exit" => Self::Exit,
            "echo" => Self::Echo(arguments.trim().into()),
            "type" => Self::Type(arguments.trim().into()),
            unknown => Self::Unknown(unknown.trim().into()),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Command::Exit => "exit",
                Command::Echo(_) => "echo",
                Command::Type(_) => "type",
                Command::Unknown(_) => "unknown",
            }
        )
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        match Command::from(command) {
            Command::Exit => break,
            Command::Echo(arguments) => println!("{}", arguments),
            Command::Type(command) => run_type_command(command),
            Command::Unknown(command) => println!("{}: command not found", command),
        }
    }
}

fn run_type_command(arguments: String) {
    match arguments.into() {
        Command::Unknown(command) => {
            let path = env::var_os("PATH").unwrap();
            let file = env::split_paths(&path)
                .flat_map(fs::read_dir)
                .flatten()
                .flatten()
                .filter(is_executable)
                .find(|entry| entry.file_name() == OsString::from(&command));

            match file {
                Some(file) => println!("{} is {}", command, file.path().display()),
                None => println!("{}: not found", command),
            }
        }
        command => println!("{} is a shell builtin", command),
    }
}

fn is_executable(dir_entry: &DirEntry) -> bool {
    match dir_entry.metadata() {
        Ok(metadata) => {
            let is_executable = metadata.mode() & 0o111 > 0;
            is_executable && !metadata.is_dir()
        }
        Err(_) => false,
    }
}
