#[cfg(test)]
mod tests;

pub struct Escaper {
    input: std::vec::IntoIter<u8>,
}

impl Escaper {
    pub fn new(input: impl Into<Vec<u8>>) -> Self {
        Self {
            input: input.into().into_iter(),
        }
    }
}

impl Iterator for Escaper {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut arg = Vec::new();
        while let Some(byte) = self.input.next() {
            if byte.is_ascii_whitespace() {
                if arg.is_empty() {
                    continue;
                } else {
                    break;
                }
            }
            if byte == b'\'' {
                arg.extend(SingleQuote(&mut self.input).next().unwrap());
            } else if byte == b'"' {
                arg.extend(DoubleQuote(&mut self.input).next().unwrap());
            } else {
                arg.push(byte);
            }
        }
        if arg.is_empty() {
            None
        } else {
            Some(arg)
        }
    }
}

struct SingleQuote<'a, I>(&'a mut I);

impl<I> Iterator for SingleQuote<'_, I>
where
    I: Iterator<Item = u8>,
{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.0.take_while(|b| *b != b'\'').collect::<Vec<_>>();
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

struct DoubleQuote<'a, I>(&'a mut I);

impl<I> Iterator for DoubleQuote<'_, I>
where
    I: Iterator<Item = u8>,
{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.0.take_while(|b| *b != b'"').collect::<Vec<_>>();
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}
