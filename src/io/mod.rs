use std::{
    fmt::Debug,
    io::{BufRead, BufReader, Write},
};

pub use input::{Input, InputBytes, InputString};
pub use stdio::StdIoStream;

pub use parser::Escaper;
pub use terminal::Terminal;

pub mod input;
mod parser;
pub mod stdio;
mod terminal;

#[cfg(test)]
pub(crate) mod tests;

pub trait Stream: std::io::Read + std::io::Write {}

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

    pub fn write_line(&mut self, buf: &[u8]) -> std::io::Result<()> {
        let mut bufs = &mut [std::io::IoSlice::new(buf), std::io::IoSlice::new(b"\r\n")][..];
        let mut bytes_left = bufs.iter().map(|s| s.len()).sum::<usize>();
        while bytes_left > 0 {
            let bytes_written = self.write_vectored(bufs)?;
            std::io::IoSlice::advance_slices(&mut bufs, bytes_written);
            bytes_left -= bytes_written;
        }
        self.flush()?;
        Ok(())
    }

    /// # Safety
    ///
    /// `S` is wrapped in a `BufReader` so reading from it is not reccomended.
    /// Writing to `S` is fine
    pub unsafe fn inner(&mut self) -> &mut S {
        self.stream.get_mut()
    }

    pub fn into_inner(self) -> S {
        self.stream.into_inner()
    }
}

impl<S> std::io::Write for Io<S>
where
    S: std::io::Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.get_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.get_mut().flush()
    }
}
