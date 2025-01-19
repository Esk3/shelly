use std::path::PathBuf;

use crate::{
    commands::{self, Event, RouterError, ShellCommands},
    exit::ExitCode,
};

#[cfg(test)]
mod tests;

pub struct Shell {
    data: State,
    commands: ShellCommands,
}

#[derive(thiserror::Error, Debug)]
pub enum HandlerError {
    #[error("router error: {0}")]
    Router(#[from] RouterError),
    #[error("command error: {0}")]
    Command(#[from] commands::Error),
}

impl Shell {
    #[must_use]
    pub fn new(data: State, commands: ShellCommands) -> Self {
        Self { data, commands }
    }

    pub fn handle_request(&mut self, request: Request) -> Result<Response, HandlerError> {
        let handler = self.commands.find_handler(&request)?;
        let response = handler.call(request, &self.data);
        self.hande_response(response)
    }

    #[must_use]
    pub fn new_line(&self) -> String {
        "$ ".to_string()
    }

    fn hande_response(
        &mut self,
        response: Result<crate::commands::Response, commands::Error>,
    ) -> Result<Response, HandlerError> {
        let response = response?;
        if let Some(response) = self.handle_events(response.event).unwrap() {
            return Ok(response);
        }
        let message = response.message.unwrap_or("no reply".to_string());
        Ok(Response::Message(message))
    }

    fn handle_events(&mut self, events: Option<Vec<Event>>) -> Result<Option<Response>, ()> {
        let Some(events) = events else {
            return Ok(None);
        };
        for event in events {
            let res = self.handle_event(event)?;
            if !matches!(res, Response::None) {
                return Ok(Some(res));
            }
        }
        Ok(None)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn handle_event(&mut self, event: Event) -> Result<Response, ()> {
        let res = match event {
            Event::ChangeCwd(_) => todo!(),
            Event::SetCwd(dir) => {
                self.data.cwd = PathBuf::from(dir);
                Response::None
            }
            Event::Exit(exit_code) => Response::Exit(exit_code),
        };
        Ok(res)
    }
}

#[cfg(test)]
impl Default for Shell {
    fn default() -> Self {
        Self {
            data: State::dummy(),
            commands: ShellCommands::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub command: String,
    pub args: Vec<String>,
}

impl Request {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(command: impl ToString, args: impl Into<Vec<String>>) -> Self {
        Self {
            command: command.to_string(),
            args: args.into(),
        }
    }
    pub fn empty(command: impl ToString) -> Self {
        Self::new(command, Vec::new())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Response {
    None,
    Message(String),
    Exit(ExitCode),
}

impl Response {
    pub fn into_message(self) -> Result<String, Self> {
        if let Self::Message(msg) = self {
            Ok(msg)
        } else {
            Err(self)
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub cwd: std::path::PathBuf,
    pub path: Vec<String>,
    pub env_data: EnvData,
}

impl State {
    pub fn new(cwd: impl Into<std::path::PathBuf>, env_data: EnvData) -> Self {
        let path = env_data
            .path_env
            .split(':')
            .map(std::string::ToString::to_string)
            .collect();
        Self {
            cwd: cwd.into(),
            env_data,
            path,
        }
    }
    #[cfg(test)]
    #[must_use]
    pub fn dummy() -> Self {
        Self::new("/home/dummy", EnvData::new("first", "second"))
    }
}

#[derive(Debug)]
pub struct EnvData {
    pub path_env: String,
    pub home_env: String,
}

impl EnvData {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(path_env: impl ToString, home_env: impl ToString) -> Self {
        Self {
            path_env: path_env.to_string(),
            home_env: home_env.to_string(),
        }
    }

    #[must_use]
    pub fn env() -> Self {
        Self {
            path_env: std::env::var("PATH").unwrap(),
            home_env: std::env::var("HOME").unwrap(),
        }
    }
}
