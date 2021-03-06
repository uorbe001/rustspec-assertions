extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_none};

#[test]
fn be_none_test() {
    expect(&None::<isize>).to(be_none!());
}

#[test]
#[should_panic]
fn be_none_test_fail() {
    expect(&Some(1isize)).to(be_none!());
}

#[test]
fn not_be_none_test() {
    expect(&Some(1isize)).not_to(be_none!());
}

#[test]
#[should_panic]
fn not_be_none_test_fail() {
    expect(&None::<isize>).not_to(be_none!());
}

