use super::*;

#[test]
fn echo_name_is_echo() {
    assert_eq!(Echo.name(), "echo");
}

#[test]
fn echo_returns_first_arg() {
    let args = ["abc", "xyz", "hello world"].map(std::string::ToString::to_string);
    for arg in args {
        let request = Request::new("echo", [arg.clone()]);
        let response = Echo.call(request, &State::dummy()).unwrap();
        assert_eq!(response.message.unwrap(), arg);
        assert_eq!(response.event, None);
    }
}
