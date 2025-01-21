#[allow(unused_imports)]
use shell_starter_rust::{
    commands::ShellCommands,
    io::StdIoStream,
    shell::{EnvData, Shell, State},
    shell_handler::Handler,
};

fn main() {
    Handler::new(
        Shell::new(State::new(EnvData::env()), ShellCommands::default()),
        StdIoStream::new(std::io::stdin(), std::io::stdout()),
    )
    .run();
}
