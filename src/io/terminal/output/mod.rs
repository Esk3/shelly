use std::io::Write;
use termion::{cursor::DetectCursorPos, raw::IntoRawMode};

#[cfg(test)]
pub mod tests;

pub trait OutputEvents {
    fn output(&mut self, output: Event) -> std::io::Result<()>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Event<'a> {
    WriteBack(&'a [u8]),
    WriteStart(&'a [u8]),
    ClearChar(u16),
}

pub struct StdOutEvents {
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

impl StdOutEvents {
    pub fn new(stdout: termion::raw::RawTerminal<std::io::Stdout>) -> Self {
        Self { stdout }
    }

    pub fn stdout() -> std::io::Result<Self> {
        std::io::stdout().try_into()
    }
}

impl OutputEvents for StdOutEvents {
    fn output(&mut self, output: Event) -> std::io::Result<()> {
        match output {
            Event::WriteBack(buf) => self.stdout.write_all(buf),
            Event::WriteStart(buf) => {
                let (_, y) = self.stdout.cursor_pos()?;
                write!(
                    self.stdout,
                    "{}{}",
                    termion::cursor::Goto(1, y),
                    termion::clear::UntilNewline
                )?;
                self.stdout.write_all(buf)
            }
            Event::ClearChar(amount) => {
                write!(
                    self.stdout,
                    "{}{}",
                    termion::cursor::Left(amount),
                    termion::clear::UntilNewline
                )
            }
        }?;
        self.stdout.flush()
    }
}

impl TryFrom<std::io::Stdout> for StdOutEvents {
    type Error = std::io::Error;

    fn try_from(value: std::io::Stdout) -> Result<Self, Self::Error> {
        Ok(Self::new(value.into_raw_mode()?))
    }
}

impl std::fmt::Debug for StdOutEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StdOutEvents").finish_non_exhaustive()
    }
}
