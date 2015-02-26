use matchers::matcher::Matcher;

pub struct Equals<T> {
    value: T,
    file_line: (&'static str, usize)
}

impl<T: PartialEq> Matcher<T> for Equals<T> {
    fn assert_check(&self, expected: T) -> bool {
        expected == self.value
    }

    #[allow(unused_variables)] fn msg(&self, expected: T) -> String {
        format!("Expected {} to equal {} but it did not.", stringify!(expected), stringify!(self.value))
    }

    #[allow(unused_variables)] fn negated_msg(&self, expected: T) -> String {
        format!("Expected {} NOT to equal {} but it did.", stringify!(expected), stringify!(self.value))
    }

    fn get_file_line(&self) -> (&'static str, usize) {
        self.file_line
    }
}

pub fn eq<T: PartialEq>(value: T, file_line: (&'static str, usize)) -> Box<Equals<T>> {
    Box::new(Equals { value: value, file_line: file_line })
}

#[macro_export]
macro_rules! eq(
    ($value:expr) => (
        eq($value.clone(), (file!(), expand_line!()))
    );
);
