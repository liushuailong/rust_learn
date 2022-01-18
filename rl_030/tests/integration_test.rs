use rl_030;

mod common;

#[test]
fn this_test_will_ok() {
    common::setup();
    assert_eq!(10, rl_030::prints_and_return_10(8));
}

