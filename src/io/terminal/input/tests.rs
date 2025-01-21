use super::*;

pub struct MockInput<I>(I);

impl<I> InputEvents for MockInput<I>
where
    I: Iterator<Item = Event>,
{
    fn recive_event(&mut self) -> Result<Event> {
        self.0.next().ok_or(Error::NoEvent)
    }
}

impl<I> From<I> for MockInput<I::IntoIter>
where
    I: IntoIterator<Item = Event>,
{
    fn from(value: I) -> Self {
        Self(value.into_iter())
    }
}

pub struct DummyInput;
impl InputEvents for DummyInput {
    fn recive_event(&mut self) -> Result<Event> {
        unimplemented!("dummy input")
    }
}
