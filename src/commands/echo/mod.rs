use crate::shell::ByteRequest;

use super::{Command, Error, Response, State};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Echo;

impl Command for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn call(&mut self, request: ByteRequest, _: &State) -> Result<Response, Error> {
        let request = request
            .args
            .into_iter()
            .map(|s| std::string::String::from_utf8_lossy(&s).to_string())
            .collect::<Vec<_>>();
        let echo = request.join(" ");

        Ok(Response::new_message(echo))
    }
}
