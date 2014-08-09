extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct Ge<T> {
    value: T
}

impl<T: Ord + Show> Matcher<T> for Ge<T> {
    fn assert_check(&self, expected: T) -> bool {
        expected >= self.value
    }

    fn msg(&self, expected: T) -> String {
        format!("Expected {} to be greater or equal to {} but it was not.", self.value, expected)
    }

    fn negated_msg(&self, expected: T) -> String {
        format!("Expected {} NOT to be greater or equal to {} but it was.", self.value, expected)
    }
}

pub fn be_ge<T: Ord + Show>(value: T) -> Box<Ge<T>> {
    box Ge { value: value }
}
