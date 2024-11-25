use dogstatsd::EventAlertType;
use dogstatsd::EventOptions;
use dogstatsd::EventPriority;

use crate::event_with_options;
use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;
use crate::TagTrackerConfiguration;
use crate::EMPTY_TAGS;

#[test]
pub fn event_with_options_with_literal() {
    let mock = mocks::event_with_options_mock("test", "test_value", &[], None);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_event_with_options("test", "test_value", EMPTY_TAGS, None);
}

#[test]
pub fn event_with_options_with_type() {
    let mock = mocks::event_with_options_mock("test1_event", "test_value", &[], None);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_event_with_options(
        TestEvent::Test1,
        "test_value",
        EMPTY_TAGS,
        None,
    );
}

#[test]
pub fn event_with_options_with_literal_and_tags() {
    let mock = mocks::event_with_options_mock("test", "test_value", &["added:tag", "env:test"], None);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_event_with_options(
        "test",
        "test_value",
        vec!["added:tag".to_string()],
        None,
    );
}

#[test]
pub fn event_with_options_with_type_and_tags() {
    let mock = mocks::event_with_options_mock("test1_event", "test_value", &["added:tag", "env:test"], None);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_event_with_options(
        TestEvent::Test1,
        "test_value",
        vec!["added:tag".to_string()],
        None,
    );
}

#[test]
pub fn event_with_options_with_options() {
    let options = Some(
        EventOptions::new()
            .with_alert_type(EventAlertType::Info)
            .with_priority(EventPriority::Low)
            .with_aggregation_key("aggregation_key")
            .with_source_type_name("source_type_name")
            .with_hostname("hostname")
            .with_timestamp(12341234),
    );

    let mock = mocks::event_with_options_mock("test1_event", "test_value", &["added:tag", "env:test"], options);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_event_with_options(
        TestEvent::Test1,
        "test_value",
        vec!["added:tag".to_string()],
        options,
    );
}

#[test]
pub fn test_macro() {
    let tag = String::from("tag");
    // no tags
    event_with_options!("test", "test_value");
    // just literal tags
    event_with_options!("test", "test_value"; "literal" => 1);
    // just expression tags
    event_with_options!("test", "test_value"; "expression" => tag);
    // mixed tags
    event_with_options!("test", "test_value"; "literal" => 1, "expression" => tag);
}
