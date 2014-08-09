extern crate core;

use self::core::fmt::Show;

pub trait Matcher <T: Show> {
    fn assert_check(&self, expected: T) -> bool;
    fn msg(&self, expected: T) -> String;
    fn negated_msg(&self, expected: T) -> String;
}
