use crate::commands::Event;

use super::*;

fn tester<F>(f: F) -> Result<Response, Error>
where
    F: FnOnce(Cd) -> Result<Response, Error>,
{
    f(Cd)
}

fn test_event<F>(f: F) -> Vec<Event>
where
    F: FnOnce(Cd) -> Result<Response, Error>,
{
    tester(f).unwrap().event.unwrap()
}

#[test]
fn cd_returns_change_dir_event_on_relative_path() {
    let event =
        test_event(|mut cd| cd.call(Request::new("cd", ["./abc".to_string()]), &State::dummy()));
    assert_eq!(event, [Event::ChangeCwd("./abc".to_string())]);
}

#[test]
fn cd_returns_set_dir_event_on_absolute_path() {
    let event =
        test_event(|mut cd| cd.call(Request::new("cd", ["/abc".to_string()]), &State::dummy()));
    assert_eq!(event, [Event::SetCwd("/abc".to_string())]);
}
