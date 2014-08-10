extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_lt, be_less_than};

#[test]
fn be_lt_int_test() {
    expect(1i).to(be_lt(2i));
}

#[test]
fn be_less_than_int_test() {
    expect(1i).to(be_less_than(2i));
}

#[test]
#[should_fail]
fn be_lt_int_equal_test_fail() {
    expect(1i).to(be_lt(1i));
}

#[test]
#[should_fail]
fn be_lt_int_test_fail() {
    expect(2i).to(be_lt(1i));
}

#[test]
fn not_be_lt_int_test() {
    expect(2i).not_to(be_lt(1i));
}

#[test]
fn not_be_lt_int_equal_test() {
    expect(1i).not_to(be_lt(1i));
}

#[test]
#[should_fail]
fn not_be_lt_int_equal_test_fail() {
    expect(1i).not_to(be_lt(2i));
}
