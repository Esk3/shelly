use std::{fmt::Debug, io::Write};

use crate::{
    io::{InputBytes, Io, Stream},
    shell::{self, Shell},
};

#[cfg(test)]
mod tests;

pub struct Handler<F, S> {
    shell: Shell<F>,
    io: Io<S>,
}

impl<F, S> Handler<F, S>
where
    F: crate::fs::FileSystem,
    S: Stream + Debug,
{
    pub fn new(shell: Shell<F>, stream: S) -> Self {
        Self {
            shell,
            io: Io::new(stream),
        }
    }

    pub fn run(mut self) {
        loop {
            self.print_prompt().unwrap();
            if let RequestResult::Break = self.handle_command() {
                break;
            }
        }
    }

    fn handle_command(&mut self) -> RequestResult {
        let input = self.read_input().unwrap();

        let result = self.handle_input(input);
        let response = match result {
            Ok(res) => res,
            Err(err) => return self.handle_input_error(err),
        };
        let response = Self::handle_shell_response(response);
        self.handle_response(response)
    }

    fn handle_input_error(&mut self, err: InputError) -> RequestResult {
        match err {
            InputError::Empty => RequestResult::Continue,
            InputError::Command(handler_error) => match handler_error {
                shell::HandlerError::Router(router_error) => {
                    self.io
                        .write_line(router_error.to_string().as_bytes())
                        .unwrap();
                    RequestResult::Continue
                }
                shell::HandlerError::Command(error) => todo!("{error:?}"),
            },
            InputError::Utf8(err) => todo!("{err:?}"),
        }
    }

    fn print_prompt(&mut self) -> std::io::Result<usize> {
        let text = self.shell.prompt();
        let buf = text.as_bytes();
        self.io.write_all(buf)?;
        self.io.flush()?;
        Ok(buf.len())
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
                let bytes = s.as_bytes();
                self.io.write_line(bytes).unwrap();
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
