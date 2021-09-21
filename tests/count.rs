mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn count_with_literal() {
    let mock = mocks::count_mock("test", 10, &[]);
    Datadog::new(mock, true, vec![]).count("test", 10, vec![]);
}

#[test]
pub fn count_with_type() {
    let mock = mocks::count_mock("test1_event", 10, &[]);
    Datadog::new(mock, true, vec![]).count(common::TestEvent::Test1, 10, vec![]);
}

#[test]
pub fn count_with_literal_and_tags() {
    let mock = mocks::decr_mock("test", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).decr("test", vec!["added:tag".to_string()]);
}

#[test]
pub fn count_with_type_and_tags() {
    let mock = mocks::decr_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"])
        .decr(common::TestEvent::Test1, vec!["added:tag".to_string()]);
}
