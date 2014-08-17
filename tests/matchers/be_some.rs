#[phase(plugin, link)] extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_some};

#[test]
fn be_some_test() {
    expect(Some(1i)).to(be_some!());
}

#[test]
#[should_fail]
fn be_some_test_fail() {
    expect(None::<int>).to(be_some!());
}

#[test]
fn not_be_some_test() {
    expect(None::<int>).not_to(be_some!());
}

#[test]
#[should_fail]
fn not_be_some_test_fail() {
    expect(Some(1i)).not_to(be_some!());
}

