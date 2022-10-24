use crate::{Datadog, TrackerConfiguration};
use std::vec;

use super::mocks::{expect_event, expect_incr, MockClient};

#[test]
pub fn check_event_sent_exactly_once() {
    let threshold = 7;
    let title = "test";
    let message = "Exceeded tag cardinality limit";
    let tags: Vec<_> = vec![format!("test:{}", threshold)];
    let mut mock = MockClient::new();
    for i in 0..100 {
        if i == threshold {
            mock = expect_event(mock, title, &message, tags.clone());
        }
        mock = expect_incr(mock, "test", vec![format!("{}", i)]);
    }
    let tracking_config = TrackerConfiguration::new()
        .with_threshold(threshold)
        .with_event(String::from(title), String::from(message));
    let dd = Datadog::new(mock, true, tracking_config);
    for i in 0..100 {
        dd.do_incr("test", vec![format!("{}", i)]);
    }
}
