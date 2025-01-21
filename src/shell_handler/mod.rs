use std::fmt::Debug;

use crate::{
    io::{InputBytes, Io, StdIoStream, Stream},
    shell::{self, Shell},
};

#[cfg(test)]
mod tests;

pub type StdHandler = Handler<StdIoStream>;

pub struct Handler<S> {
    shell: Shell,
    io: Io<S>,
}

impl<S> Handler<S>
where
    S: Stream + Debug,
{
    pub fn new(shell: Shell, stream: S) -> Self {
        Self {
            shell,
            io: Io::new(stream),
        }
    }

    pub fn run(mut self) {
        loop {
            self.print_newline().unwrap();
            if let RequestResult::Break = self.handle_command() {
                break;
            }
        }
    }

    fn handle_command(&mut self) -> RequestResult {
        let input = self.read_input().unwrap();

        let response = match self.handle_input(input) {
            Ok(res) => res,
            Err(err) => match err {
                InputError::Empty => return RequestResult::Continue,
                InputError::Command(handler_error) => match handler_error {
                    shell::HandlerError::Router(router_error) => {
                        unsafe {
                            self.io
                                .inner()
                                .write_all(router_error.to_string().as_bytes())
                                .unwrap();
                            self.io.inner().flush().unwrap();
                        }
                        unsafe {
                            self.io.inner().write_all(b"\r\n").unwrap();
                            self.io.inner().flush().unwrap();
                        }
                        return RequestResult::Continue;
                    }
                    shell::HandlerError::Command(error) => todo!("{error}"),
                },
                InputError::Utf8(_) => todo!(),
            },
        };
        let response = Self::handle_shell_response(response);
        self.handle_response(response)
    }

    fn print_newline(&mut self) -> std::io::Result<usize> {
        let text = self.shell.new_line();
        // Safety
        // writing directly to stream at the start of a new line is fine as long as we don't do a read
        unsafe {
            let bytes = self.io.inner().write(text.as_bytes())?;
            self.io.inner().flush()?;
            Ok(bytes)
        }
    }

    fn read_input(&mut self) -> std::io::Result<InputBytes> {
        self.io.read_input()
    }

    #[allow(clippy::unnecessary_wraps)]
    fn handle_input(&mut self, input: InputBytes) -> Result<crate::shell::Response, InputError> {
        let request = input.into();
        Ok(self.shell.handle_request(request)?)
    }

    fn handle_shell_response(response: crate::shell::Response) -> Response {
        match response {
            crate::shell::Response::None => Response::None,
            crate::shell::Response::Message(s) => Response::Write(s),
            crate::shell::Response::Exit(exit_code) => Response::Exit(exit_code.into()),
        }
    }

    fn handle_response(&mut self, response: Response) -> RequestResult {
        match response {
            Response::Write(s) => {
                // TODO
                // Safety
                // writing response directly to stream is ok but should be replaced by proper
                // method
                unsafe {
                    self.io.inner().write_all(s.as_bytes()).unwrap();
                    self.io.inner().flush().unwrap();
                };
                unsafe {
                    self.io.inner().write_all(b"\r\n").unwrap();
                    self.io.inner().flush().unwrap();
                }
                RequestResult::Continue
            }
            Response::Exit(code) => {
                std::process::exit(code.try_into().unwrap());
            }
            Response::None => RequestResult::Continue,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Response {
    None,
    Write(String),
    Exit(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RequestResult {
    Continue,
    Break,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum InputError {
    #[error("input was empty")]
    Empty,
    #[error("command error: {0}")]
    Command(#[from] shell::HandlerError),
    #[error("invalid utf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}
