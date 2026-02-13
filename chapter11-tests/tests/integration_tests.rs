mod common;

use chapter11_tests::add;

// only library crate can run integration tests
// so it's suggested to use lib.rs mainly and a simple main.rs
#[test]
fn it_adds_two() {
    common::setup();
    let result = add(2, 2);
    assert_eq!(result, 4);
}
