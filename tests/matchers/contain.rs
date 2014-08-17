#[phase(plugin, link)] extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, contain};

#[test]
fn contain_int_test() {
    expect(vec![2i, 1i]).to(contain!(1i));
    expect(vec![1i, 2i]).to(contain!(1i));
}

#[test]
#[should_fail]
fn contain_int_test_fail() {
    expect(vec![2i, 1i]).to(contain!(3i));
}

#[test]
fn not_contain_int_test() {
    expect(vec![2i, 1i]).not_to(contain!(3i));
}

#[test]
#[should_fail]
fn not_contain_int_test_fail() {
    expect(vec![2i, 1i]).not_to(contain!(1i));
}

