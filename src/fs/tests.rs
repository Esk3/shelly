use super::*;

#[derive(Debug)]
pub struct MockFs {
    files: Vec<std::path::PathBuf>,
    dirs: Vec<std::path::PathBuf>,
}

impl MockFs {
    #[must_use]
    pub fn new(files: Vec<std::path::PathBuf>, dirs: Vec<std::path::PathBuf>) -> Self {
        Self { files, dirs }
    }
    #[must_use]
    pub fn empty() -> Self {
        Self::new([].to_vec(), [].to_vec())
    }
}

impl FileSystem for MockFs {
    fn is_file<P>(&self, path: P) -> bool
    where
        P: AsRef<std::path::Path>,
    {
        self.files.iter().any(|f| f == path.as_ref())
    }

    fn is_dir<P>(&self, path: P) -> bool
    where
        P: AsRef<std::path::Path>,
    {
        self.dirs.iter().any(|dir| dir == path.as_ref())
    }

    fn read_dir<P>(&self, path: P) -> impl Iterator<Item = DirEntry>
    where
        P: AsRef<std::path::Path>,
    {
        self.files
            .iter()
            .filter(|f| f.starts_with(&path))
            .map(|f| {
                DirEntry::new(
                    path.as_ref().to_path_buf(),
                    f.file_name().unwrap().to_str().unwrap().to_string(),
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn canonicalize<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>,
    {
        Some(path.as_ref().into())
    }
}
