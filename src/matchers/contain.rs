extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct Contain<T> {
    value: T,
    file_line: (&'static str, uint)
}

impl <T: Show + PartialEq> Matcher<Vec<T>> for Contain<T> {
    fn assert_check(&self, expected: Vec<T>) -> bool {
        expected.contains(&self.value)
    }

    fn msg(&self, expected: Vec<T>) -> String {
        format!("Expected {} to contain {} but it did not.", expected, self.value)
    }

    fn negated_msg(&self, expected: Vec<T>) -> String {
        format!("Expected {} NOT to contain {} but it did.", expected, self.value)
    }

    fn get_file_line(&self) -> (&'static str, uint) {
        self.file_line
    }
}

pub fn contain<T: Show + PartialEq>(value: T, file_line: (&'static str, uint)) -> Box<Contain<T>> {
    box Contain { value: value, file_line: file_line }
}

#[macro_export]
pub macro_rules! contain(
    ($value:expr) => (
        contain($value.clone(), (file!(), expand_line!()))
    );
);
