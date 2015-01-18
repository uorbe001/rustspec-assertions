extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_le};

#[test]
fn be_le_int_test() {
    expect(&1is).to(be_le!(2is));
}

#[test]
fn be_le_int_equal_test() {
    expect(&1is).to(be_le!(1is));
}

#[test]
#[should_fail]
fn be_le_int_test_fail() {
    expect(&2is).to(be_le!(1is));
}

#[test]
fn not_be_le_int_test() {
    expect(&2is).not_to(be_le!(1is));
}

#[test]
#[should_fail]
fn be_le_int_equal_test_fail() {
    expect(&1is).not_to(be_le!(1is));
}

#[test]
#[should_fail]
fn not_be_le_int_equal_test_fail() {
    expect(&1is).not_to(be_le!(2is));
}

#[test]
fn le_f64_test() {
    expect(&1f64).to(be_le!(1.1f64));
}
