mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn timing_with_literal() {
    let mock = mocks::timing_mock("test", 10, &[]);
    Datadog::new(mock, true, vec![]).timing("test", 10, vec![]);
}

#[test]
pub fn timing_with_type() {
    let mock = mocks::timing_mock("test1_event", 10, &[]);
    Datadog::new(mock, true, vec![]).timing(common::TestEvent::Test1, 10, vec![]);
}

#[test]
pub fn timing_with_literal_and_tags() {
    let mock = mocks::timing_mock("test", 10, &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).timing("test", 10, vec!["added:tag".to_string()]);
}

#[test]
pub fn timing_with_type_and_tags() {
    let mock = mocks::timing_mock("test1_event", 10, &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).timing(
        common::TestEvent::Test1,
        10,
        vec!["added:tag".to_string()],
    );
}
