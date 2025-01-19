use super::*;

#[test]
fn cmd_type_name() {
    assert_eq!(CmdType::new(Vec::new()).name(), "type");
}

#[test]
fn type_is_always_included() {
    let cmd = CmdType::new(Vec::new());
    let name = cmd.name();
    assert!(cmd.is_builtin(name));
}

#[test]
fn is_builting_returns_true_if_in_list_on_creation() {
    let cmds = ["abc", "xyz"];
    let command = CmdType::new(cmds);

    assert!(command.is_builtin("abc"));
    assert!(!command.is_builtin("notFound"));
    assert!(command.is_builtin("xyz"));
}

#[test]
fn handle_command_returns_found_if_is_builtin_is_true() {
    let builtin = ["abc", "xyz", "123"];
    let not_found = ["something", "else"];
    let command = CmdType::new(builtin);

    for cmd in builtin {
        assert_eq!(
            command.handle_command(cmd.to_string()),
            Response::new(cmd, Kind::Builtin)
        );
    }
    for cmd in not_found {
        assert_eq!(
            command.handle_command(cmd.to_string()),
            Response::new(cmd, Kind::NotFound)
        );
    }
}

#[test]
fn call_returns_response() {
    let builtin = ["abc", "xyz", "123"];
    let not_found = ["something", "else"];
    let mut command = CmdType::new(builtin);

    for cmd in builtin {
        let message = command
            .call(Request::new("type", [cmd.to_string()]), &State::dummy())
            .unwrap()
            .message
            .unwrap();
        assert_eq!(message, format!("{cmd} is a shell builtin"));
    }
    for cmd in not_found {
        let message = command
            .call(Request::new("type", [cmd.to_string()]), &State::dummy())
            .unwrap()
            .message
            .unwrap();
        assert_eq!(message, format!("{cmd}: not found"));
    }
}
