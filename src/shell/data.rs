use std::path::PathBuf;

#[derive(Debug)]
pub struct State {
    pub cwd: std::path::PathBuf,
    pub path: Vec<String>,
    pub fs: crate::fs::OsFileSystem,
}

impl State {
    pub fn new(env_data: EnvData) -> Self {
        let path = env_data
            .path_env
            .split(':')
            .map(std::string::ToString::to_string)
            .collect();
        Self {
            cwd: env_data.current_dir,
            path,
            fs: crate::fs::OsFileSystem,
        }
    }
    #[cfg(test)]
    #[must_use]
    pub fn dummy() -> Self {
        Self::new(EnvData::new("first", "second", "/home/dummy/dir".into()))
    }
}

#[derive(Debug)]
pub struct EnvData {
    pub path_env: String,
    pub home_env: String,
    pub current_dir: PathBuf,
}

impl EnvData {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(path_env: impl ToString, home_env: impl ToString, current_dir: PathBuf) -> Self {
        Self {
            path_env: path_env.to_string(),
            home_env: home_env.to_string(),
            current_dir,
        }
    }

    #[must_use]
    pub fn env() -> Self {
        Self {
            path_env: std::env::var("PATH").unwrap(),
            home_env: std::env::var("HOME").unwrap(),
            current_dir: std::env::current_dir().unwrap(),
        }
    }
}
