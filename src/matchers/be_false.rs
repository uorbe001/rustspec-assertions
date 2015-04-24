use matchers::matcher::Matcher;

#[derive(Copy, Clone)]
pub struct BeFalse {
    file_line: (&'static str, u32)
}

impl Matcher<bool> for BeFalse {
    #[allow(unused_variables)] fn assert_check(&self, expected: bool) -> bool {
        expected == false
    }

    #[allow(unused_variables)] fn msg(&self, expected: bool) -> String {
        format!("Expected {} to be false but it was not.", stringify!(expected))
    }

    #[allow(unused_variables)] fn negated_msg(&self, expected: bool) -> String {
        format!("Expected {} NOT to be false but it was.", stringify!(expected))
    }

    fn get_file_line(&self) -> (&'static str, u32) {
        self.file_line
    }
}

pub fn be_false(file_line: (&'static str, u32)) -> Box<BeFalse> {
    Box::new(BeFalse { file_line: file_line })
}

#[macro_export]
macro_rules! be_false(
    () => (
        be_false((file!(), expand_line!()))
    );
);
