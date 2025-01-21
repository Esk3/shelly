use std::{
    fmt::Debug,
    io::{BufRead, BufReader},
};

pub use input::{Input, InputBytes, InputString};
pub use stdio::StdIoStream;

pub use parser::Escaper;

pub mod input;
mod parser;
pub mod stdio;

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

pub trait Stream: std::io::Read + std::io::Write {}
