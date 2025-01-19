use std::{
    fmt::Debug,
    io::{BufRead, BufReader},
    string::FromUtf8Error,
};

#[cfg(test)]
pub(crate) mod tests;

#[derive(Debug)]
pub struct Io<S> {
    stream: BufReader<S>,
}

impl<S> Io<S>
where
    S: Stream + Debug,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream: BufReader::new(stream),
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn read_input(&mut self) -> std::io::Result<InputBytes> {
        let mut buf = String::new();
        self.stream.read_line(&mut buf)?;
        Ok(InputBytes::new(buf.trim().as_bytes().to_vec(), buf.len()))
    }

    pub fn write_response(&mut self) -> std::io::Result<usize> {
        todo!()
    }

    /// # Safety
    /// reading or writing directly to stream may leave it in an invalid state for future reads
    /// which may result in the next command beeing seen as invalid and discarded
    pub unsafe fn inner(&mut self) -> &mut S {
        self.stream.get_mut()
    }

    pub fn into_inner(self) -> S {
        self.stream.into_inner()
    }
}

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
    #[must_use]
    pub fn new_string(string: String, bytes_read: usize) -> Self {
        Self::new(string, bytes_read)
    }
}

impl TryFrom<InputBytes> for InputString {
    type Error = FromUtf8Error;

    fn try_from(value: InputBytes) -> Result<Self, Self::Error> {
        let string = String::from_utf8(value.value)?;
        Ok(Self::new(string, value.bytes_read))
    }
}

pub trait Stream: std::io::Read + std::io::Write {}

#[derive(Debug)]
pub struct StdIoStream {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
}

impl StdIoStream {
    #[must_use]
    pub fn new(stdin: std::io::Stdin, stdout: std::io::Stdout) -> Self {
        Self { stdin, stdout }
    }
}

impl Stream for StdIoStream {}

impl std::io::Read for StdIoStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stdin.read(buf)
    }
}
impl std::io::Write for StdIoStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}
