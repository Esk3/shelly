use crate::{io::Escaper, shell::ByteRequest};

#[derive(Debug)]
pub struct Input<T> {
    pub value: T,
    pub bytes_read: usize,
}

impl<T> Input<T> {
    pub fn new(value: impl Into<T>, bytes_read: usize) -> Self {
        Self {
            value: value.into(),
            bytes_read,
        }
    }
}

pub type InputBytes = Input<Vec<u8>>;
pub type InputString = Input<String>;

impl InputString {
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn new_string(string: impl ToString, bytes_read: usize) -> Self {
        Self::new(string.to_string(), bytes_read)
    }
}

impl From<InputBytes> for ByteRequest {
    fn from(value: InputBytes) -> Self {
        let mut e = Escaper::new(value.value);
        let command = e.next().unwrap_or_default();
        let args = e.collect::<Vec<_>>();
        ByteRequest::new(command, args)
    }
}
