use crate::{
    commands::Event,
    shell::{ByteRequest, TextRequest},
};

use super::{Command, Error, Response, State};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Cd;

impl Command for Cd {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn call(&mut self, request: ByteRequest, _: &State) -> Result<Response, Error> {
        let request = TextRequest::try_from(request).unwrap();
        let arg = request.args.into_iter().next().filter(|s| s != "~").map_or(
            // TODO: state.home ?
            std::env::var("HOME").unwrap().into(),
            std::convert::Into::into,
        );
        let event = Event::ChangeCwd(arg);
        Ok(Response::new_event(event))
    }
}
