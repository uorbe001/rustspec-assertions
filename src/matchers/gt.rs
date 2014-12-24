extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct Gt<T> {
    value: T,
    file_line: (&'static str, uint)
}

impl<T: PartialOrd + Show> Matcher<T> for Gt<T> {
    fn assert_check(&self, expected: T) -> bool {
        expected > self.value
    }

    fn msg(&self, expected: T) -> String {
        format!("Expected {} to be greater than {} but it was not.", expected, self.value)
    }

    fn negated_msg(&self, expected: T) -> String {
        format!("Expected {} NOT to be greater than {} but it was.", expected, self.value)
    }

    fn get_file_line(&self) -> (&'static str, uint) {
        self.file_line
    }
}

pub fn be_gt<T: PartialOrd + Show>(value: T, file_line: (&'static str, uint)) -> Box<Gt<T>> {
    box Gt { value: value, file_line: file_line }
}

#[macro_export]
pub macro_rules! be_gt(
    ($value:expr) => (
        be_gt($value.clone(), (file!(), expand_line!()))
    );
);
