use std::env;
#[allow(unused_imports)]
//#[warn(clippy::pedantic)]
use std::io::{self, Write};

fn main() {
    let args = ShellArgs {
        path: env::var("PATH").unwrap(),
    };
    let shell = Shell::new(Commands::default(), args);
    shell.run();
}

struct Shell {
    run: bool,
    commands: Commands,
    args: ShellArgs,
}

impl Shell {
    pub fn new(commands: Commands, args: ShellArgs) -> Self {
        Self {
            run: true,
            commands,
            args,
        }
    }
    pub fn run(mut self) {
        let stdin = io::stdin();
        let mut input = String::new();
        while self.run {
            print!("$ ");
            io::stdout().flush().unwrap();
            stdin.read_line(&mut input).unwrap();

            let args = self.commands.extract_command(&input, &self.args);
            let result = match args {
                Some(cmd) => cmd(),
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
    pub fn extract_command<'a>(
        &'a self,
        input: &str,
        shell_args: &'a ShellArgs,
    ) -> Option<Box<dyn Fn() -> ExitState + '_>> {
        self.commands
            .iter()
            .find_map(|c| c.extract(input, shell_args))
    }
    pub fn not_found(&self, input: &str) -> ExitState {
        ExitState {
            code: ExitCode::Err,
            cmd: ExitCommand::Print(format!("{} not found", input)),
        }
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
    fn extract<'a>(
        &'a self,
        input: &str,
        shell_args: &'a ShellArgs,
    ) -> Option<Box<dyn Fn() -> ExitState + '_>>;
}

#[derive(Clone)]
pub struct ShellArgs {
    path: String,
}
pub struct CommandArgs {
    input: Vec<String>,
    shell_args: ShellArgs,
}

pub struct Echo;
impl ShellCommand for Echo {
    fn execute(&self, args: CommandArgs) -> ExitState {
        ExitState {
            code: ExitCode::Ok,
            cmd: ExitCommand::Print(args.input.into_iter().next().unwrap()),
        }
    }
    fn extract<'a>(
        &'a self,
        input: &str,
        shell_args: &'a ShellArgs,
    ) -> Option<Box<dyn Fn() -> ExitState + '_>> {
        if input != "echo" {
            return None;
        }
        Some(Box::new(|| {
            self.execute(CommandArgs {
                input: Vec::new(),
                shell_args: shell_args.clone(),
            })
        }))
    }
}

pub struct Type;
impl ShellCommand for Type {
    fn execute(&self, args: CommandArgs) -> ExitState {
        let t = std::fs::metadata(format!(
            "{}/{}",
            args.shell_args.path,
            args.input.first().unwrap()
        ));
        let p = if t.is_ok() { "found" } else { "not found" };
        ExitState {
            code: ExitCode::Ok,
            cmd: ExitCommand::Print(p.to_string()),
        }
    }
    fn extract<'a>(
        &'a self,
        input: &str,
        shell_args: &'a ShellArgs,
    ) -> Option<Box<dyn Fn() -> ExitState + '_>> {
        if !input.starts_with("type ") {
            return None;
        }
        Some(Box::new(|| {
            self.execute(CommandArgs {
                input: Vec::new(),
                shell_args: shell_args.clone(),
            })
        }))
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
