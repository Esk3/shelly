use std::io::Read;

use crate::text::AutoComplete;

use super::*;

type MockTerm<I, O> = Terminal<input::tests::MockInput<I>, output::tests::MockOutput<O>>;
type DummyTerm = Terminal<input::tests::DummyInput, output::tests::SinkOutput>;

#[test]
fn read_line_from_term_until_end_of_input() {
    let mut buf = [0; 10];
    let bytes_read = MockTerm::new(
        std::iter::repeat(input::Event::Byte(b'a')).take(5).into(),
        std::iter::empty().into(),
        AutoComplete::new(std::iter::empty::<String>()),
    )
    .read(&mut buf)
    .unwrap();
    assert_eq!(bytes_read, 5);
    assert_eq!(buf[..5], [b'a'; 5]);
    assert_eq!(buf[5..], [0; 5]);
}

#[test]
fn read_line_until_newline() {
    let mut buf = [0; 10];
    let _ = MockTerm::new(
        [
            input::Event::Byte(b'a'),
            input::Event::Byte(b'b'),
            input::Event::Byte(b'c'),
            input::Event::Byte(b'd'),
            input::Event::Byte(b'e'),
            input::Event::Byte(b'f'),
            input::Event::Newline,
        ]
        .into(),
        std::iter::empty().into(),
        AutoComplete::new(std::iter::empty::<String>()),
    )
    .read(&mut buf)
    .unwrap();
    assert_eq!(buf, *b"abcdef\r\n\0\0");
}
