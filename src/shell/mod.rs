use std::path::PathBuf;

use crate::{
    commands::{self, Event, RouterError, ShellCommands},
    fs::FileSystem,
};

pub use data::{EnvData, State};
pub use request::{ByteRequest, Request, TextRequest};
pub use response::Response;

mod data;
mod request;
mod response;

#[cfg(test)]
mod tests;

pub struct Shell {
    data: State,
    commands: ShellCommands,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
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

    pub fn handle_request(&mut self, request: ByteRequest) -> Result<Response, HandlerError> {
        match self.commands.find_handler(&request) {
            Ok(handler) => {
                let response = handler.call(request, &self.data);
                self.hande_response(response)
            }
            Err(err) => match err {
                RouterError::NotFound(_) => {
                    let request = TextRequest::try_from(request).unwrap();
                    if let Some(_path) = self.data.fs.find_file_in_default_path(&request.command)
                    //ShellCommands::find_executable_path(&request.command, &self.data.path)
                    {
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

    fn handle_events(
        &mut self,
        events: Option<Vec<Event>>,
    ) -> Result<Option<Response>, EventError> {
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
    fn handle_event(&mut self, event: Event) -> Result<Response, EventError> {
        let res = match event {
            Event::ChangeCwd(input_path) => {
                let new_path = self.data.cwd.join(&input_path);
                let new_path = self.data.fs.canonicalize(new_path).unwrap();
                //let path = self
                //    .data
                //    .cwd
                //    .join(&input_path)
                //.canonicalize()
                //.map_err(|_| EventError::InvalidPath(input_path.clone()))?;
                if self.data.fs.find_dir(&new_path).is_none() {
                    return Err(EventError::InvalidPath(input_path));
                }
                //if !std::fs::metadata(&path).unwrap().is_dir() {
                //    return Err(EventError::InvalidPath(input_path));
                //}
                self.data.cwd = new_path;
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
            //fs: crate::fs::tests::MockFs::new(
            //    ["/abc/xyz"]
            //        .into_iter()
            //        .map(std::convert::Into::into)
            //        .collect(),
            //    [
            //        "/abc/",
            //        "/xyz",
            //        "/hello_world",
            //        "/home/other",
            //        "/home/dummy/dir/abc",
            //        "/home/dummy/dir/abc/xyz",
            //        "/home/dummy/dir/abc/xyz/hello_world",
            //    ]
            //    .into_iter()
            //    .map(std::convert::Into::into)
            //    .collect(),
            //),
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum EventError {
    #[error("cd: {0}: No such file or directory")]
    InvalidPath(PathBuf),
}
