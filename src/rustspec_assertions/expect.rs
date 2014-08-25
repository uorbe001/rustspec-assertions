#![macro_escape]
extern crate core;

use self::core::fmt::Show;
use matchers::matcher::Matcher;

pub struct Expect<T> {
    value: T,
    negated: bool
}

impl<T: Show + Clone> Expect<T> {
    pub fn to(&self, other: Box<Matcher<T>>) {
        let mut cond = !other.assert_check(self.value.clone());

        if self.negated {
            cond = !cond;
        }

        if cond {
            self.fail(other)
        }
    }

    pub fn not_to(&mut self, other: Box<Matcher<T>>) {
        self.negated = true;
        self.to(other);
    }

    fn fail(&self, other: Box<Matcher<T>>) {
        let mut msg;

        if self.negated {
            msg = other.negated_msg(self.value.clone());
        } else {
            msg = other.msg(self.value.clone());
        }

        ::std::rt::begin_unwind(msg, &other.get_file_line());
    }
}

pub fn expect<T: Show + Clone>(value: &T) -> Expect<T> {
    Expect { value: value.clone(), negated: false }
}
