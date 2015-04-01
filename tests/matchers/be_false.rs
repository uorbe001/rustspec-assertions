extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_false};

#[test]
fn be_false_test() {
    expect(&false).to(be_false!());
}

#[test]
#[should_panic]
fn be_false_test_fail() {
    expect(&true).to(be_false!());
}

#[test]
fn not_be_false_test() {
    expect(&true).not_to(be_false!());
}

#[test]
#[should_panic]
fn not_be_false_test_fail() {
    expect(&false).not_to(be_false!());
}

