use std::sync::{atomic::AtomicBool, Arc};

use crate::{DatadogWrapper, TagTrackerConfiguration};

use super::mocks::{expect_event, expect_incr, MockClient};

#[test]
pub fn no_actions_tracker_does_nothing() {
    let mut mock = MockClient::new();
    let tracker_config = TagTrackerConfiguration::new().with_threshold(5);
    // Expect 100 increment calls
    for i in 0..100 {
        mock = expect_incr(mock, "test", vec![format!("{}", i)]);
    }
    let dd = DatadogWrapper::new(mock, true, tracker_config);
    for i in 0..100 {
        dd.do_incr("test", vec![format!("{}", i)]);
    }
}

#[test]
pub fn event_action_tracker_emits_event() {
    let threshold = 5;
    let title = "test";
    let message = "Exceeded tag cardinality limit";
    let mut mock = MockClient::new();
    let tracker_config = TagTrackerConfiguration::new()
        .with_threshold(threshold)
        .with_event(String::from(title), String::from(message));
    let tags = vec![format!("test:{}", threshold)];
    // Expect 100 increment calls
    for i in 0..100 {
        if i == threshold {
            mock = expect_event(mock, title, message, tags.clone());
        }
        mock = expect_incr(mock, "test", vec![format!("{}", i)]);
    }
    let dd = DatadogWrapper::new(mock, true, tracker_config);
    for i in 0..100 {
        dd.do_incr("test", vec![format!("{}", i)]);
    }
}

#[test]
fn custom_action_is_run() {
    let threshold = 5;
    let mut mock = MockClient::new();
    // We expect the closure to be called when the threshold is reached, which is
    // when 5 unique tag values are seen. Since we start with a tag of 0, then 1,
    // etc, the point at which the threshold is reached is when we call incr with
    // a tag of threshold - 1
    let expected_tags: Vec<_> = vec![format!("{}", threshold - 1)];
    let called = Arc::new(AtomicBool::new(false));
    let outer = called.clone();
    let custom_action = move |metric: &str, tags: &[&str]| {
        assert_eq!(metric, "test");
        assert!(tags.iter().all(|t| expected_tags.iter().any(|e| e == t)));
        assert!(tags.len() == expected_tags.len());
        assert!(!called.load(std::sync::atomic::Ordering::SeqCst)); // Assert we're called at most once
        called.store(true, std::sync::atomic::Ordering::SeqCst);
    };
    let tracker_config = TagTrackerConfiguration::new()
        .with_threshold(threshold)
        .with_custom_action(custom_action);
    // Expect 100 increment calls
    for i in 0..100 {
        mock = expect_incr(mock, "test", vec![format!("{}", i)]);
    }
    let dd = DatadogWrapper::new(mock, true, tracker_config);
    for i in 0..100 {
        dd.do_incr("test", vec![format!("{}", i)]);
    }
    // Assert the closure is called at least once
    assert!(outer.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
pub fn check_event_sent_exactly_once() {
    let threshold = 7;
    let title = "test";
    let message = "Exceeded tag cardinality limit";
    let tags: Vec<_> = vec![format!("test:{}", threshold)];
    let mut mock = MockClient::new();
    for i in 0..100 {
        if i == threshold {
            mock = expect_event(mock, title, message, tags.clone());
        }
        mock = expect_incr(mock, "test", vec![format!("{}", i)]);
    }
    let tracking_config = TagTrackerConfiguration::new()
        .with_threshold(threshold)
        .with_event(String::from(title), String::from(message));
    let dd = DatadogWrapper::new(mock, true, tracking_config);
    for i in 0..100 {
        dd.do_incr("test", vec![format!("{}", i)]);
    }
}

#[test]
pub fn check_algorithm_counts_unique_sets_directly() {
    let threshold = 3;
    let set1: Vec<_> = vec!["a", "b", "c"].iter().map(|s| s.to_string()).collect();
    let set2: Vec<_> = vec!["a", "b", "d"].iter().map(|s| s.to_string()).collect();
    let set3: Vec<_> = vec!["a", "c", "d"].iter().map(|s| s.to_string()).collect();
    // The above are 3 unique tag sets, so we expect an event to be emitted
    let mock = MockClient::new();
    let mock = expect_incr(mock, "test", set1.clone());
    let mock = expect_incr(mock, "test", set2.clone());
    let mock = expect_incr(mock, "test", set3.clone());
    let mock = expect_event(mock, "title", "text", vec![format!("test:{}", threshold)]);
    let tracking_config = TagTrackerConfiguration::new()
        .with_threshold(threshold)
        .with_event("title".to_string(), "text".to_string()); // This event should be emitted
    let dd = DatadogWrapper::new(mock, true, tracking_config);
    dd.do_incr("test", set1);
    dd.do_incr("test", set2);
    dd.do_incr("test", set3);
}
