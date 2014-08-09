extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct Expect<T> {
    value: T
}

impl<T: Show + Clone> Expect<T> {
    pub fn to(&self, other: Box<Matcher<T>>) {
        if !other.assert_check(self.value.clone()) {
            self.fail(other)
        }
    }

    pub fn not_to(&self, other: Box<Matcher<T>>) {
        if other.assert_check(self.value.clone()) {
            self.fail(other)
        }
    }

    fn fail(&self, other: Box<Matcher<T>>) {
        // let mut t = term::stdout().unwrap();
        // t.fg(term::color::RED).unwrap();
        fail!(other.negated_msg(self.value.clone()));
    }
}

pub fn expect<T: Eq + Ord + Show>(value: T) -> Expect<T> {
    Expect { value: value }
}
