#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Pwd;

impl super::Command for Pwd {
    type Request = super::Request;
    type Response = super::Response;
    type Error = super::Error;
    type State = super::State;

    fn name(&self) -> &'static str {
        "pwd"
    }

    fn call(
        &mut self,
        _: Self::Request,
        state: &Self::State,
    ) -> Result<Self::Response, Self::Error> {
        Ok(Self::Response::new_message(state.cwd.to_str().unwrap()))
    }
}
