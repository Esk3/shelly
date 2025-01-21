use super::Stream;

#[derive(Debug)]
pub struct StdIoStream {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
}

impl StdIoStream {
    #[must_use]
    pub fn new(stdin: std::io::Stdin, stdout: std::io::Stdout) -> Self {
        Self { stdin, stdout }
    }
}

impl Stream for StdIoStream {}

impl std::io::Read for StdIoStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stdin.read(buf)
    }
}
impl std::io::Write for StdIoStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}
