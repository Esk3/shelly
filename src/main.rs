#[allow(unused_imports)]
//#[warn(clippy::pedantic)]
use std::io::{self, Write};
use std::{env, path::PathBuf, process};

fn main() {
    let config = AppConfig::new();
    let app = App::new(config);
    app.run();
}

struct App {
    pub shell_state: ShellState,
}
impl App {
    pub fn new(config: AppConfig) -> Self {
        let shell_state = ShellState::new(config);
        Self { shell_state }
    }
    pub fn run(mut self) {
        let stdin = io::stdin();
        let mut input = String::new();
        'main: loop {
            print!("$ ");
            io::stdout().flush().unwrap();
            stdin.read_line(&mut input).unwrap();
            {
                let input = ShellInput::new(input.clone(), &mut self.shell_state);
                let handler = router(&input);
                let handler = match handler {
                    Some(handler) => handler,
                    None => Box::new(Self::not_found),
                };
                let result = handler(input);
                for cmd in result.0 {
                    match cmd {
                        ShellCommand::Exit(_exit_code) => break 'main,
                        ShellCommand::Print(s) => println!("{s}"),
                    }
                }
            }
            input.clear();
        }
    }
    pub fn not_found(input: ShellInput) -> ShellOutput {
        ShellOutput(vec![ShellCommand::Print(format!(
            "{}: command not found",
            input.input.join(" ").trim_end()
        ))])
    }
}

struct AppConfig {
    path_env: String,
}
impl AppConfig {
    pub fn new() -> Self {
        Self {
            path_env: env::var("PATH").unwrap(),
        }
    }
}
struct ShellState {
    #[allow(dead_code)]
    cwd: PathBuf,
    env_paths: Vec<String>,
}
impl ShellState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            cwd: PathBuf::from("/app"),
            env_paths: config.path_env.split(':').map(String::from).collect(),
        }
    }
}
type ShellHandler = Box<dyn Fn(ShellInput) -> ShellOutput>;
struct ShellInput<'a> {
    state: &'a mut ShellState,
    input: Vec<String>,
}
impl<'a> ShellInput<'a> {
    pub fn new(input: String, state: &'a mut ShellState) -> Self {
        Self {
            input: input.split_whitespace().map(String::from).collect(),
            state,
        }
    }
}
#[derive(Default)]
struct ShellOutput(Vec<ShellCommand>);
enum ShellCommand {
    Print(String),
    Exit(u32),
}

fn router(input: &ShellInput) -> Option<ShellHandler> {
    Some(Box::new(match input.input.first()?.as_str() {
        "echo" => echo_handler,
        "exit" => exit_handler,
        "type" => type_handler,
        "cd" => cd_handler,
        "pwd" => pwd_handler,
        _ if is_valid_program(input) => execute_handler,
        _ => return None,
    }))
}

fn echo_handler(input: ShellInput) -> ShellOutput {
    ShellOutput(vec![ShellCommand::Print(input.input[1..].join(" "))])
}

fn exit_handler(_: ShellInput) -> ShellOutput {
    ShellOutput(vec![ShellCommand::Exit(0)])
}

fn type_handler(input: ShellInput) -> ShellOutput {
    let shell_commands = ["echo", "exit", "type", "cd", "pwd"];
    if let Some(cmd) = shell_commands
        .iter()
        .find(|cmd| *cmd == input.input.get(1).unwrap())
    {
        return ShellOutput(vec![ShellCommand::Print(format!(
            "{} is a shell builtin",
            cmd
        ))]);
    }
    if let Some(path) = input
        .state
        .env_paths
        .iter()
        .map(|path| format!("{}/{}", path, input.input.get(1).unwrap()))
        .find(|path| std::fs::metadata(path).is_ok())
    {
        return ShellOutput(vec![ShellCommand::Print(format!(
            "{} is {}",
            input.input.get(1).unwrap(),
            path
        ))]);
    }
    ShellOutput(vec![ShellCommand::Print(format!(
        "{}: not found",
        input.input.get(1).unwrap()
    ))])
}

fn execute_handler(input: ShellInput) -> ShellOutput {
    let _output = process::Command::new(input.input.first().unwrap())
        .args(&input.input[1..])
        .status()
        .unwrap();
    ShellOutput(Vec::new())
}
fn is_valid_program(input: &ShellInput) -> bool {
    input
        .state
        .env_paths
        .iter()
        .map(|path| format!("{}/{}", path, input.input.first().unwrap()))
        .any(|path| std::fs::File::open(path).is_ok())
}

fn cd_handler(input: ShellInput) -> ShellOutput {
    let cwd = &input.state.cwd;
    let dir = input.input.into_iter().nth(1).unwrap();
    let path = cwd.join(&dir).canonicalize().unwrap();
    match path.try_exists() {
        Ok(true) => {
            input.state.cwd = path;
            ShellOutput::default()
        }
        _ => ShellOutput(vec![ShellCommand::Print(format!(
            "cd: {dir}: No such file or directory"
        ))]),
    }
}
fn pwd_handler(input: ShellInput) -> ShellOutput {
    ShellOutput(vec![ShellCommand::Print(
        input.state.cwd.to_str().unwrap().to_string(),
    )])
}

pub struct ExitState {
    #[allow(dead_code)]
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
                //println!("exit {}", self.code as u32);
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
