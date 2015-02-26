use matchers::matcher::Matcher;

pub struct Ge<T> {
    value: T,
    file_line: (&'static str, usize)
}

impl<T: PartialOrd> Matcher<T> for Ge<T> {
    fn assert_check(&self, expected: T) -> bool {
        expected >= self.value
    }

    #[allow(unused_variables)] fn msg(&self, expected: T) -> String {
        format!("Expected {} to be greater or equal to {} but it was not.", stringify!(expected), stringify!(self.value))
    }

    #[allow(unused_variables)] fn negated_msg(&self, expected: T) -> String {
        format!("Expected {} NOT to be greater or equal to {} but it was.", stringify!(expected), stringify!(self.value))
    }

    fn get_file_line(&self) -> (&'static str, usize) {
        self.file_line
    }
}

pub fn be_ge<T: PartialOrd>(value: T, file_line: (&'static str, usize)) -> Box<Ge<T>> {
    Box::new(Ge { value: value, file_line: file_line })
}

#[macro_export]
macro_rules! be_ge(
    ($value:expr) => (
        be_ge($value.clone(), (file!(), expand_line!()))
    );
);
