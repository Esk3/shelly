use super::Stream;

mod input;
mod output;

#[cfg(test)]
mod tests;

pub type StdIoTerminal = Terminal<input::StdInEvents, output::StdOutEvents>;

impl StdIoTerminal {
    pub fn create(dict: crate::text::AutoComplete) -> Result<Self, ()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Terminal<I, O> {
    stdin: I,
    stdout: O,
    buf: [u8; 1024],
    i: usize,
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
            buf: [0; 1024],
            i: 0,
            autocomplete,
        }
    }

    fn read_until_newline(&mut self) -> &[u8] {
        todo!()
        //self.i = 0;
        //while let Some(event) = self.stdin.next() {
        //    let event = event.unwrap();
        //    let key = match event {
        //        termion::event::Event::Key(key) => key,
        //        termion::event::Event::Mouse(_) => todo!(),
        //        termion::event::Event::Unsupported(_) => todo!(),
        //    };
        //
        //    match self.handle_event(key) {
        //        EventResult::None => {
        //            let byte = self.read_byte(key);
        //            self.write_all(&[byte]).unwrap();
        //        }
        //        EventResult::Continue => {}
        //        EventResult::Finish => {
        //            return &self.buf[..self.i];
        //        }
        //    }
        //    self.flush().unwrap();
        //}
        //todo!()
    }

    fn handle_event(&mut self, key: termion::event::Key) -> EventResult {
        todo!()
        //match key {
        //    termion::event::Key::Backspace => {
        //        if self.i == 0 {
        //            return EventResult::Continue;
        //        }
        //        self.i -= 1;
        //        write!(
        //            self.stdout,
        //            "{}{}",
        //            termion::cursor::Left(1),
        //            termion::clear::UntilNewline
        //        )
        //        .unwrap();
        //        EventResult::Continue
        //    }
        //    termion::event::Key::Char(char) => {
        //        let byte = char as u8;
        //        match byte {
        //            b'\t' => {
        //                let command = self.buf[..self.i]
        //                    .iter()
        //                    .position(u8::is_ascii_whitespace)
        //                    .unwrap_or(self.i);
        //                let word = String::from_utf8(self.buf[..command].to_vec()).unwrap();
        //                let prefix_matching = self.autocomplete.prefix_matching(&word);
        //                let new_word = prefix_matching.first().unwrap().word().unwrap();
        //                let offset = new_word.len() - word.len();
        //                for i in 0..self.i {
        //                    self.buf[i + offset] = self.buf[i];
        //                }
        //                self.i += offset;
        //                for (i, byte) in new_word.bytes().enumerate() {
        //                    self.buf[i] = byte;
        //                }
        //                self.replace_line();
        //                EventResult::Continue
        //            }
        //            b'\r' | b'\n' => {
        //                self.read_byte(termion::event::Key::Char('\r'));
        //                self.read_byte(termion::event::Key::Char('\n'));
        //                self.write_all(b"\r\n").unwrap();
        //                EventResult::Finish
        //            }
        //            _ => EventResult::None,
        //        }
        //    }
        //    termion::event::Key::Ctrl(char) => {
        //        if char == 'c' {
        //            std::process::exit(0)
        //        } else {
        //            todo!()
        //        }
        //    }
        //    _ => todo!(),
        //}
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
        todo!()
        //let (_, y) = self.stdout.cursor_pos().unwrap();
        //write!(
        //    self.stdout,
        //    "{}{}",
        //    termion::clear::UntilNewline,
        //    termion::cursor::Goto(3, y)
        //)
        //.unwrap();
        //self.stdout.write_all(&self.buf[..self.i]).unwrap();
    }
}

enum EventResult {
    None,
    Continue,
    Finish,
}

impl<I, O> std::io::Read for Terminal<I, O>
where
    I: input::InputEvents,
    O: output::OutputEvents,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let end = buf.len();
        for i in 0..end {
            let event = match self.stdin.recive_event() {
                Ok(event) => event,
                Err(input::Error::NoEvent) => return Ok(i),
                Err(err) => todo!("{err}"),
            };
            match event {
                input::Event::Byte(byte) => buf[i] = byte,
                input::Event::Newline => {
                    buf[i] = b'\r';
                    buf[i + 1] = b'\n';
                    return Ok(i);
                }
                input::Event::Tab => todo!(),
                input::Event::Backspace => todo!(),
                input::Event::Exit => todo!(),
            }
        }
        Ok(end)
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
