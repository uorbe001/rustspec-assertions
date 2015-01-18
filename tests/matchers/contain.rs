extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, contain};

#[test]
fn contain_int_test() {
    expect(&vec![2is, 1is]).to(contain!(1is));
    expect(&vec![1is, 2is]).to(contain!(1is));
}

#[test]
#[should_fail]
fn contain_int_test_fail() {
    expect(&vec![2is, 1is]).to(contain!(3is));
}

#[test]
fn not_contain_int_test() {
    expect(&vec![2is, 1is]).not_to(contain!(3is));
}

#[test]
#[should_fail]
fn not_contain_int_test_fail() {
    expect(&vec![2is, 1is]).not_to(contain!(1is));
}

