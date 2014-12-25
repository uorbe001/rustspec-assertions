extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

#[deriving(Copy)]
pub struct BeNone {
    file_line: (&'static str, uint)
}

impl <T: Show> Matcher<Option<T>> for BeNone {
    fn assert_check(&self, expected: Option<T>) -> bool {
        expected.is_none()
    }

    fn msg(&self, expected: Option<T>) -> String {
        format!("Expected {} to be none but it was not.", expected)
    }

    fn negated_msg(&self, expected: Option<T>) -> String {
        format!("Expected {} NOT to be none but it was.", expected)
    }

    fn get_file_line(&self) -> (&'static str, uint) {
        self.file_line
    }
}

pub fn be_none(file_line: (&'static str, uint)) -> Box<BeNone> {
    box BeNone { file_line: file_line }
}

#[macro_export]
pub macro_rules! be_none(
    () => (
        be_none((file!(), expand_line!()))
    );
);
