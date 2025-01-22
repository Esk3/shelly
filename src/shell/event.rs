use std::path::PathBuf;

use super::Shell;
use crate::shell::{Event, Response};

impl<F> Shell<F>
where
    F: crate::fs::FileSystem,
{
    pub fn handle_events(&mut self, events: Option<Vec<Event>>) -> Result<Option<Response>, Error> {
        let Some(events) = events else {
            return Ok(None);
        };
        for event in events {
            let res = self.handle_event(event)?;
            if !matches!(res, Response::None) {
                return Ok(Some(res));
            }
        }
        Ok(None)
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn handle_event(&mut self, event: Event) -> Result<Response, Error> {
        let res = match event {
            Event::ChangeCwd(input_path) => {
                let new_path = self.data.cwd.join(&input_path);
                let new_path = self.fs.canonicalize(new_path).unwrap();
                if self.fs.find_dir(&new_path).is_none() {
                    return Err(Error::InvalidPath(input_path));
                }
                self.data.cwd = new_path;
                Response::None
            }
            Event::Exit(exit_code) => Response::Exit(exit_code),
        };
        Ok(res)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("cd: {0}: No such file or directory")]
    InvalidPath(PathBuf),
}
