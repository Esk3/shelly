#[cfg(test)]
pub mod tests;

pub trait FileSystem {
    fn is_file<P>(&self, path: P) -> bool
    where
        P: AsRef<std::path::Path>;
    fn is_dir<P>(&self, path: P) -> bool
    where
        P: AsRef<std::path::Path>;
    fn read_dir<P>(&self, path: P) -> impl Iterator<Item = DirEntry>
    where
        P: AsRef<std::path::Path>;
    fn canonicalize<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>;
    fn read_dirs<I, P>(&self, paths: I) -> impl Iterator<Item = DirEntry>
    where
        I: Iterator<Item = P>,
        P: AsRef<std::path::Path>,
    {
        paths.flat_map(|path| self.read_dir(path))
    }
}

#[derive(Debug)]
pub struct OsFileSystem;

impl OsFileSystem {
    #[must_use]
    pub fn is_executable(metadata: &std::fs::Metadata) -> bool {
        std::os::linux::fs::MetadataExt::st_mode(metadata) & 0o111 != 0
    }
}

impl FileSystem for OsFileSystem {
    fn is_file<P>(&self, path: P) -> bool
    where
        P: AsRef<std::path::Path>,
    {
        path.as_ref().is_file()
    }

    fn is_dir<P>(&self, path: P) -> bool
    where
        P: AsRef<std::path::Path>,
    {
        path.as_ref().is_dir()
    }

    fn read_dir<P>(&self, path: P) -> impl Iterator<Item = DirEntry>
    where
        P: AsRef<std::path::Path>,
    {
        std::fs::read_dir(&path)
            .unwrap()
            .flatten()
            .map(|e| DirEntry::new(e.path(), e.file_name().into_string().unwrap()))
    }
    fn canonicalize<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>,
    {
        path.as_ref().canonicalize().ok()
    }
}

#[derive(Debug, Clone)]
pub struct DirEntry {
    dir: std::path::PathBuf,
    name: String,
}

impl DirEntry {
    fn new(dir: std::path::PathBuf, name: String) -> Self {
        Self { dir, name }
    }
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[must_use]
    pub fn into_name(self) -> String {
        self.name
    }
    #[must_use]
    pub fn dir(&self) -> &std::path::Path {
        &self.dir
    }
}
