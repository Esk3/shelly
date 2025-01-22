use std::fmt::Display;

use crate::shell::{ByteRequest, TextRequest};

use super::{Command, Error, State};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct CmdType<F> {
    builtin_names: Vec<&'static str>,
    fs: F,
}

impl<F> CmdType<F> {
    #[must_use]
    pub fn new(cmds: impl Into<Vec<&'static str>>, fs: F) -> Self {
        Self {
            builtin_names: cmds.into(),
            fs,
        }
    }

    fn is_builtin(&self, cmd: &str) -> bool
    where
        F: crate::fs::FileSystem,
    {
        self.builtin_names.contains(&cmd) || cmd == self.name()
    }

    #[must_use]
    pub fn is_executable(&self, command: &str) -> Option<String>
    where
        F: crate::fs::FileSystem,
    {
        self.fs
            .find_file_in_default_path(command)
            .map(|p| p.to_str().unwrap().to_string())
    }

    fn handle_command(&self, command: String) -> Response
    where
        F: crate::fs::FileSystem,
    {
        let kind = if self.is_builtin(&command) {
            Kind::Builtin
        } else if let Some(path) = self.is_executable(&command) {
            Kind::Executable(path)
        } else {
            Kind::NotFound
        };
        Response::new(command, kind)
    }
}

impl<F> Command for CmdType<F>
where
    F: crate::fs::FileSystem,
{
    type Request = ByteRequest;
    type Response = super::Response;
    type Error = Error;
    type State = State;

    fn name(&self) -> &'static str {
        "type"
    }

    fn call(
        &mut self,
        request: Self::Request,
        _state: &Self::State,
    ) -> Result<Self::Response, Self::Error> {
        let request = TextRequest::try_from(request).unwrap();
        let res = self.handle_command(request.args.first().unwrap().clone());
        Ok(super::Response::new_message(res.to_string()))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Response {
    command: String,
    kind: Kind,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command = &self.command;
        match &self.kind {
            Kind::Builtin => write!(f, "{command} is a shell builtin"),
            Kind::Executable(path) => write!(f, "{command} is {path}"),
            Kind::NotFound => write!(f, "{command}: not found"),
        }
    }
}

impl Response {
    #[allow(clippy::needless_pass_by_value)]
    fn new(command: impl ToString, kind: Kind) -> Self {
        Self {
            command: command.to_string(),
            kind,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Kind {
    Builtin,
    Executable(String),
    NotFound,
}
