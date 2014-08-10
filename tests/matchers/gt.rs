extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_gt};

#[test]
fn be_gt_int_test() {
    expect(4i).to(be_gt!(1i));
}

#[test]
#[should_fail]
fn be_gt_int_equal_test_fail() {
    expect(1i).to(be_gt!(1i));
}

#[test]
#[should_fail]
fn be_gt_int_test_fail() {
    expect(1i).to(be_gt!(2i));
}

#[test]
fn not_be_gt_int_test() {
    expect(1i).not_to(be_gt!(2i));
}

#[test]
fn not_be_gt_int_equal_test() {
    expect(1i).not_to(be_gt!(1i));
}

#[test]
#[should_fail]
fn not_be_gt_int_equal_test_fail() {
    expect(2i).not_to(be_gt!(1i));
}
