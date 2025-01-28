use crate::{exit::ExitCode, shell::TextRequest};

use super::{into_text_command::TextCommand, Error, Event, Response, State};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Exit;

impl TextCommand for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn call(&mut self, request: TextRequest, _: &State) -> Result<Response, Error> {
        let code = request.args.first().map_or(0, |code| code.parse().unwrap());
        let code = ExitCode::from(code);
        Ok(Response::new_event(Event::Exit(code)))
    }
}
