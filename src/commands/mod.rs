use std::fmt::Debug;

use crate::{exit::ExitCode, shell::Request};

use crate::shell::State;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

pub mod cd;
pub mod echo;
pub mod exit;
pub mod pwd;

#[cfg(test)]
mod tests;

pub trait Command: Debug {
    type Request;
    type Response;
    type Error;
    type State;
    fn name(&self) -> &'static str;
    #[allow(clippy::missing_errors_doc)]
    fn call(
        &mut self,
        request: Self::Request,
        state: &Self::State,
    ) -> Result<Self::Response, Self::Error>;
}

pub type ShellCommand =
    Box<dyn Command<Request = Request, Response = Response, Error = Error, State = State>>;

pub struct ShellCommands(Vec<ShellCommand>);

impl ShellCommands {
    #[must_use]
    pub fn new(commands: Vec<ShellCommand>) -> Self {
        Self(commands)
    }
    pub fn add<C>(&mut self, command: C) -> &mut Self
    where
        C: Command<Request = Request, Response = Response, Error = Error, State = State> + 'static,
    {
        self.0.push(Box::new(command));
        self
    }

    /// # Errors
    ///
    /// This function will return an error if handler with name of command is not found
    pub fn find_handler(&mut self, request: &Request) -> Result<&mut ShellCommand, RouterError> {
        self.0
            .iter_mut()
            .find(|cmd| cmd.name().to_lowercase() == request.command.to_lowercase())
            .ok_or(RouterError::NotFound(request.command.clone()))
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum RouterError {
    #[error("command not found: {0}")]
    NotFound(String),
}

impl Default for ShellCommands {
    fn default() -> Self {
        let mut this = Self::new(Vec::default());
        this.add(cd::Cd)
            .add(pwd::Pwd)
            .add(echo::Echo)
            .add(exit::Exit);
        this
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub message: Option<String>,
    pub event: Option<Vec<Event>>,
}

impl Response {
    #[must_use]
    pub fn new(message: Option<String>, event: Option<Vec<Event>>) -> Self {
        Self { message, event }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn new_message(message: impl ToString) -> Self {
        Self::new(Some(message.to_string()), None)
    }

    #[must_use]
    pub fn new_events(events: Vec<Event>) -> Self {
        Self::new(None, Some(events))
    }

    #[must_use]
    pub fn new_event(event: Event) -> Self {
        Self::new_events(vec![event])
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    ChangeCwd(String),
    SetCwd(String),
    Exit(ExitCode),
}

//fn type_handler(input: ShellInput) -> ShellOutput {
//    let shell_commands = ["echo", "exit", "type", "cd", "pwd"];
//    if let Some(cmd) = shell_commands
//        .iter()
//        .find(|cmd| *cmd == input.input.get(1).unwrap())
//    {
//        return ShellOutput(vec![ShellCommand::Print(format!(
//            "{} is a shell builtin",
//            cmd
//        ))]);
//    }
//    if let Some(path) = input
//        .state
//        .env_paths
//        .iter()
//        .map(|path| format!("{}/{}", path, input.input.get(1).unwrap()))
//        .find(|path| std::fs::metadata(path).is_ok())
//    {
//        return ShellOutput(vec![ShellCommand::Print(format!(
//            "{} is {}",
//            input.input.get(1).unwrap(),
//            path
//        ))]);
//    }
//    ShellOutput(vec![ShellCommand::Print(format!(
//        "{}: not found",
//        input.input.get(1).unwrap()
//    ))])
//}
//
//fn execute_handler(input: ShellInput) -> ShellOutput {
//    let _output = process::Command::new(input.input.first().unwrap())
//        .args(&input.input[1..])
//        .status()
//        .unwrap();
//    ShellOutput(Vec::new())
//}
//
//fn is_valid_program(input: &ShellInput) -> bool {
//    input
//        .state
//        .env_paths
//        .iter()
//        .map(|path| format!("{}/{}", path, input.input.first().unwrap()))
//        .any(|path| std::fs::File::open(path).is_ok())
//}
