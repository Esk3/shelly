use bytes::Buf;

use crate::{
    shell::{ByteRequest, Request, TextRequest},
    shell_handler::InputError,
};

use super::*;

#[test]
fn read_input_reads_until_linefeed() {
    //let mut io = Io::new(MockStream::new(b"hello world\r\nthis is one a new line"));
    //let input = io.read_input().unwrap();
    //let str = InputString::try_from(input).unwrap();
    //assert_eq!(str.value, "hello world");
}

fn try_into_request_test(s: &str) -> Result<TextRequest, InputError> {
    let input = InputBytes::new(s, 1);
    let request = ByteRequest::from(input);
    TextRequest::try_from(request)
}

#[test]
#[ignore = "todo"]
fn empty_input_string_returns_error() {
    let err = try_into_request_test("").unwrap_err();
    assert_eq!(err, InputError::Empty);
}

#[test]
fn splits_on_whitespace_where_first_is_command_and_rest_in_args() {
    let req = try_into_request_test("hello world 123").unwrap();
    assert_eq!(
        req,
        Request::new(
            "hello",
            ["world", "123"].map(std::string::ToString::to_string)
        )
    );
}

#[test]
fn single_quote_gets_goruped_into_single_arg() {
    let req = try_into_request_test("hello 'something with spaces' next args").unwrap();
    assert_eq!(
        req,
        Request::new(
            "hello",
            ["something with spaces", "next", "args"].map(std::string::ToString::to_string)
        )
    );
}

#[test]
fn handles_multiple_whitespace() {
    let args = ["hello world", "hello  world", "hello        world"];
    for arg in args {
        assert_eq!(
            try_into_request_test(arg).unwrap(),
            Request::new("hello", ["world".to_string()]),
            "{arg}"
        );
    }
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
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
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
