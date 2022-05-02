mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn count_with_literal() {
    let mock = mocks::count_mock("test", 10, &[]);
    Datadog::new(mock, true).count("test", 10, vec![]);
}

#[test]
pub fn count_with_type() {
    let mock = mocks::count_mock("test1_event", 10, &[]);
    Datadog::new(mock, true).count(common::TestEvent::Test1, 10, vec![]);
}

#[test]
pub fn count_with_literal_and_tags() {
    let mock = mocks::count_mock("test", 10, &["added:tag", "env:test"]);
    Datadog::new(mock, true).count("test", 10, vec!["added:tag".to_string()]);
}

#[test]
pub fn count_with_type_and_tags() {
    let mock = mocks::count_mock("test1_event", 10, &["added:tag", "env:test"]);
    Datadog::new(mock, true).count(common::TestEvent::Test1, 10, vec!["added:tag".to_string()]);
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_count_with_literal() {
    prima_datadog::count!("test", 10);
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_count_with_type() {
    use common::TestEvent;
    prima_datadog::count!(TestEvent::Test1, 10);
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_count_with_literal_and_tags() {
    prima_datadog::count!("test", 10; "added" => "tag");
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_count_with_type_and_tags() {
    use common::{TestEvent, TestEvent2};
    prima_datadog::count!(TestEvent::Test1, 10; "added" => TestEvent2::Test2);
}
