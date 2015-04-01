extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_gt};

#[test]
fn be_gt_int_test() {
    expect(&4isize).to(be_gt!(1isize));
}

#[test]
#[should_panic]
fn be_gt_int_equal_test_fail() {
    expect(&1isize).to(be_gt!(1isize));
}

#[test]
#[should_panic]
fn be_gt_int_test_fail() {
    expect(&1isize).to(be_gt!(2isize));
}

#[test]
fn not_be_gt_int_test() {
    expect(&1isize).not_to(be_gt!(2isize));
}

#[test]
fn not_be_gt_int_equal_test() {
    expect(&1isize).not_to(be_gt!(1isize));
}

#[test]
#[should_panic]
fn not_be_gt_int_equal_test_fail() {
    expect(&2isize).not_to(be_gt!(1isize));
}

#[test]
fn gt_f64_test() {
    expect(&1.2f64).to(be_gt!(1f64));
}

