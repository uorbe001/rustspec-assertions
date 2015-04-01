extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, contain};

#[test]
fn contain_int_test() {
    expect(&vec![2isize, 1isize]).to(contain!(1isize));
    expect(&vec![1isize, 2isize]).to(contain!(1isize));
}

#[test]
#[should_panic]
fn contain_int_test_fail() {
    expect(&vec![2isize, 1isize]).to(contain!(3isize));
}

#[test]
fn not_contain_int_test() {
    expect(&vec![2isize, 1isize]).not_to(contain!(3isize));
}

#[test]
#[should_panic]
fn not_contain_int_test_fail() {
    expect(&vec![2isize, 1isize]).not_to(contain!(1isize));
}

