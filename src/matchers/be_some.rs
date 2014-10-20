extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct BeSome {
    file_line: (&'static str, uint)
}

impl <T: Show> Matcher<Option<T>> for BeSome {
    fn assert_check(&self, expected: Option<T>) -> bool {
        expected.is_some()
    }

    fn msg(&self, expected: Option<T>) -> String {
        format!("Expected {} to be some but it was not.", expected)
    }

    fn negated_msg(&self, expected: Option<T>) -> String {
        format!("Expected {} NOT to be some but it was.", expected)
    }

    fn get_file_line(&self) -> (&'static str, uint) {
        self.file_line
    }
}

pub fn be_some(file_line: (&'static str, uint)) -> Box<BeSome> {
    box BeSome { file_line: file_line }
}

#[macro_export]
pub macro_rules! be_some(
    () => (
        be_some((file!(), expand_line!()))
    );
)
