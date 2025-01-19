use std::fmt::Display;

use super::{Command, Error, Request, State};

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

    fn is_executable(&self) -> bool {
        todo!()
    }

    fn handle_command(&self, command: String) -> Response {
        let kind = if self.is_builtin(&command) {
            Kind::Builtin
        } else {
            Kind::NotFound
        };
        Response::new(command, kind)
    }
}

impl Command for CmdType {
    type Request = Request;
    type Response = super::Response;
    type Error = Error;
    type State = State;

    fn name(&self) -> &'static str {
        "type"
    }

    fn call(
        &mut self,
        request: Self::Request,
        _: &Self::State,
    ) -> Result<Self::Response, Self::Error> {
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
        match self.kind {
            Kind::Builtin => write!(f, "{command} is a shell builtin"),
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
    NotFound,
}
