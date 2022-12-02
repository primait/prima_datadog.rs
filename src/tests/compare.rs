use std::{thread::sleep, time::Duration};

use rand::Rng;

use crate::compare;

#[test]
fn test_compare() {
    let mut rng = rand::thread_rng();
    let path = rng.gen_range(0..2);
    compare!("test", path, || {
        sleep(Duration::from_millis(rng.gen_range(5..10)));
    }, || {
        sleep(Duration::from_millis(rng.gen_range(5..12)));
    }; "test" => 1);
}

// TODO - Add more tests
