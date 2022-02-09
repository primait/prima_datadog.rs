mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn incr_with_literal() {
    let mock = mocks::incr_mock("test", &[]);
    Datadog::new(mock, true, vec![]).incr("test", vec![]);
}

#[test]
pub fn incr_with_type() {
    let mock = mocks::incr_mock("test1_event", &[]);
    Datadog::new(mock, true, vec![]).incr(common::TestEvent::Test1, vec![]);
}

#[test]
pub fn incr_with_literal_and_tags() {
    let mock = mocks::incr_mock("test", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).incr("test", vec!["added:tag".to_string()]);
}

#[test]
pub fn incr_with_type_and_tags() {
    let mock = mocks::incr_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).incr(common::TestEvent::Test1, vec!["added:tag".to_string()]);
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_incr_with_literal() {
    prima_datadog::incr!("test");
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_incr_with_type() {
    use common::TestEvent;
    prima_datadog::incr!(TestEvent::Test1);
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_incr_with_literal_and_tags() {
    prima_datadog::incr!("test"; "added" => "tag");
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_incr_with_type_and_tags() {
    use common::{TestEvent, TestEvent2};
    prima_datadog::incr!(TestEvent::Test1; TestEvent2::Test2 => "tag");
}
