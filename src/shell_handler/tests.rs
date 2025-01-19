use crate::{io::tests::MockStream, shell};

type MockHandler = Handler<MockStream>;

use super::*;

#[test]
fn print_newline_writes_to_writer() {
    let mut stream = MockStream::empty();
    Handler::new(Shell::default(), &mut stream)
        .print_newline()
        .unwrap();
    assert_eq!(stream, Shell::default().new_line().as_bytes());
}

#[test]
fn read_input_returns_first_line() {
    let stream = MockStream::new(b"hey\r\nthrere");
    let input = Handler::new(Shell::default(), stream).read_input().unwrap();
    assert_eq!(input.value, b"hey");
}

#[test]
fn handle_shell_response_returns_exit() {
    for i in 0..10 {
        let res = MockHandler::handle_shell_response(shell::Response::Exit(i.into()));
        assert_eq!(res, Response::Exit(i));
    }
}

#[test]
fn exit_maps_to_break() {
    let response =
        MockHandler::new(Shell::default(), MockStream::empty()).handle_response(Response::Exit(1));
    assert_eq!(response, RequestResult::Break);
}

#[test]
#[ignore = "todo"]
fn exit_code() {
    todo!()
}

#[test]
fn handler_echos() {
    let mut stream = MockStream::empty();
    let args = ["this", "abc", "xyz"];
    for arg in args {
        let mut input = b"echo ".to_vec();
        input.extend(arg.as_bytes());
        let bytes_read = input.len();
        let res = Handler::new(Shell::default(), &mut stream)
            .handle_input(InputBytes::new(input, bytes_read))
            .unwrap();
        assert_eq!(res, shell::Response::Message(arg.to_string()));
    }
}

#[test]
fn handler_writes() {
    let args = ["this", "abc", "xyz"];
    for arg in args {
        let mut stream = MockStream::empty();
        Handler::new(Shell::default(), &mut stream)
            .handle_response(Response::Write(arg.to_string()));
        assert_eq!(stream, arg.as_bytes());
    }
}
