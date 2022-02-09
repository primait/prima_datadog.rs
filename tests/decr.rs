mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn decr_with_literal() {
    let mock = mocks::decr_mock("test", &[]);
    Datadog::new(mock, true, vec![]).decr("test", vec![]);
}

#[test]
pub fn decr_with_type() {
    let mock = mocks::decr_mock("test1_event", &[]);
    Datadog::new(mock, true, vec![]).decr(common::TestEvent::Test1, vec![]);
}

#[test]
pub fn decr_with_literal_and_tags() {
    let mock = mocks::decr_mock("test", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).decr("test", vec!["added:tag".to_string()]);
}

#[test]
pub fn decr_with_type_and_tags() {
    let mock = mocks::decr_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).decr(common::TestEvent::Test1, vec!["added:tag".to_string()]);
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_decr_with_literal() {
    prima_datadog::decr!("test");
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_decr_with_type() {
    use common::TestEvent;
    prima_datadog::decr!(TestEvent::Test1);
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_decr_with_literal_and_tags() {
    prima_datadog::decr!("test"; "added" => "tag");
}

#[test]
#[cfg(feature = "dev-null")]
pub fn macro_decr_with_type_and_tags() {
    use common::{TestEvent, TestEvent2};
    prima_datadog::decr!(TestEvent::Test1; TestEvent2::Test2 => "tag");
}
