use std::env;
#[allow(unused_imports)]
#[warn(clippy::pedantic)]
use std::io::{self, Write};

fn main() {
    let shell = Shell::new(Commands::default());
    shell.run();
}

struct Shell {
    run: bool,
    commands: Commands,
}

impl Shell {
    pub fn new(commands: Commands) -> Self {
        Self {
            run: true,
            commands,
        }
    }
    pub fn run(mut self) {
        let stdin = io::stdin();
        let mut input = String::new();
        while self.run {
            print!("$ ");
            io::stdout().flush().unwrap();
            stdin.read_line(&mut input).unwrap();

            let args = self.commands.extract_command(&input);
            let result = match args {
                Some(args) => self.commands.execute_command(args),
                None => self.commands.not_found(&input),
            };
            self.run = !result.resolve();
            input.clear();
        }
    }
}

pub struct Commands {
    commands: Vec<Box<dyn ShellCommand>>,
}

impl Commands {
    pub fn extract_command(&self, input: &str) -> Option<CommandArgs> {
        todo!()
    }
    pub fn execute_command(&self, args: CommandArgs) -> ExitState {
        todo!()
    }
    pub fn not_found(&self, input: &str) -> ExitState {
        todo!()
    }
}

impl Default for Commands {
    fn default() -> Self {
        Self {
            commands: vec![Box::new(Echo)],
        }
    }
}

pub trait ShellCommand {
    fn execute(&self, args: CommandArgs) -> ExitState;
    fn extract(&self, shell_args: &ShellArgs) -> Option<CommandArgs>;
}

pub struct ShellArgs {}
pub struct CommandArgs<'a> {
    input: Vec<String>,
    shell_args: &'a ShellArgs,
}

pub struct Echo;
impl ShellCommand for Echo {
    fn execute(&self, args: CommandArgs) -> ExitState {
        ExitState {
            code: ExitCode::Ok,
            cmd: ExitCommand::Print(args.input.into_iter().next().unwrap()),
        }
    }
    fn extract(&self, shell_args: &ShellArgs) -> Option<CommandArgs> {
        todo!()
    }
}

pub struct ExitState {
    code: ExitCode,
    cmd: ExitCommand,
}
impl ExitState {
    pub fn resolve(self) -> bool {
        match self.cmd {
            ExitCommand::None => false,
            ExitCommand::Print(s) => {
                println!("{s}");
                false
            }
            ExitCommand::Exit => {
                println!("exit {}", self.code as u32);
                true
            }
        }
    }
}
pub enum ExitCode {
    Ok = 0,
    Err = 1,
}
pub enum ExitCommand {
    None,
    Print(String),
    Exit,
}
