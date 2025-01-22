use super::*;

pub struct MockFs {
    files: Vec<std::path::PathBuf>,
    dirs: Vec<std::path::PathBuf>,
}

impl MockFs {
    #[must_use]
    pub fn new(files: Vec<std::path::PathBuf>, dirs: Vec<std::path::PathBuf>) -> Self {
        Self { files, dirs }
    }
}

impl FileSystem for MockFs {
    fn find_file<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>,
    {
        if self.files.iter().any(|p| p.as_path() == path.as_ref()) {
            Some(path.as_ref().to_path_buf())
        } else {
            None
        }
    }

    fn find_file_in_default_path<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>,
    {
        todo!()
    }

    fn find_dir<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>,
    {
        if self.dirs.iter().any(|p| p.as_path() == path.as_ref()) {
            Some(path.as_ref().to_path_buf())
        } else {
            None
        }
    }

    fn canonicalize<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>,
    {
        Some(path.as_ref().into())
    }
}
