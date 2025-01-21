use super::*;

pub struct MockOutput<I>(I);

impl<'a, I> OutputEvents for MockOutput<I>
where
    I: Iterator<Item = Event<'a>>,
{
    fn output(&mut self, output: Event) -> std::io::Result<()> {
        assert_eq!(Some(output), self.0.next());
        Ok(())
    }
}

impl<'a, I> From<I> for MockOutput<I::IntoIter>
where
    I: IntoIterator<Item = Event<'a>>,
{
    fn from(value: I) -> Self {
        Self(value.into_iter())
    }
}

pub struct SinkOutput;

impl OutputEvents for SinkOutput {
    fn output(&mut self, _: Event) -> std::io::Result<()> {
        Ok(())
    }
}
