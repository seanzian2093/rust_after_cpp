// use crate::ch11;
use rust_after_cpp::ch11::writing_tests::WritingTests;
mod common;

#[test]
fn it_adds_two_ex() {
    common::setup();
    assert_eq!(4, WritingTests::add_two(2));
}
