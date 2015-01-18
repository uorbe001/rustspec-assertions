extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, eq};

#[test]
fn eq_int_test() {
    expect(&2is).to(eq!(2is))
}

#[test]
#[should_fail]
fn eq_int_test_fail() {
    expect(&5is).to(eq!(4is));
}

#[test]
fn not_eq_int_test() {
    expect(&2is).not_to(eq!(4is));
}

#[test]
#[should_fail]
fn not_eq_int_test_fail() {
    expect(&5is).not_to(eq!(5is));
}

#[test]
fn eq_f64_test() {
    expect(&1f64).to(eq!(1f64));
}

#[test]
fn eq_usize_test() {
    expect(&1us).to(eq!(1us));
}

