use crate::Datadog;
use std::vec;

use super::mocks::{expect_event, expect_incr, MockClient};

#[test]
pub fn check_event_sent_exactly_once() {
    let threshold = 7;
    let mut mock = MockClient::new();
    for i in 0..100 {
        if i == threshold {
            mock = expect_event(mock, crate::TAG_LIMIT_EXCEEDED, "Exceeded tag limit", vec![]);
        }
        mock = expect_incr(mock, "test", vec![format!("{}", i)]);
    }
    let dd = Datadog::new(mock, true, threshold);
    for i in 0..100 {
        dd.do_incr("test", vec![format!("{}", i)]);
    }
}
