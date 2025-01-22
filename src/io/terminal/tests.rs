use std::io::Read;

use crate::text::AutoComplete;

use super::*;

type MockInput<I> = Terminal<input::tests::MockInput<I>, output::tests::SinkOutput>;
#[allow(dead_code)]
type DummyTerm = Terminal<input::tests::DummyInput, output::tests::SinkOutput>;

const DICT: [&str; 3] = ["abc", "xyz", "echo"];

fn read_helper<I>(input: I) -> Vec<Vec<u8>>
where
    I: IntoIterator<Item = input::Event>,
{
    let iter = input.into_iter();
    let (min_size, _) = iter.size_hint();
    let mut buf = vec![0; min_size + 10];
    let mut term = MockInput::new(
        iter.into(),
        output::tests::SinkOutput,
        AutoComplete::new(DICT),
    );
    let mut result = Vec::new();
    loop {
        let bytes_read = term.read(&mut buf).unwrap();
        if bytes_read == 0 {
            break;
        }
        result.push(buf[..bytes_read].to_vec());
    }
    result
}

#[test]
fn read_line_from_term_until_end_of_input() {
    let output = read_helper([const { input::Event::Byte(b'a') }; 5]);
    assert_eq!(output, [[b'a'; 5]]);
}

#[test]
fn read_line_until_newline() {
    let output = read_helper([
        input::Event::Byte(b'a'),
        input::Event::Byte(b'b'),
        input::Event::Byte(b'c'),
        input::Event::Byte(b'd'),
        input::Event::Byte(b'e'),
        input::Event::Byte(b'f'),
        input::Event::Newline,
        input::Event::Byte(b'g'),
        input::Event::Byte(b'g'),
        input::Event::Byte(b'g'),
    ]);

    assert_eq!(output, [b"abcdef\r\n".to_vec(), b"ggg".to_vec()]);
}

#[test]
fn backspace_is_not_included_in_read() {
    let output = read_helper([
        input::Event::Byte(b'a'),
        input::Event::Byte(b'a'),
        input::Event::Backspace,
        input::Event::Byte(b'b'),
        input::Event::Byte(b'b'),
        input::Event::Backspace,
        input::Event::Byte(b'c'),
        input::Event::Byte(b'c'),
        input::Event::Backspace,
        input::Event::Byte(b'd'),
        input::Event::Byte(b'd'),
        input::Event::Backspace,
        input::Event::Newline,
    ]);
    assert_eq!(output, [b"abcd\r\n"]);
}

#[test]
fn handle_read_autocomplete_lone_command_test() {
    let output = read_helper([
        input::Event::Byte(b'a'),
        input::Event::Byte(b'b'),
        input::Event::Tab,
        input::Event::Newline,
    ]);
    assert_eq!(output, [b"abc\r\n"]);

    let output = read_helper([
        input::Event::Byte(b'x'),
        input::Event::Byte(b'y'),
        input::Event::Tab,
        input::Event::Newline,
    ]);
    assert_eq!(output, [b"xyz\r\n"]);
}

#[test]
fn handle_read_autocomplete_command_test() {
    let output = read_helper([
        input::Event::Byte(b'a'),
        input::Event::Byte(b'b'),
        input::Event::Byte(b' '),
        input::Event::Byte(b'h'),
        input::Event::Byte(b'i'),
        input::Event::Tab,
        input::Event::Newline,
    ]);
    assert_eq!(output, [b"abc hi\r\n"]);

    let output = read_helper([
        input::Event::Byte(b'x'),
        input::Event::Byte(b'y'),
        input::Event::Byte(b' '),
        input::Event::Byte(b'h'),
        input::Event::Byte(b'e'),
        input::Event::Byte(b'l'),
        input::Event::Byte(b'l'),
        input::Event::Byte(b'o'),
        input::Event::Tab,
        input::Event::Newline,
    ]);
    assert_eq!(output, [b"xyz hello\r\n"]);
}

#[test]
fn too_many_backspace() {
    read_helper([
        input::Event::Backspace,
        input::Event::Backspace,
        input::Event::Byte(b'a'),
        input::Event::Backspace,
        input::Event::Backspace,
    ]);
}
