pub struct ExitState {
    #[allow(dead_code)]
    code: ExitCode,
    cmd: ExitCommand,
}

impl ExitState {
    #[must_use]
    pub fn resolve(self) -> bool {
        match self.cmd {
            ExitCommand::None => false,
            ExitCommand::Print(s) => {
                println!("{s}");
                false
            }
            ExitCommand::Exit => {
                //println!("exit {}", self.code as u32);
                true
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExitCode {
    Ok,
    Err(usize),
}

pub enum ExitCommand {
    None,
    Print(String),
    Exit,
}

impl From<usize> for ExitCode {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Ok,
            n => Self::Err(n),
        }
    }
}

impl From<ExitCode> for usize {
    fn from(value: ExitCode) -> Self {
        match value {
            ExitCode::Ok => 0,
            ExitCode::Err(n) => n,
        }
    }
}
