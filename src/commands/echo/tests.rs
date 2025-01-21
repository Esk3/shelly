use super::*;

#[test]
fn echo_name_is_echo() {
    assert_eq!(Echo.name(), "echo");
}

#[test]
fn echo_returns_first_arg() {
    let args = ["abc", "xyz", "hello world"].map(std::string::ToString::to_string);
    for arg in args {
        let request = ByteRequest::new("echo", [arg.as_bytes().to_vec()]);
        let response = Echo.call(request, &State::dummy()).unwrap();
        assert_eq!(response.message.unwrap(), arg);
        assert_eq!(response.event, None);
    }
}

#[test]
fn echo_returns_all_args() {
    let args = [
        ["123", "second", "third"],
        ["3242", "209808", "eprrteqew"],
        ["qwerty", "zxcvb", "abc"],
    ]
    .map(|args| args.map(std::string::ToString::to_string));
    for args in args {
        let request = ByteRequest::new(
            "echo",
            args.iter()
                .map(|s| s.as_bytes().to_vec())
                .collect::<Vec<_>>(),
        );
        let response = Echo.call(request, &State::dummy()).unwrap();
        assert_eq!(response.message.unwrap(), args.join(" "));
        assert_eq!(response.event, None);
    }
}
