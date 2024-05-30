#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    repl();
}

fn repl() {
    let mut run = true;

    let stdin = io::stdin();
    let mut input = String::new();

    while run {
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();

        let i = &input.split_whitespace().collect::<Vec<&str>>();
        let command_input = CommandInput(i);
        let command = Command::extract(command_input);
        run = !command.run();

        input.clear();
    }
}

struct CommandInput<'a>(&'a [&'a str]);

enum Command {
    Exit(ExitCode),
    NotFound(String),
    Echo(String),
}

impl Command {
    pub fn extract(input: CommandInput) -> Self {
        match input.0[0] {
            "exit" => Self::Exit(ExitCode::Ok),
            "echo" => Self::Echo(input.0[1..].join(" ")),
            _ => Self::NotFound(input.0.join(" ")),
        }
    }
    pub fn run(self) -> bool {
        match self {
            Self::Exit(_) => true,
            Self::Echo(echo) => {
                println!("{echo}");
                false
            }
            Self::NotFound(cmd) => {
                println!("{cmd}: command not found");
                false
            }
        }
    }
}

enum ExitCode {
    Ok = 0,
}
