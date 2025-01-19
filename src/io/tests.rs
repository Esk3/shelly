use bytes::Buf;

use super::*;

#[test]
fn read_input_reads_until_linefeed() {
    let mut io = Io::new(MockStream::new(b"hello world\r\nthis is one a new line"));
    let input = io.read_input().unwrap();
    let str = InputString::try_from(input).unwrap();
    assert_eq!(str.value, "hello world");
}

#[derive(Debug)]
pub struct MockStream {
    pub buf: Vec<u8>,
}

impl MockStream {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self { buf: bytes.into() }
    }
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }
}

impl Stream for &mut MockStream {}
impl Stream for MockStream {}

impl std::io::Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buf.reader().read(buf)
    }
}

impl std::io::Write for MockStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buf.flush()
    }
}

impl PartialEq<&[u8]> for MockStream {
    fn eq(&self, other: &&[u8]) -> bool {
        self.buf.eq(other)
    }
}
