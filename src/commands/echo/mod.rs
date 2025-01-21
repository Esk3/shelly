use crate::shell::{ByteRequest, TextRequest};

use super::{Command, Error, Response, State};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Echo;

impl Command for Echo {
    type Request = ByteRequest;
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
        let request = TextRequest::try_from(request).unwrap();
        let echo = request.args.join(" ");

        Ok(Response::new_message(echo))
    }
}
