use matchers::matcher::Matcher;

#[derive(Copy, Clone)]
pub struct BeNone {
    file_line: (&'static str, u32)
}

impl <T> Matcher<Option<T>> for BeNone {
    #[allow(unused_variables)] fn assert_check(&self, expected: Option<T>) -> bool {
        expected.is_none()
    }

    #[allow(unused_variables)] fn msg(&self, expected: Option<T>) -> String {
        format!("Expected {} to be none but it was not.", stringify!(expected))
    }

    #[allow(unused_variables)] fn negated_msg(&self, expected: Option<T>) -> String {
        format!("Expected {} NOT to be none but it was.", stringify!(expected))
    }

    fn get_file_line(&self) -> (&'static str, u32) {
        self.file_line
    }
}

pub fn be_none(file_line: (&'static str, u32)) -> Box<BeNone> {
    Box::new(BeNone { file_line: file_line })
}

#[macro_export]
macro_rules! be_none(
    () => (
        be_none((file!(), expand_line!()))
    );
);
