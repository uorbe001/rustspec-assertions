use matchers::matcher::Matcher;

pub struct Lt<T> {
    value: T,
    file_line: (&'static str, u32)
}

impl<T: PartialOrd> Matcher<T> for Lt<T> {
    fn assert_check(&self, expected: T) -> bool {
        expected < self.value
    }

    #[allow(unused_variables)] fn msg(&self, expected: T) -> String {
        format!("Expected {} to be less than {} but it was not.", stringify!(expected), stringify!(self.value))
    }

    #[allow(unused_variables)] fn negated_msg(&self, expected: T) -> String {
        format!("Expected {} NOT to be less than {} but it was.", stringify!(expected), stringify!(self.value))
    }

    fn get_file_line(&self) -> (&'static str, u32) {
        self.file_line
    }
}

pub fn be_lt<T: PartialOrd>(value: T, file_line: (&'static str, u32)) -> Box<Lt<T>> {
    Box::new(Lt { value: value, file_line: file_line })
}

#[macro_export]
macro_rules! be_lt(
    ($value:expr) => (
        be_lt($value.clone(), (file!(), expand_line!()))
    );
);
