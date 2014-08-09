extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct Equals<T> {
    value: T
}

impl<T: Eq + Show> Matcher<T> for Equals<T> {
    fn assert_check(&self, expected: T) -> bool {
        expected == self.value
    }

    fn msg(&self, expected: T) -> String {
        format!("Expected {} to equal {} but it did not.", self.value, expected)
    }

    fn negated_msg(&self, expected: T) -> String {
        format!("Expected {} NOT to equal {} but it did.", self.value, expected)
    }
}

pub fn eq<T: Eq + Show>(value: T) -> Box<Equals<T>> {
    box Equals { value: value }
}
