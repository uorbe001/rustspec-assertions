extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_some};

#[test]
fn be_some_test() {
    expect(&Some(1isize)).to(be_some!());
}

#[test]
#[should_panic]
fn be_some_test_fail() {
    expect(&None::<isize>).to(be_some!());
}

#[test]
fn not_be_some_test() {
    expect(&None::<isize>).not_to(be_some!());
}

#[test]
#[should_panic]
fn not_be_some_test_fail() {
    expect(&Some(1isize)).not_to(be_some!());
}

