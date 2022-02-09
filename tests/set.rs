mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn set_with_literal() {
    let mock = mocks::set_mock("test", "test_value", &[]);
    Datadog::new(mock, true, vec![]).set("test", "test_value", vec![]);
}

#[test]
pub fn set_with_type() {
    let mock = mocks::set_mock("test1_event", "test_value", &[]);
    Datadog::new(mock, true, vec![]).set(common::TestEvent::Test1, "test_value", vec![]);
}

#[test]
pub fn set_with_literal_and_tags() {
    let mock = mocks::set_mock("test", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).set("test", "test_value", vec!["added:tag".to_string()]);
}

#[test]
pub fn set_with_type_and_tags() {
    let mock = mocks::set_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).set(common::TestEvent::Test1, "test_value", vec!["added:tag".to_string()]);
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_set_with_literal() {
    prima_datadog::set!("test", "test_value");
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_set_with_type() {
    use common::TestEvent;
    prima_datadog::set!(TestEvent::Test1, "test_value");
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_set_with_literal_and_tags() {
    prima_datadog::set!("test", "test_value"; "added" => "tag");
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_set_with_type_and_tags() {
    use common::{TestEvent, TestEvent2};
    prima_datadog::set!(TestEvent::Test1, "test_value"; "added" => TestEvent2::Test2);
}
