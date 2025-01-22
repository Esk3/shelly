use crate::commands::{self, Event, RouterError, ShellCommands};

pub use data::{EnvData, State};
pub use request::{ByteRequest, Request, TextRequest};
pub use response::Response;

mod data;
mod event;
mod request;
mod response;

#[cfg(test)]
mod tests;

pub type OsShell = Shell<crate::fs::OsFileSystem>;

pub struct Shell<F> {
    data: State,
    commands: ShellCommands,
    fs: F,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum HandlerError {
    #[error("router error: {0}")]
    Router(#[from] RouterError),
    #[error("command error: {0}")]
    Command(#[from] commands::Error),
}

impl<F> Shell<F>
where
    F: crate::fs::FileSystem,
{
    #[must_use]
    pub fn new(data: State, commands: ShellCommands, fs: F) -> Self {
        Self { data, commands, fs }
    }

    pub fn handle_request(&mut self, request: ByteRequest) -> Result<Response, HandlerError> {
        match self.commands.find_handler(&request) {
            Ok(handler) => {
                let response = handler.call(request, &self.data);
                self.hande_response(response)
            }
            Err(err) => match err {
                RouterError::NotFound(_) => {
                    let request = TextRequest::try_from(request).unwrap();
                    if let Some(_path) = self.fs.find_file_in_default_path(&request.command) {
                        let res = std::process::Command::new(request.command)
                            .args(request.args)
                            .output()
                            .unwrap();
                        Ok(Response::Message(String::from_utf8(res.stdout).unwrap()))
                    } else {
                        Err(err.into())
                    }
                }
            },
        }
    }

    #[must_use]
    pub fn prompt(&self) -> String {
        "$ ".to_string()
    }

    fn hande_response(
        &mut self,
        response: Result<crate::commands::Response, commands::Error>,
    ) -> Result<Response, HandlerError> {
        let response = response?;
        match self.handle_events(response.event) {
            Ok(Some(response)) => return Ok(response),
            Ok(None) => (),
            Err(err) => return Ok(Response::Message(err.to_string())),
        }
        let Some(message) = response.message else {
            return Ok(Response::None);
        };
        Ok(Response::Message(message))
    }
}
