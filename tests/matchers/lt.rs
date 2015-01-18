extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_lt};

#[test]
fn be_lt_int_test() {
    expect(&1is).to(be_lt!(2is));
}

#[test]
#[should_fail]
fn be_lt_int_equal_test_fail() {
    expect(&1is).to(be_lt!(1is));
}

#[test]
#[should_fail]
fn be_lt_int_test_fail() {
    expect(&2is).to(be_lt!(1is));
}

#[test]
fn not_be_lt_int_test() {
    expect(&2is).not_to(be_lt!(1is));
}

#[test]
fn not_be_lt_int_equal_test() {
    expect(&1is).not_to(be_lt!(1is));
}

#[test]
#[should_fail]
fn not_be_lt_int_equal_test_fail() {
    expect(&1is).not_to(be_lt!(2is));
}

#[test]
fn lt_f64_test() {
    expect(&1f64).to(be_lt!(1.1f64));
}
