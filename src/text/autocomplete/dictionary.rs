use crate::{commands::ShellCommands, fs::FileSystem};

pub type Dictionary = ListDictionary;

pub struct ListDictionary(Vec<String>);

impl ListDictionary {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn init<F>(router: &ShellCommands, fs: &F) -> Self
    where
        F: FileSystem,
    {
        let mut this = Self::new();
        this.add_all(router.all_names());
        this
    }
    pub fn words(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(std::string::String::as_str)
    }
    pub fn add<S>(&mut self, s: S)
    where
        S: ToString,
    {
        self.0.push(s.to_string());
    }
    pub fn add_all<I, S>(&mut self, iter: I)
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.0.extend(iter.into_iter().map(|s| s.to_string()));
    }
}
