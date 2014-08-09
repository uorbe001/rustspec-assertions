extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, eq, equal};

#[test]
fn eq_int_test() {
    expect(4i).to(eq(4i));
}

#[test]
fn equal_int_test() {
    // This is just the same as eq
    expect(4i).to(equal(4i));
}

#[test]
#[should_fail]
fn eq_int_test_fail() {
    expect(5i).to(eq(4i));
}

#[test]
fn not_eq_int_test() {
    expect(2i).not_to(eq(4i));
}

#[test]
#[should_fail]
fn not_eq_int_test_fail() {
    expect(5i).not_to(eq(5i));
}
