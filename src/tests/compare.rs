use std::thread;

use rand::Rng;

use crate::compare;

#[test]
fn test_macro() {
    let mut rng = rand::rng();
    let path = rng.random_range(0..2);
    let something = "test".to_string();
    // simple compare, no tags
    compare!("test", path, || {}, || {});
    // Simple compare, with tags
    compare!("test", path, || {}, || {}; "tag1" => "tag2");
    // Move compare, no tags
    compare!("test", path, move || {}, move || {});
    // Move compare, with tags
    compare!("test", path, move || {}, move || {}; "tag1" => "tag2");

    // Comparison which returns some value
    let res = compare!("test", path, || 1, || 2);
    assert_eq!(res, path + 1);

    // Comparison which moves something
    let expected = something.len() / (path + 1);
    let handle = compare!(
        "test",
        path,
        move || thread::spawn(move || something.len()),
        move || thread::spawn(move || something.len() / 2)
    );
    assert_eq!(handle.join().unwrap(), expected);
}

// TODO - Add more tests
