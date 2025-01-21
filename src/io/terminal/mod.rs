use std::io::Write;

use termion::{cursor::DetectCursorPos, input::TermRead, raw::IntoRawMode};

use super::Stream;

#[cfg(test)]
mod tests;

pub struct Terminal {
    stdin: termion::input::Events<std::io::Stdin>,
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
    buf: [u8; 1024],
    i: usize,
    autocomplete: crate::text::AutoComplete,
}

impl Terminal {
    #[must_use]
    pub fn new(
        stdin: std::io::Stdin,
        stdout: termion::raw::RawTerminal<std::io::Stdout>,
        autocomplete: crate::text::AutoComplete,
    ) -> Self {
        Self {
            stdin: stdin.events(),
            stdout,
            buf: [0; 1024],
            i: 0,
            autocomplete,
        }
    }

    pub fn create(autocomplete: crate::text::AutoComplete) -> std::io::Result<Self> {
        Ok(Self::new(
            std::io::stdin(),
            std::io::stdout().into_raw_mode()?,
            autocomplete,
        ))
    }

    fn read_until_newline(&mut self) -> &[u8] {
        self.i = 0;
        while let Some(event) = self.stdin.next() {
            let event = event.unwrap();
            let key = match event {
                termion::event::Event::Key(key) => key,
                termion::event::Event::Mouse(_) => todo!(),
                termion::event::Event::Unsupported(_) => todo!(),
            };

            match self.handle_event(key) {
                EventResult::None => {
                    let byte = self.read_byte(key);
                    self.write_all(&[byte]).unwrap();
                }
                EventResult::Continue => {}
                EventResult::Finish => {
                    return &self.buf[..self.i];
                }
            }
            self.flush().unwrap();
        }
        todo!()
    }

    fn handle_event(&mut self, key: termion::event::Key) -> EventResult {
        match key {
            termion::event::Key::Backspace => {
                self.i -= 1;
                write!(
                    self.stdout,
                    "{}{}",
                    termion::cursor::Left(1),
                    termion::clear::UntilNewline
                )
                .unwrap();
                EventResult::Continue
            }
            termion::event::Key::Char(char) => {
                let byte = char as u8;
                match byte {
                    b'\t' => {
                        let command = self.buf[..self.i]
                            .iter()
                            .position(u8::is_ascii_whitespace)
                            .unwrap_or(self.i);
                        let word = String::from_utf8(self.buf[..command].to_vec()).unwrap();
                        let prefix_matching = self.autocomplete.prefix_matching(&word);
                        let new_word = prefix_matching.first().unwrap().word().unwrap();
                        let offset = new_word.len() - word.len();
                        for i in 0..self.i {
                            self.buf[i + offset] = self.buf[i];
                        }
                        self.i += offset;
                        for (i, byte) in new_word.bytes().enumerate() {
                            self.buf[i] = byte;
                        }
                        self.replace_line();
                        EventResult::Continue
                    }
                    b'\r' | b'\n' => {
                        self.read_byte(termion::event::Key::Char('\r'));
                        self.read_byte(termion::event::Key::Char('\n'));
                        self.write_all(b"\r\n").unwrap();
                        EventResult::Finish
                    }
                    _ => EventResult::None,
                }
            }
            termion::event::Key::Ctrl(char) => {
                if char == 'c' {
                    std::process::exit(0)
                } else {
                    todo!()
                }
            }
            _ => todo!(),
        }
    }

    fn read_byte(&mut self, key: termion::event::Key) -> u8 {
        if let termion::event::Key::Char(byte) = key {
            self.buf[self.i] = byte as u8;
            self.i += 1;
            byte as u8
        } else {
            todo!()
        }
    }

    fn replace_line(&mut self) {
        let (_, y) = self.stdout.cursor_pos().unwrap();
        write!(
            self.stdout,
            "{}{}",
            termion::clear::UntilNewline,
            termion::cursor::Goto(3, y)
        )
        .unwrap();
        self.stdout.write_all(&self.buf[..self.i]).unwrap();
    }
}

enum EventResult {
    None,
    Continue,
    Finish,
}

impl std::io::Read for Terminal {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let read = self.read_until_newline();
        let size = read.len();
        for (i, byte) in read.iter().enumerate() {
            buf[i] = *byte;
        }
        Ok(size)
    }
}

impl std::io::Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}

impl Stream for Terminal {}

impl std::fmt::Debug for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Terminal")
            .field("buf", &self.buf)
            .field("i", &self.i)
            .finish_non_exhaustive()
    }
}
