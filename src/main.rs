mod arguments;

use arguments::Arguments;

use std::fs::DirEntry;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::{
    env,
    ffi::OsString,
    fs,
    io::{self, Write},
    process,
};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        Program::from(command.as_str()).run();
    }
}

enum Program<'a> {
    Empty,
    Exit,

    Cd(Arguments),
    Echo(Arguments),
    Pwd,
    Type(Arguments),
    External(&'a str, Arguments),
}

impl<'a> From<&'a str> for Program<'a> {
    fn from(value: &'a str) -> Self {
        let (command, arguments) = value.trim().split_once(' ').unwrap_or((value.trim(), ""));

        match command {
            "" => Self::Empty,
            "exit" => Self::Exit,

            "cd" => Self::Cd(arguments.into()),
            "echo" => Self::Echo(arguments.into()),
            "pwd" => Self::Pwd,
            "type" => Self::Type(arguments.into()),
            unknown => Self::External(unknown.trim(), arguments.into()),
        }
    }
}

impl<'a> Program<'a> {
    fn new(value: &'a str) -> Self {
        Self::from(value)
    }

    fn run(&self) {
        match self {
            Program::Empty => (),
            Program::Exit => process::exit(0),

            Program::Cd(path) => run_cd_command(path),
            Program::Echo(arguments) => println!("{}", arguments),
            Program::Pwd => run_pwd_command(),
            Program::Type(command) => run_type_command(command),
            Program::External(command, arguments) => run_external_command(command, arguments),
        }
    }
}

fn run_cd_command(args: &Arguments) {
    let mut path = PathBuf::from(args.as_slice().first().unwrap());

    if let Ok(tail) = path.strip_prefix("~") {
        if let Some(mut home) = env::home_dir() {
            home.push(tail);
            path = home;
        } else {
            println!("cd: {}: No home directory found", path.display())
        }
    }

    if let Err(error) = env::set_current_dir(&path) {
        match error.kind() {
            io::ErrorKind::NotFound => {
                println!("cd: {}: No such file or directory", path.display())
            }
            _ => println!("cd: {}", error.to_string()),
        }
    }
}

fn run_pwd_command() {
    let dir = env::current_dir();

    match dir {
        Ok(dir) => println!("{}", dir.display()),
        Err(err) => println!("{}", err.to_string()),
    }
}

fn run_type_command(arguments: &Arguments) {
    match Program::new(arguments.as_slice().first().unwrap()) {
        Program::External(command, _) => {
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
        _ => println!("{} is a shell builtin", arguments),
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

fn run_external_command(command: &str, arguments: &Arguments) {
    let status = process::Command::new(command)
        .args(arguments.as_slice())
        .status();

    if let Err(error) = status {
        match error.kind() {
            io::ErrorKind::NotFound => println!("{}: command not found", command),
            io::ErrorKind::PermissionDenied => println!("{}: permission denied", command),
            _ => println!("{}: {}", command, error.to_string()),
        }
    }
}
