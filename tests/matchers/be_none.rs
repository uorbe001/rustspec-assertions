extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_none};

#[test]
fn be_none_test() {
    expect(&None::<isize>).to(be_none!());
}

#[test]
#[should_fail]
fn be_none_test_fail() {
    expect(&Some(1is)).to(be_none!());
}

#[test]
fn not_be_none_test() {
    expect(&Some(1is)).not_to(be_none!());
}

#[test]
#[should_fail]
fn not_be_none_test_fail() {
    expect(&None::<isize>).not_to(be_none!());
}

