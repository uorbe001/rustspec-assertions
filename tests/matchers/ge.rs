extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_ge};

#[test]
fn be_ge_int_test() {
    expect(4i).to(be_ge!(1i));
}

#[test]
fn be_ge_int_equal_test() {
    expect(1i).to(be_ge!(1i));
}

#[test]
#[should_fail]
fn be_ge_int_test_fail() {
    expect(1i).to(be_ge!(2i));
}

#[test]
fn not_to_be_ge_int_test() {
    expect(1i).not_to(be_ge!(2i));
}

#[test]
#[should_fail]
fn not_to_be_ge_int_equal_test() {
    expect(1i).not_to(be_ge!(1i));
}

#[test]
#[should_fail]
fn not_to_be_ge_int_equal_test_fail() {
    expect(2i).not_to(be_ge!(1i));
}

#[test]
fn gt_f64_test() {
    expect(1.2f64).to(be_ge!(1f64));
}

