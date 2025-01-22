use crate::shell::{ByteRequest, State, TextRequest};

use super::{Error, Response};

pub trait TextCommand {
    fn name(&self) -> &'static str;
    fn call(&mut self, request: TextRequest, state: &State) -> Result<Response, Error>;
}

pub struct IntoTextCommand<T>(T);

impl<T> super::Command for IntoTextCommand<T>
where
    T: TextCommand,
{
    fn name(&self) -> &'static str {
        self.0.name()
    }

    fn call(&mut self, request: ByteRequest, state: &State) -> Result<Response, Error> {
        let request = TextRequest::try_from(request).unwrap();
        self.0.call(request, state)
    }
}
