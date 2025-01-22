use super::*;

#[test]
fn cmd_type_name() {
    assert_eq!(
        CmdType::new(Vec::new(), crate::fs::tests::MockFs::empty()).name(),
        "type"
    );
}

#[test]
fn type_is_always_included() {
    let cmd = CmdType::new(Vec::new(), crate::fs::tests::MockFs::empty());
    let name = cmd.name();
    assert!(cmd.is_builtin(name));
}

#[test]
fn is_builting_returns_true_if_in_list_on_creation() {
    let cmds = ["abc", "xyz"];
    let command = CmdType::new(cmds, crate::fs::tests::MockFs::empty());

    assert!(command.is_builtin("abc"));
    assert!(!command.is_builtin("notFound"));
    assert!(command.is_builtin("xyz"));
}

//#[test]
//fn handle_command_returns_found_if_is_builtin_is_true() {
//    let builtin = ["abc", "xyz", "123"];
//    let not_found = ["something", "else"];
//    let command = CmdType::new(builtin);
//
//    for cmd in builtin {
//        assert_eq!(
//            command.handle_command(cmd.to_string(), &[]),
//            Response::new(cmd, Kind::Builtin)
//        );
//    }
//    for cmd in not_found {
//        assert_eq!(
//            command.handle_command(cmd.to_string(), &[]),
//            Response::new(cmd, Kind::NotFound)
//        );
//    }
//}

#[test]
fn call_returns_response() {
    let builtin = ["abc", "xyz", "123"];
    let not_found = ["something", "else"].map(|s| s.as_bytes().to_vec());
    let mut command = CmdType::new(
        builtin,
        crate::fs::tests::MockFs::new(builtin.map(|s| s.into()).to_vec(), [].to_vec()),
    );
    let builtin = ["abc", "xyz", "123"].map(|s| s.as_bytes().to_vec());

    for cmd in builtin {
        let message = command
            .call(ByteRequest::new(b"type", [cmd.clone()]), &State::dummy())
            .unwrap()
            .message
            .unwrap();
        assert_eq!(
            message,
            format!("{} is a shell builtin", String::from_utf8(cmd).unwrap())
        );
    }
    for cmd in not_found {
        let message = command
            .call(ByteRequest::new(b"type", [cmd.clone()]), &State::dummy())
            .unwrap()
            .message
            .unwrap();
        assert_eq!(
            message,
            format!("{}: not found", String::from_utf8(cmd).unwrap())
        );
    }
}
