mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn event_with_literal() {
    let mock = mocks::event_mock("test", "test_value", &[]);
    Datadog::new(mock, true).event("test", "test_value", vec![]);
}

#[test]
pub fn event_with_type() {
    let mock = mocks::event_mock("test1_event", "test_value", &[]);
    Datadog::new(mock, true).event(common::TestEvent::Test1, "test_value", vec![]);
}

#[test]
pub fn event_with_literal_and_tags() {
    let mock = mocks::event_mock("test", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true).event("test", "test_value", vec!["added:tag".to_string()]);
}

#[test]
pub fn event_with_type_and_tags() {
    let mock = mocks::event_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true).event(common::TestEvent::Test1, "test_value", vec!["added:tag".to_string()]);
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_event_with_literal() {
    prima_datadog::event!("test", "test_value");
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_event_with_type() {
    use common::TestEvent;
    prima_datadog::event!(TestEvent::Test1, "test_value");
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_event_with_literal_and_tags() {
    prima_datadog::event!("test", "test_value"; "added" => "tag");
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_event_with_type_and_tags() {
    use common::{TestEvent, TestEvent2};
    prima_datadog::event!(TestEvent::Test1, "test_value"; "added" => TestEvent2::Test2);
}
