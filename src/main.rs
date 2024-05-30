#[allow(unused_imports)]
//#![warn(clippy::pedantic)]
use std::io::{self, Write};

fn main() {
    repl();
}

fn repl() {
    let mut run = true;

    let stdin = io::stdin();
    let mut input = String::new();

    let commands = Commands::builder()
        .add_command(Echo::handler)
        .add_command(Exit::handler)
        .add_command(Type::handler)
        .add_command(NotFound::handler);

    while run {
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();

        let request = Request::from(input.as_str());
        let response = commands.extract_command(&request);
        run = !response.0.run();

        input.clear();
    }
}

struct Exit(i32);
impl Command for Exit {
    fn run(&self) -> bool {
        true
    }
}

struct Type(String);
impl Command for Type {
    fn run(&self) -> bool {
        todo!()
    }
}

impl Type {
    pub fn handler(request: &Request) -> Option<Response> {
        let Some(first) = request.0.first() else {
            return None;
        };
        if *first != "type" {
            return None;
        }
        Some(Response(Box::new(Self((*request.0.get(1)?).to_string()))))
    }
}

impl Exit {
    pub fn handler(request: &Request) -> Option<Response> {
        let Some(first) = request.0.first() else {
            return None;
        };
        if *first != "exit" {
            return None;
        }
        Some(Response(Box::new(Self(0))))
    }
}

struct NotFound(String);
impl Command for NotFound {
    fn run(&self) -> bool {
        println!("{}: command not found", self.0);
        false
    }
}
impl NotFound {
    pub fn handler(request: &Request) -> Option<Response> {
        Some(Response(Box::new(Self(
            (*request.0.first().unwrap()).to_string(),
        ))))
    }
}

struct Echo(String);
impl Command for Echo {
    fn run(&self) -> bool {
        println!("{}", self.0);
        false
    }
}
impl Echo {
    pub fn handler(request: &Request) -> Option<Response> {
        let Some(first) = request.0.first() else {
            return None;
        };
        if *first != "echo" {
            return None;
        }
        Some(Response(Box::new(Self(request.0[1..].join(" ")))))
    }
}

struct Request<'a>(Vec<&'a str>);
impl<'a> From<&'a str> for Request<'a> {
    fn from(value: &'a str) -> Self {
        Self(value.split_whitespace().collect::<Vec<&str>>())
    }
}
struct Response(Box<dyn Command>);

trait Command {
    fn run(&self) -> bool;
}

type HandlerType = Box<dyn Fn(&Request) -> Option<Response>>;

struct CommandHandler(HandlerType);
trait IntoCommandHandler {
    fn into(self) -> CommandHandler;
}
impl<T> IntoCommandHandler for T
where
    T: Fn(&Request) -> Option<Response> + 'static,
{
    fn into(self) -> CommandHandler {
        CommandHandler(Box::new(self))
    }
}

struct Commands(Vec<CommandHandler>);

impl Commands {
    pub fn builder() -> Self {
        Self(Vec::new())
    }
    pub fn add_command(mut self, cmd: impl IntoCommandHandler) -> Self {
        self.0.push(cmd.into());
        self
    }
    pub fn extract_command(&self, request: &Request) -> Response {
        self.0.iter().find_map(|h| h.0(request)).unwrap()
    }
}
