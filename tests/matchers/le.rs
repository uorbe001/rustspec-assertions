extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_le, be_less_or_equal_to};

#[test]
fn be_le_int_test() {
    expect(1i).to(be_le(2i));
}

#[test]
fn be_less_or_equal_to_int_test() {
    expect(1i).to(be_less_or_equal_to(2i));
}

#[test]
fn be_le_int_equal_test() {
    expect(1i).to(be_le(1i));
}

#[test]
#[should_fail]
fn be_le_int_test_fail() {
    expect(2i).to(be_le(1i));
}

#[test]
fn not_be_le_int_test() {
    expect(2i).not_to(be_le(1i));
}

#[test]
#[should_fail]
fn be_le_int_equal_test_fail() {
    expect(1i).not_to(be_le(1i));
}

#[test]
#[should_fail]
fn not_be_le_int_equal_test_fail() {
    expect(1i).not_to(be_le(2i));
}
