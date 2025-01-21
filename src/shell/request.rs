use crate::shell_handler::InputError;

#[derive(Debug, PartialEq, Eq)]
pub struct Request<T> {
    pub command: T,
    pub args: Vec<T>,
}

impl<T> Request<T> {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(command: impl Into<T>, args: impl Into<Vec<T>>) -> Self {
        Self {
            command: command.into(),
            args: args.into(),
        }
    }
    pub fn empty(command: impl Into<T>) -> Self {
        Self::new(command, Vec::new())
    }
}

pub type ByteRequest = Request<Vec<u8>>;
pub type TextRequest = Request<String>;

impl TryFrom<ByteRequest> for TextRequest {
    type Error = InputError;

    fn try_from(value: ByteRequest) -> Result<Self, Self::Error> {
        let command = String::from_utf8(value.command)?;
        if command.is_empty() {
            return Err(InputError::Empty);
        }
        let args = value
            .args
            .into_iter()
            .map(String::from_utf8)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(command, args))
    }
}
