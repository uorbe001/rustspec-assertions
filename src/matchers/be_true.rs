use matchers::matcher::Matcher;

#[derive(Copy)]
pub struct BeTrue {
    file_line: (&'static str, usize)
}

impl Matcher<bool> for BeTrue {
    #[allow(unused_variables)] fn assert_check(&self, expected: bool) -> bool {
        expected == true
    }

    #[allow(unused_variables)] fn msg(&self, expected: bool) -> String {
        format!("Expected {} to be true but it was not.", stringify!(expected))
    }

    #[allow(unused_variables)] fn negated_msg(&self, expected: bool) -> String {
        format!("Expected {} NOT to be true but it was.", stringify!(expected))
    }

    fn get_file_line(&self) -> (&'static str, usize) {
        self.file_line
    }
}

pub fn be_true(file_line: (&'static str, usize)) -> Box<BeTrue> {
    Box::new(BeTrue { file_line: file_line })
}

#[macro_export]
pub macro_rules! be_true(
    () => (
        be_true((file!(), expand_line!()))
    );
);
