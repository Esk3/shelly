use termion::input::TermRead;

#[cfg(test)]
pub mod tests;

pub trait InputEvents {
    fn recive_event(&mut self) -> Result<Event>;
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Event {
    Byte(u8),
    Newline,
    Tab,
    Backspace,
    Exit,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no event avalible")]
    NoEvent,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unexpected event: {0:?}")]
    Unhandled(termion::event::Event),
}

pub struct StdInEvents {
    stdin: termion::input::Events<std::io::Stdin>,
}

impl StdInEvents {
    pub fn new() -> Self {
        Self {
            stdin: std::io::stdin().events(),
        }
    }
}

impl InputEvents for StdInEvents {
    fn recive_event(&mut self) -> Result<Event> {
        let Some(event) = self.stdin.next() else {
            return Err(Error::NoEvent);
        };
        let event = event?;
        match event {
            termion::event::Event::Key(termion::event::Key::Char('\r' | '\n')) => {
                Ok(Event::Newline)
            }
            termion::event::Event::Key(termion::event::Key::Char('\t')) => Ok(Event::Tab),
            termion::event::Event::Key(termion::event::Key::Char(char)) => {
                Ok(Event::Byte(char as u8))
            }
            termion::event::Event::Key(termion::event::Key::Ctrl('c')) => Ok(Event::Exit),
            termion::event::Event::Key(termion::event::Key::Backspace) => Ok(Event::Backspace),
            unhandled_event => Err(Error::Unhandled(unhandled_event)),
        }
    }
}

impl From<std::io::Stdin> for StdInEvents {
    fn from(value: std::io::Stdin) -> Self {
        Self {
            stdin: value.events(),
        }
    }
}

impl std::fmt::Debug for StdInEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StdInEvents").finish_non_exhaustive()
    }
}
