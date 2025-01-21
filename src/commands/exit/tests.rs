use crate::commands::Event;

use super::*;

#[test]
fn exit_name() {
    assert_eq!(Exit.name(), "exit");
}

#[test]
fn exit_returns_exit_event() {
    let res = Exit
        .call(ByteRequest::empty("exit"), &State::dummy())
        .unwrap();
    assert_eq!(
        res.event.unwrap()[0],
        Event::Exit(crate::exit::ExitCode::Ok)
    );
    assert_eq!(res.message, None);
}

#[test]
fn exit_event_code_matches_arg() {
    let codes = [1, 2, 3, 4];
    for code in codes {
        let res = Exit
            .call(
                ByteRequest::new("exit", [code.to_string().as_bytes().to_vec()]),
                &State::dummy(),
            )
            .unwrap();
        assert_eq!(res.event.unwrap(), [Event::Exit(ExitCode::Err(code))]);
    }
}
