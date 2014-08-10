#[phase(plugin, link)] extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, eq};

#[test]
fn eq_int_test() {
    expect(2i).to(eq!(2i))
}

#[test]
#[should_fail]
fn eq_int_test_fail() {
    expect(5i).to(eq!(4i));
}

#[test]
fn not_eq_int_test() {
    expect(2i).not_to(eq!(4i));
}

#[test]
#[should_fail]
fn not_eq_int_test_fail() {
    expect(5i).not_to(eq!(5i));
}

#[test]
fn eq_f64_test() {
    expect(1f64).to(eq!(1f64));
}

