extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_ge, be_greater_or_equal_to};

#[test]
fn be_ge_int_test() {
    expect(4i).to(be_ge(1i));
}

#[test]
fn be_greater_or_equal_to_int_test() {
    expect(4i).to(be_greater_or_equal_to(1i));
}

#[test]
fn be_ge_int_equal_test() {
    expect(1i).to(be_ge(1i));
}

#[test]
#[should_fail]
fn be_ge_int_test_fail() {
    expect(1i).to(be_ge(2i));
}

#[test]
fn not_to_be_ge_int_test() {
    expect(1i).not_to(be_ge(2i));
}

#[test]
#[should_fail]
fn not_to_be_ge_int_equal_test() {
    expect(1i).not_to(be_ge(1i));
}

#[test]
#[should_fail]
fn not_to_be_ge_int_equal_test_fail() {
    expect(2i).not_to(be_ge(1i));
}
