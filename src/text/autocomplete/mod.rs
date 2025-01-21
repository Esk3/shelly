#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct AutoComplete {
    dict: Vec<String>,
}

impl AutoComplete {
    pub fn new(dict: impl IntoIterator<Item = impl ToString>) -> Self {
        Self {
            dict: dict.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    #[must_use]
    pub fn prefix_matching(&self, arg: &str) -> Vec<CompleteInfo<'_>> {
        let mut collect = self
            .dict
            .iter()
            .filter(|s| s.strip_prefix(arg).is_some())
            .map(|s| CompleteInfo::new_found(s, 0))
            .collect::<Vec<_>>();
        collect.sort_by_key(|info| info.word().unwrap().to_string());
        collect
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompleteInfo<'a> {
    result: Option<&'a str>,
    steps: usize,
}

impl<'a> CompleteInfo<'a> {
    fn new(result: Option<&'a str>, steps: usize) -> Self {
        Self { result, steps }
    }
    fn new_found(s: &'a str, steps: usize) -> Self {
        Self::new(Some(s), steps)
    }

    pub fn word(&self) -> Option<&str> {
        self.result
    }
}
