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
    let event = test_event(|mut cd| {
        cd.call(
            ByteRequest::new(b"cd", [b"./abc".to_vec()]),
            &State::dummy(),
        )
    });
    assert_eq!(event, [Event::ChangeCwd("./abc".into())]);
}
