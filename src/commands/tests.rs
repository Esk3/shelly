use crate::shell::Request;

use super::*;

fn tester<F, T>(f: F) -> T
where
    F: FnOnce(ShellCommands) -> T,
{
    f(ShellCommands::new(Vec::new()))
}

#[test]
fn route_empty_returns_not_found() {
    let cmd = "abc";
    let request = ByteRequest::new(cmd, []);
    tester(|mut router| {
        let Err(err) = router.find_handler(&request) else {
            panic!()
        };
        assert_eq!(err, RouterError::NotFound(cmd.into()));
    });
}

fn tester_default<F, T>(f: F) -> T
where
    F: FnOnce(ShellCommands) -> T,
{
    f(ShellCommands::new_default(crate::fs::tests::MockFs::new(
        ["/abc/xyz"]
            .into_iter()
            .map(std::convert::Into::into)
            .collect(),
        [
            "/abc/",
            "/xyz",
            "/hello_world",
            "/home/other",
            "/home/dummy/dir/abc",
            "/home/dummy/dir/abc/xyz",
            "/home/dummy/dir/abc/xyz/hello_world",
        ]
        .into_iter()
        .map(std::convert::Into::into)
        .collect(),
    )))
}

#[test]
fn router_finds_command_with_same_name() {
    tester_default(|mut router| {
        let cmd = "cd";
        let handler = router.find_handler(&Request::new(cmd, [])).unwrap();
        assert_eq!(handler.name(), cmd);
    });
}

#[test]
fn returns_no_names_when_empty() {
    tester(|router| assert!(router.all_names().is_empty()));
}

#[test]
fn returns_names_of_commands_added() {
    tester(|mut router| {
        router.add(cd::Cd);
        assert_eq!(router.all_names(), ["cd"]);
        router.add(pwd::Pwd);
        assert_eq!(router.all_names(), ["cd", "pwd"]);
    });
}
