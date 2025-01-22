use std::io::Write;

use super::Stream;

mod input;
mod output;

#[cfg(test)]
mod tests;

pub type StdIoTerminal = Terminal<input::StdInEvents, output::StdOutEvents>;

impl StdIoTerminal {
    pub fn create(autocomplete: crate::text::AutoComplete) -> std::io::Result<Self> {
        Ok(Terminal::new(
            input::StdInEvents::new(),
            output::StdOutEvents::try_from(std::io::stdout())?,
            autocomplete,
        ))
    }
}

#[derive(Debug)]
pub struct Terminal<I, O> {
    stdin: I,
    stdout: O,
    autocomplete: crate::text::AutoComplete,
}

impl<I, O> Terminal<I, O>
where
    I: input::InputEvents,
    O: output::OutputEvents,
{
    #[must_use]
    pub fn new(stdin: I, stdout: O, autocomplete: crate::text::AutoComplete) -> Self {
        Self {
            stdin,
            stdout,
            autocomplete,
        }
    }
}

impl<I, O> std::io::Read for Terminal<I, O>
where
    I: input::InputEvents,
    O: output::OutputEvents,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let end = buf.len();
        let mut i = 0;
        while i < end {
            let event = match self.stdin.recive_event() {
                Ok(event) => event,
                Err(input::Error::NoEvent) => break,
                Err(err) => todo!("{err}"),
            };
            match event {
                input::Event::Byte(byte) => {
                    buf[i] = byte;
                    self.write_all(&buf[i..=i]).unwrap();
                    i += 1;
                }
                input::Event::Newline => {
                    buf[i] = b'\r';
                    i += 1;
                    buf[i] = b'\n';
                    i += 1;
                    self.write_all(b"\r\n").unwrap();
                    break;
                }
                input::Event::Tab => {
                    let s = buf[..i].split(u8::is_ascii_whitespace).next().unwrap();
                    let info = &self
                        .autocomplete
                        .prefix_matching(std::str::from_utf8(s).unwrap())[0];
                    let word = info.word().unwrap().as_bytes();
                    let size = word.len() - s.len();
                    for i in (s.len()..i + size).rev() {
                        buf[i] = buf[i - size];
                    }
                    (0..word.len()).for_each(|i| {
                        buf[i] = word[i];
                    });
                    self.stdout
                        .output(output::Event::ClearChar(i.try_into().unwrap()))
                        .unwrap();
                    i += size;
                    self.write_all(&buf[..i]).unwrap();
                }
                input::Event::Backspace => {
                    self.stdout.output(output::Event::ClearChar(1)).unwrap();
                    i = i.saturating_sub(1);
                    continue;
                }
                input::Event::Exit => todo!(),
            }
        }
        Ok(i)
    }
}

impl<I, O> std::io::Write for Terminal<I, O>
where
    I: input::InputEvents,
    O: output::OutputEvents,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.output(output::Event::WriteBack(buf))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<I, O> Stream for Terminal<I, O>
where
    I: input::InputEvents,
    O: output::OutputEvents,
{
}
