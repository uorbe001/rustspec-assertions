extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct Gt<T> {
    value: T
}

impl<T: Ord + Show> Matcher<T> for Gt<T> {
    fn assert_check(&self, expected: T) -> bool {
        expected > self.value
    }

    fn msg(&self, expected: T) -> String {
        format!("Expected {} to be greater than {} but it was not.", self.value, expected)
    }

    fn negated_msg(&self, expected: T) -> String {
        format!("Expected {} NOT to be greater than {} but it was.", self.value, expected)
    }
}

pub fn be_gt<T: Ord + Show>(value: T) -> Box<Gt<T>> {
    box Gt { value: value }
}
