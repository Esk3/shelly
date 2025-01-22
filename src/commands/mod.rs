use std::fmt::Debug;
use std::path::PathBuf;

use crate::exit::ExitCode;
use crate::shell::{ByteRequest, State};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("invalid input")]
    InvalidInput,
}

pub mod cd;
pub mod cmd_type;
pub mod echo;
pub mod exit;
mod into_text_command;
pub mod pwd;

#[cfg(test)]
mod tests;

pub trait Command {
    fn name(&self) -> &'static str;
    fn call(&mut self, request: ByteRequest, state: &State) -> Result<Response, Error>;
}

pub type ShellCommand = Box<dyn Command>;

pub struct ShellCommands(Vec<ShellCommand>);

impl ShellCommands {
    #[must_use]
    pub fn new(commands: Vec<ShellCommand>) -> Self {
        Self(commands)
    }
    pub fn add<C>(&mut self, command: C) -> &mut Self
    where
        C: Command + 'static + Debug,
    {
        self.0.push(Box::new(command));
        self
    }

    pub fn find_handler(
        &mut self,
        request: &ByteRequest,
    ) -> Result<&mut ShellCommand, RouterError> {
        self.0
            .iter_mut()
            .find(|cmd| {
                cmd.name().to_lowercase().as_bytes() == request.command.to_ascii_lowercase()
            })
            .ok_or(RouterError::NotFound(request.command.clone()))
    }

    fn all_names(&self) -> Vec<&'static str> {
        self.0.iter().map(|cmd| cmd.name()).collect()
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum RouterError {
    #[error("{}: command not found", String::from_utf8(.0.clone()).unwrap())]
    NotFound(Vec<u8>),
}

impl Default for ShellCommands {
    fn default() -> Self {
        let mut this = Self::new(Vec::default());
        this.add(cd::Cd)
            .add(pwd::Pwd)
            .add(echo::Echo)
            .add(exit::Exit);
        let all = this.all_names();
        let fs = crate::fs::OsFileSystem;
        // TODO
        #[cfg(test)]
        let fs = crate::fs::tests::MockFs::empty();
        this.add(cmd_type::CmdType::new(all, fs));
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
    ChangeCwd(PathBuf),
    Exit(ExitCode),
}
