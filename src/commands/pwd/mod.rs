#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Pwd;

impl super::Command for Pwd {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn call(
        &mut self,
        _: crate::shell::ByteRequest,
        state: &crate::shell::State,
    ) -> Result<super::Response, super::Error> {
        Ok(super::Response::new_message(state.cwd.to_str().unwrap()))
    }
}
