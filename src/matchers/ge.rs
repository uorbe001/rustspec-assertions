extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct Ge<T> {
    value: T,
    file_line: (&'static str, uint)
}

impl<T: PartialOrd + Show> Matcher<T> for Ge<T> {
    fn assert_check(&self, expected: T) -> bool {
        expected >= self.value
    }

    fn msg(&self, expected: T) -> String {
        format!("Expected {} to be greater or equal to {} but it was not.", expected, self.value)
    }

    fn negated_msg(&self, expected: T) -> String {
        format!("Expected {} NOT to be greater or equal to {} but it was.", expected, self.value)
    }

    fn get_file_line(&self) -> (&'static str, uint) {
        self.file_line
    }
}

pub fn be_ge<T: PartialOrd + Show>(value: T, file_line: (&'static str, uint)) -> Box<Ge<T>> {
    box Ge { value: value, file_line: file_line }
}

#[macro_export]
pub macro_rules! be_ge(
    ($value:expr) => (
        be_ge($value.clone(), (file!(), expand_line!()))
    );
)
