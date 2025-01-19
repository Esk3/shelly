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
    let request = Request::new(cmd, []);
    tester(|mut router| {
        let err = router.find_handler(&request).unwrap_err();
        assert_eq!(err, RouterError::NotFound(cmd.to_string()));
    });
}

fn tester_default<F, T>(f: F) -> T
where
    F: FnOnce(ShellCommands) -> T,
{
    f(ShellCommands::default())
}

#[test]
fn router_finds_command_with_same_name() {
    tester_default(|mut router| {
        let cmd = "cd";
        let handler = router.find_handler(&Request::new(cmd, [])).unwrap();
        assert_eq!(handler.name(), cmd);
    });
}
