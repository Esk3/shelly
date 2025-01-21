#[allow(unused_imports)]
use shell_starter_rust::{
    commands::ShellCommands,
    io::StdIoStream,
    shell::{EnvData, Shell, State},
    shell_handler::Handler,
};
use shell_starter_rust::{io::Terminal, text::AutoComplete};

fn main() {
    let dict = vec!["echo", "type", "exit"];
    Handler::new(
        Shell::new(State::new(EnvData::env()), ShellCommands::default()),
        Terminal::create(AutoComplete::new(dict)).unwrap(),
    )
    .run();
}
