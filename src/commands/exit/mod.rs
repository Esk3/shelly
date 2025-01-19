use crate::exit::ExitCode;

use super::{Command, Error, Event, Request, Response, State};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Exit;

impl Command for Exit {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type State = State;

    fn name(&self) -> &'static str {
        "exit"
    }

    fn call(
        &mut self,
        request: Self::Request,
        _: &Self::State,
    ) -> Result<Self::Response, Self::Error> {
        let code = request.args.first().map_or(0, |code| code.parse().unwrap());
        let code = ExitCode::from(code);
        Ok(Response::new_event(Event::Exit(code)))
    }
}
