use std::sync::{atomic::AtomicBool, Arc};

use crate::{Datadog, TrackerConfiguration};

use super::mocks::{expect_event, expect_incr, MockClient};

#[test]
pub fn no_actions_tracker_does_nothing() {
    let mut mock = MockClient::new();
    let tracker_config = TrackerConfiguration::new().with_threshold(5);
    // Expect 100 increment calls
    for i in 0..100 {
        mock = expect_incr(mock, "test", vec![format!("{}", i)]);
    }
    let dd = Datadog::new(mock, true, tracker_config);
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
    let tracker_config = TrackerConfiguration::new()
        .with_threshold(threshold)
        .with_event(String::from(title), String::from(message));
    let tags = vec![format!("test:{}", threshold)];
    // Expect 100 increment calls
    for i in 0..100 {
        if i == threshold {
            mock = expect_event(mock, title, &message, tags.clone());
        }
        mock = expect_incr(mock, "test", vec![format!("{}", i)]);
    }
    let dd = Datadog::new(mock, true, tracker_config);
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
    let custom_action: Box<_> = Box::from(move |metric, tags: Vec<String>| {
        assert_eq!(metric, "test");
        assert!(tags.iter().all(|t| expected_tags.contains(t)));
        assert!(tags.len() == expected_tags.len());
        assert!(!called.load(std::sync::atomic::Ordering::SeqCst)); // Assert we're called at most once
        called.store(true, std::sync::atomic::Ordering::SeqCst);
    });
    let tracker_config = TrackerConfiguration::new()
        .with_threshold(threshold)
        .with_custom(custom_action);
    // Expect 100 increment calls
    for i in 0..100 {
        mock = expect_incr(mock, "test", vec![format!("{}", i)]);
    }
    let dd = Datadog::new(mock, true, tracker_config);
    for i in 0..100 {
        dd.do_incr("test", vec![format!("{}", i)]);
    }
    // Assert the closure is called at least once
    assert!(outer.load(std::sync::atomic::Ordering::SeqCst));
}
