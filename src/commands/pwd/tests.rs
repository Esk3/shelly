use crate::{
    commands::Command,
    shell::{Request, State},
};

use super::*;

fn tester<F, T>(f: F) -> T
where
    F: FnOnce(Pwd) -> T,
{
    f(Pwd)
}

#[test]
fn pwd_name() {
    let name = tester(|cmd| cmd.name());
    assert_eq!(name, "pwd");
}

#[test]
fn returns_string() {
    let dir = tester(|mut cmd| cmd.call(Request::new("pwd", []), &State::dummy()))
        .unwrap()
        .message
        .unwrap();
    assert_eq!(dir, "/home/dummy/dir");
}
