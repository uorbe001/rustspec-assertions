pub trait Matcher <T> {
    fn assert_check(&self, expected: T) -> bool;
    fn msg(&self, expected: T) -> String;
    fn negated_msg(&self, expected: T) -> String;
    fn get_file_line(&self) -> (&'static str, usize);
}

