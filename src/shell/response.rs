use crate::exit::ExitCode;

#[derive(Debug, PartialEq, Eq)]
pub enum Response {
    None,
    Message(String),
    Exit(ExitCode),
}

impl Response {
    pub fn into_message(self) -> Result<String, Self> {
        if let Self::Message(msg) = self {
            Ok(msg)
        } else {
            Err(self)
        }
    }
}
