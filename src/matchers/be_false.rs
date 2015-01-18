extern crate core;

use matchers::matcher::Matcher;

pub struct BeFalse {
    file_line: (&'static str, usize)
}

impl Matcher<bool> for BeFalse {
    fn assert_check(&self, expected: bool) -> bool {
        expected == false
    }

    fn msg(&self, expected: bool) -> String {
        format!("Expected {:?} to be false but it was not.", expected)
    }

    fn negated_msg(&self, expected: bool) -> String {
        format!("Expected {:?} NOT to be false but it was.", expected)
    }

    fn get_file_line(&self) -> (&'static str, usize) {
        self.file_line
    }
}

pub fn be_false(file_line: (&'static str, usize)) -> Box<BeFalse> {
    Box::new(BeFalse { file_line: file_line })
}

#[macro_export]
pub macro_rules! be_false(
    () => (
        be_false((file!(), expand_line!()))
    );
);
