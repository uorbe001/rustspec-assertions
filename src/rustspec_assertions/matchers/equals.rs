extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct Equals<T> {
    value: T,
    file_line: (&'static str, uint)
}

impl<T: PartialEq + Show> Matcher<T> for Equals<T> {
    fn assert_check(&self, expected: T) -> bool {
        expected == self.value
    }

    fn msg(&self, expected: T) -> String {
        format!("Expected {} to equal {} but it did not.", self.value, expected)
    }

    fn negated_msg(&self, expected: T) -> String {
        format!("Expected {} NOT to equal {} but it did.", self.value, expected)
    }

    fn get_file_line(&self) -> (&'static str, uint) {
        self.file_line
    }
}

pub fn eq<T: PartialEq + Show>(value: T, file_line: (&'static str, uint)) -> Box<Equals<T>> {
    box Equals { value: value, file_line: file_line }
}

#[macro_export]
pub macro_rules! eq(
    ($value:expr) => (
        eq($value, (file!(), expand_line!()))
    );
)
