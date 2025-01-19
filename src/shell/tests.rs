use super::*;

fn tester<F, T>(f: F) -> T
where
    F: FnOnce(Shell) -> T,
{
    let commands = ShellCommands::default();
    let shell = Shell::new(State::dummy(), commands);
    f(shell)
}

#[test]
fn shell_returns_newline() {
    let s = tester(|shell| shell.new_line());
    assert_eq!(s, "$ ");
}

#[test]
fn shell_prints_starting_dir() {
    let res = tester(|mut shell| shell.handle_request(Request::new("pwd", [])));
    let s = res.unwrap().into_message().unwrap();
    assert_eq!(s, "/home/dummy");
}

#[test]
fn shell_prints_new_dir_when_set_with_absolute_cd() {
    let new_dir = "/home/other";
    let cwd = tester(|mut shell| {
        shell
            .handle_request(Request::new("cd", [new_dir.to_string()]))
            .unwrap();
        shell.handle_request(Request::new("pwd", []))
    })
    .unwrap()
    .into_message()
    .unwrap();
    assert_eq!(cwd, new_dir);
}

#[test]
fn handle_set_cwd_event_sets_cwd_state() {
    tester(|mut shell| {
        let cwd = ["abc", "xyz", "hello_world"];
        for cwd in cwd {
            shell.handle_event(Event::SetCwd(cwd.to_string())).unwrap();
            assert_eq!(shell.data.cwd.to_str().unwrap(), cwd);
        }
    });
}

#[test]
fn handle_events_handles_set_cwd() {
    tester(|mut shell| {
        let cwd = ["abc", "xyz", "hello_world"];
        for cwd in cwd {
            shell
                .handle_events(Some([Event::SetCwd(cwd.to_string())].to_vec()))
                .unwrap();
            assert_eq!(shell.data.cwd.to_str().unwrap(), cwd);
        }
    });
}

#[test]
fn echo() {
    let args = ["dajskf", "1235"].map(std::string::ToString::to_string);
    tester(|mut shell| {
        for arg in args {
            let res = shell
                .handle_request(Request::new("echo", [arg.clone()]))
                .unwrap();
            assert_eq!(res.into_message().unwrap(), arg);
        }
    });
}

#[test]
fn exit_returns_exit_response() {
    tester(|mut shell| {
        let res = shell.handle_request(Request::empty("exit")).unwrap();
        assert_eq!(res, Response::Exit(ExitCode::Ok));
        for code in [1, 2, 4, 19] {
            let res = shell
                .handle_request(Request::new("exit", [code.to_string()]))
                .unwrap();
            assert_eq!(res, Response::Exit(ExitCode::Err(code)));
        }
    });
}
