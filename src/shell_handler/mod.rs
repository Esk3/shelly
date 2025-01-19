use std::fmt::Debug;

use crate::{
    io::{InputBytes, InputString, Io, StdIoStream, Stream},
    shell::{Request, Shell},
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
        while let RequestResult::Continue = self.handle_command() {
            // loop
        }
    }

    fn handle_command(&mut self) -> RequestResult {
        self.print_newline().unwrap();
        let input = self.read_input().unwrap();
        let response = self.handle_input(input).unwrap();
        let response = Self::handle_shell_response(response);
        self.handle_response(response)
    }

    fn print_newline(&mut self) -> std::io::Result<usize> {
        let text = self.shell.new_line();
        // Safety
        // writing directly to stream at the start of a new line is fine as long as we don't do a read
        unsafe { self.io.inner().write(text.as_bytes()) }
    }

    fn read_input(&mut self) -> std::io::Result<InputBytes> {
        self.io.read_input()
    }

    #[allow(clippy::unnecessary_wraps)]
    fn handle_input(&mut self, input: InputBytes) -> Result<crate::shell::Response, ()> {
        let s = InputString::try_from(input).unwrap().value;
        let mut iter = s.split_whitespace().map(std::string::ToString::to_string);
        let request = Request::new(iter.next().unwrap(), iter.collect::<Vec<_>>());
        Ok(self.shell.handle_request(request).unwrap())
    }

    fn handle_shell_response(response: crate::shell::Response) -> Response {
        match response {
            crate::shell::Response::None => todo!(),
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
                unsafe { self.io.inner().write_all(s.as_bytes()) }.unwrap();
                unsafe { self.io.inner().flush() }.unwrap();
                RequestResult::Continue
            }
            Response::Exit(_) => RequestResult::Break,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Response {
    Write(String),
    Exit(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum RequestResult {
    Continue,
    Break,
}
