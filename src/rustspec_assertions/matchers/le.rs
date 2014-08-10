extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct Le<T> {
    value: T,
    file_line: (&'static str, uint)
}

impl<T: PartialOrd + Show> Matcher<T> for Le<T> {
    fn assert_check(&self, expected: T) -> bool {
        expected <= self.value
    }

    fn msg(&self, expected: T) -> String {
        format!("Expected {} to be less or equal to {} but it was not.", self.value, expected)
    }

    fn negated_msg(&self, expected: T) -> String {
        format!("Expected {} NOT to be less or equal to {} but it was.", self.value, expected)
    }

    fn get_file_line(&self) -> (&'static str, uint) {
        self.file_line
    }
}

pub fn be_le<T: PartialOrd + Show>(value: T, file_line: (&'static str, uint)) -> Box<Le<T>> {
    box Le { value: value, file_line: file_line }
}

#[macro_export]
pub macro_rules! be_le(
    ($value:expr) => (
        be_le($value, (file!(), line!()))
    );
)
