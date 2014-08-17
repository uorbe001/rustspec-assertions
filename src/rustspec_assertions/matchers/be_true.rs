extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct BeTrue {
    file_line: (&'static str, uint)
}

impl Matcher<bool> for BeTrue {
    fn assert_check(&self, expected: bool) -> bool {
        expected == true
    }

    fn msg(&self, expected: bool) -> String {
        format!("Expected {} to be true but it was not.", expected)
    }

    fn negated_msg(&self, expected: bool) -> String {
        format!("Expected {} NOT to be true but it was.", expected)
    }

    fn get_file_line(&self) -> (&'static str, uint) {
        self.file_line
    }
}

pub fn be_true(file_line: (&'static str, uint)) -> Box<BeTrue> {
    box BeTrue { file_line: file_line }
}

#[macro_export]
pub macro_rules! be_true(
    () => (
        be_true((file!(), expand_line!()))
    );
)
