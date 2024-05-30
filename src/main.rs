#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    repl();
}

fn repl() {
    let run = true;

    let stdin = io::stdin();
    let mut input = String::new();

    while run {
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();

        let command = input.trim();

        println!("{}: command not found", command);
        input.clear();
    }
}

enum Command<'a> {
    Exit(ExitCode),
    CommandNotFound(&'a str),
}

enum ExitCode {
    Ok = 0,
}

impl<'a> From<&'a str> for Command<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "exit" => Self::Exit(ExitCode::Ok),
            _ => Self::CommandNotFound(value),
        }
    }
}

impl Command {
    pub fn run(self) -> bool {}
}
