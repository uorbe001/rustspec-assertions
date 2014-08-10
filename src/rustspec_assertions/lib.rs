#![crate_name="rustspec_assertions"]
#![crate_type="lib"]

pub use matchers::equals::eq;
pub use equal = matchers::equals::eq;
pub use matchers::gt::be_gt;
pub use be_greater_than = matchers::gt::be_gt;
pub use matchers::ge::be_ge;
pub use be_greater_or_equal_to = matchers::ge::be_ge;
pub use matchers::lt::be_lt;
pub use be_less_than = matchers::lt::be_lt;
pub use expect::expect;

pub mod expect;
mod matchers {
    pub mod equals;
    pub mod gt;
    pub mod ge;
    pub mod lt;
    pub mod matcher;
}
