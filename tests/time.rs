mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn time_with_literal() {
    let mock = mocks::time_mock("test", &[]);
    Datadog::new(mock, true, vec![]).time("test", vec![], || {});
}

#[test]
pub fn time_with_type() {
    let mock = mocks::time_mock("test1_event", &[]);
    Datadog::new(mock, true, vec![]).time(common::TestEvent::Test1, vec![], || {});
}

#[test]
pub fn time_with_literal_and_tags() {
    let mock = mocks::time_mock("test", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).time("test", vec!["added:tag".to_string()], || {});
}

#[test]
pub fn time_with_type_and_tags() {
    let mock = mocks::time_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).time(
        common::TestEvent::Test1,
        vec!["added:tag".to_string()],
        || {},
    );
}
