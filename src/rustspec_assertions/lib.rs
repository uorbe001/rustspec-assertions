#![crate_name="rustspec_assertions"]
#![crate_type="lib"]

pub use matchers::equals::eq;
pub use equal = matchers::equals::eq;
pub use expect::expect;

pub mod expect;
mod matchers {
    pub mod equals;
    pub mod matcher;
}
