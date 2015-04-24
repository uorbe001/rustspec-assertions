use matchers::matcher::Matcher;

#[derive(Copy, Clone)]
pub struct BeSome {
    file_line: (&'static str, u32)
}

impl <T> Matcher<Option<T>> for BeSome {
    #[allow(unused_variables)] fn assert_check(&self, expected: Option<T>) -> bool {
        expected.is_some()
    }

    #[allow(unused_variables)] fn msg(&self, expected: Option<T>) -> String {
        format!("Expected {} to be some but it was not.", stringify!(expected))
    }

    #[allow(unused_variables)] fn negated_msg(&self, expected: Option<T>) -> String {
        format!("Expected {} NOT to be some but it was.", stringify!(expected))
    }

    fn get_file_line(&self) -> (&'static str, u32) {
        self.file_line
    }
}

pub fn be_some(file_line: (&'static str, u32)) -> Box<BeSome> {
    Box::new(BeSome { file_line: file_line })
}

#[macro_export]
macro_rules! be_some(
    () => (
        be_some((file!(), expand_line!()))
    );
);
