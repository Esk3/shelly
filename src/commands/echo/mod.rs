use super::{Command, Error, Request, Response, State};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Echo;

impl Command for Echo {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type State = State;

    fn name(&self) -> &'static str {
        "echo"
    }

    fn call(
        &mut self,
        request: Self::Request,
        _: &Self::State,
    ) -> Result<Self::Response, Self::Error> {
        let echo = request.args.first().unwrap().clone();

        Ok(Response::new_message(echo))
    }
}
