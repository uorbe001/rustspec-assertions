#[phase(plugin, link)] extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_true};

#[test]
fn be_true_test() {
    expect(true).to(be_true!());
}

#[test]
#[should_fail]
fn be_true_test_fail() {
    expect(false).to(be_true!());
}

#[test]
fn not_be_true_test() {
    expect(false).not_to(be_true!());
}

#[test]
#[should_fail]
fn not_be_true_test_fail() {
    expect(true).not_to(be_true!());
}

