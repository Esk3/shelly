use std::{fmt::Display, os::linux::fs::MetadataExt, path::PathBuf};

use crate::shell::{ByteRequest, TextRequest};

use super::{Command, Error, State};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct CmdType(Vec<&'static str>);

impl CmdType {
    #[must_use]
    pub fn new(cmds: impl Into<Vec<&'static str>>) -> Self {
        Self(cmds.into())
    }

    fn is_builtin(&self, cmd: &str) -> bool {
        self.0.contains(&cmd) || cmd == self.name()
    }

    #[must_use]
    pub fn is_executable(command: &str, path: &[String]) -> Option<String> {
        let is_executable = |metadata: &std::fs::Metadata| metadata.st_mode() & 0o111 != 0;
        path.iter()
            .map(|path| PathBuf::from(path).join(command))
            .flat_map(|path| std::fs::metadata(&path).map(|data| (path, data)))
            .filter(|(_, data)| std::fs::Metadata::is_file(data))
            .filter(|(_, data)| is_executable(data))
            .map(|(path, _)| path.to_str().unwrap().to_string())
            .next()
    }

    fn handle_command(&self, command: String, path: &[String]) -> Response {
        let kind = if self.is_builtin(&command) {
            Kind::Builtin
        } else if let Some(path) = Self::is_executable(&command, path) {
            Kind::Executable(path)
        } else {
            Kind::NotFound
        };
        Response::new(command, kind)
    }
}

impl Command for CmdType {
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
        state: &Self::State,
    ) -> Result<Self::Response, Self::Error> {
        let request = TextRequest::try_from(request).unwrap();
        let res = self.handle_command(request.args.first().unwrap().clone(), &state.path);
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
