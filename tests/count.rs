mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn count_with_literal() {
    let mock = mocks::count_mock("test", 10, &[]);
    Datadog::new(mock).count("test", 10, vec![]);
}

#[test]
pub fn count_with_type() {
    let mock = mocks::count_mock("test1_event", 10, &[]);
    Datadog::new(mock).count(common::TestEvent::Test1, 10, vec![]);
}

#[test]
pub fn count_with_literal_and_tags() {
    let mock = mocks::count_mock("test", 10, &["added:tag", "env:test"]);
    Datadog::new(mock).count("test", 10, vec!["added:tag".to_string()]);
}

#[test]
pub fn count_with_type_and_tags() {
    let mock = mocks::count_mock("test1_event", 10, &["added:tag", "env:test"]);
    Datadog::new(mock).count(common::TestEvent::Test1, 10, vec!["added:tag".to_string()]);
}
