extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, eq};

#[test]
fn eq_int_test() {
    expect(&2isize).to(eq!(2isize))
}

#[test]
#[should_panic]
fn eq_int_test_fail() {
    expect(&5isize).to(eq!(4isize));
}

#[test]
fn not_eq_int_test() {
    expect(&2isize).not_to(eq!(4isize));
}

#[test]
#[should_panic]
fn not_eq_int_test_fail() {
    expect(&5isize).not_to(eq!(5isize));
}

#[test]
fn eq_f64_test() {
    expect(&1f64).to(eq!(1f64));
}

#[test]
fn eq_usize_test() {
    expect(&1usize).to(eq!(1usize));
}

