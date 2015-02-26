use matchers::matcher::Matcher;

pub struct Contain<T: PartialEq> {
    value: T,
    file_line: (&'static str, usize)
}

impl <T: PartialEq> Matcher<Vec<T>> for Contain<T> {
    fn assert_check(&self, expected: Vec<T>) -> bool {
        expected.contains(&self.value)
    }

    #[allow(unused_variables)] fn msg(&self, expected: Vec<T>) -> String {
        format!("Expected {} to contain {} but it did not.", stringify!(expected), stringify!(self.value))
    }

    #[allow(unused_variables)] fn negated_msg(&self, expected: Vec<T>) -> String {
        format!("Expected {} NOT to contain {} but it did.", stringify!(expected), stringify!(self.value))
    }

    fn get_file_line(&self) -> (&'static str, usize) {
        self.file_line
    }
}

pub fn contain<T: PartialEq>(value: T, file_line: (&'static str, usize)) -> Box<Contain<T>> {
    Box::new(Contain { value: value, file_line: file_line })
}

#[macro_export]
macro_rules! contain(
    ($value:expr) => (
        contain($value.clone(), (file!(), expand_line!()))
    );
);
