#![feature(phase)]

#[phase(plugin, link)] extern crate rustspec_assertions;

mod matchers {
    mod equals;
    mod gt;
    mod ge;
    mod lt;
    mod le;
    mod contain;
    mod be_true;
    mod be_false;
    mod be_some;
    mod be_none;
}
