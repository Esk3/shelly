#[cfg(test)]
pub mod tests;

pub trait FileSystem {
    fn find_file<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>;
    fn find_file_in_default_path<P>(&self, file: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>;
    fn find_dir<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>;
    fn canonicalize<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>;
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
    fn find_file<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>,
    {
        if path.as_ref().is_file() {
            Some(path.as_ref().to_path_buf())
        } else {
            None
        }
    }

    fn find_file_in_default_path<P>(&self, file: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>,
    {
        let Ok(path) = std::env::var("PATH") else {
            return None;
        };
        let path_seperator = ':';
        path.split(path_seperator)
            .map(std::path::PathBuf::from)
            .map(|path| path.join(&file))
            .find(|path| path.is_file())
    }

    fn find_dir<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>,
    {
        if path.as_ref().is_dir() {
            Some(path.as_ref().to_path_buf())
        } else {
            None
        }
    }

    fn canonicalize<P>(&self, path: P) -> Option<std::path::PathBuf>
    where
        P: AsRef<std::path::Path>,
    {
        path.as_ref().canonicalize().ok()
    }
}
