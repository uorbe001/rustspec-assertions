extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct Lt<T> {
    value: T,
    file_line: (&'static str, uint)
}

impl<T: PartialOrd + Show> Matcher<T> for Lt<T> {
    fn assert_check(&self, expected: T) -> bool {
        expected < self.value
    }

    fn msg(&self, expected: T) -> String {
        format!("Expected {} to be less than {} but it was not.", exptected, self.value)
    }

    fn negated_msg(&self, expected: T) -> String {
        format!("Expected {} NOT to be less than {} but it was.", expexted, self.value)
    }

    fn get_file_line(&self) -> (&'static str, uint) {
        self.file_line
    }
}

pub fn be_lt<T: PartialOrd + Show>(value: T, file_line: (&'static str, uint)) -> Box<Lt<T>> {
    box Lt { value: value, file_line: file_line }
}

#[macro_export]
pub macro_rules! be_lt(
    ($value:expr) => (
        be_lt($value, (file!(), expand_line!()))
    );
)
