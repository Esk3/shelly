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
    type Request = ByteRequest;
    type Response = Response;
    type Error = Error;
    type State = State;

    fn name(&self) -> &'static str {
        "cd"
    }

    fn call(
        &mut self,
        request: Self::Request,
        _: &Self::State,
    ) -> Result<Self::Response, Self::Error> {
        let request = TextRequest::try_from(request).unwrap();
        let arg = request.args[0].clone();
        let event = Event::ChangeCwd(arg.into());
        Ok(Self::Response {
            message: None,
            event: Some(vec![event]),
        })
    }
}

//fn cd_handler(input: ShellInput) -> ShellOutput {
//    let cwd = &input.state.cwd;
//    let dir = input.input.into_iter().nth(1).unwrap();
//    if dir == "~" {
//        input.state.cwd = ["home", &input.state.env_home].iter().collect();
//        return ShellOutput::default();
//    }
//    let Ok(path) = cwd.join(&dir).canonicalize() else {
//        return ShellOutput(vec![ShellCommand::Print(format!(
//            "cd: {dir}: No such file or directory"
//        ))]);
//    };
//    match path.try_exists() {
//        Ok(true) => {
//            input.state.cwd = path;
//            ShellOutput::default()
//        }
//        _ => ShellOutput(vec![ShellCommand::Print(format!(
//            "cd: {dir}: No such file or directory"
//        ))]),
//    }
//}
