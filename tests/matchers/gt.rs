extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_gt};

#[test]
fn be_gt_int_test() {
    expect(&4is).to(be_gt!(1is));
}

#[test]
#[should_fail]
fn be_gt_int_equal_test_fail() {
    expect(&1is).to(be_gt!(1is));
}

#[test]
#[should_fail]
fn be_gt_int_test_fail() {
    expect(&1is).to(be_gt!(2is));
}

#[test]
fn not_be_gt_int_test() {
    expect(&1is).not_to(be_gt!(2is));
}

#[test]
fn not_be_gt_int_equal_test() {
    expect(&1is).not_to(be_gt!(1is));
}

#[test]
#[should_fail]
fn not_be_gt_int_equal_test_fail() {
    expect(&2is).not_to(be_gt!(1is));
}

#[test]
fn gt_f64_test() {
    expect(&1.2f64).to(be_gt!(1f64));
}

