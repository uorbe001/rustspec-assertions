extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_some};

#[test]
fn be_some_test() {
    expect(&Some(1is)).to(be_some!());
}

#[test]
#[should_fail]
fn be_some_test_fail() {
    expect(&None::<isize>).to(be_some!());
}

#[test]
fn not_be_some_test() {
    expect(&None::<isize>).not_to(be_some!());
}

#[test]
#[should_fail]
fn not_be_some_test_fail() {
    expect(&Some(1is)).not_to(be_some!());
}

