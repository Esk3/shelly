#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    repl();
}

fn repl() {
    let run = true;

    let stdin = io::stdin();
    let mut input = String::new();

    while run {
        stdin.read_line(&mut input).unwrap();

        let command = input.trim();

        println!("{}: command not found\n", command);
        input.clear();
    }
}
