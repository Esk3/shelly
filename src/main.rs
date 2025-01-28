use shell_starter_rust::{
    commands::ShellCommands,
    shell::{EnvData, State},
    shell_handler::Handler,
};
use shell_starter_rust::{io::Terminal, shell::OsShell, text::AutoComplete};

fn main() {
    let dict = vec!["echo", "type", "exit"];
    Handler::new(
        OsShell::new(
            State::new(EnvData::env()),
            ShellCommands::new_default(shell_starter_rust::fs::OsFileSystem),
            shell_starter_rust::fs::OsFileSystem,
        ),
        Terminal::create(AutoComplete::new(dict)).unwrap(),
    )
    .run();
}
