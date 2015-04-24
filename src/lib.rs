#![crate_name="rustspec_assertions"]
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, std_misc)]

extern crate syntax;
extern crate rustc;

pub use matchers::equals::Equals;
pub use matchers::equals::eq;
pub use matchers::gt::Gt;
pub use matchers::gt::be_gt;
pub use matchers::ge::Ge;
pub use matchers::ge::be_ge;
pub use matchers::lt::Lt;
pub use matchers::lt::be_lt;
pub use matchers::le::Le;
pub use matchers::le::be_le;
pub use matchers::contain::Contain;
pub use matchers::contain::contain;
pub use matchers::be_true::BeTrue;
pub use matchers::be_true::be_true;
pub use matchers::be_false::BeFalse;
pub use matchers::be_false::be_false;
pub use matchers::be_some::BeSome;
pub use matchers::be_some::be_some;
pub use matchers::be_none::BeNone;
pub use matchers::be_none::be_none;
pub use expect::expect;

// use syntax::ext::base;
// use syntax::ext::base::{ExtCtxt, MacResult};
// use syntax::codemap::{Span};
// use syntax::ast::TokenTree;
// use rustc::plugin::Registry;
// use syntax::ext::build::AstBuilder;

// #[plugin_registrar]
// pub fn plugin_registrar(registry: &mut Registry) {
//     registry.register_macro("expand_line", expand_line);
// }

// // report the appropiate line when assertion is summoned inside a macro.
// pub fn expand_line(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult> {
//     base::check_zero_tts(cx, sp, tts, "expand_line!");

//     let topmost = cx.backtrace().unwrap()
//         .call_site.expn_info.unwrap();

//     let loc = cx.codemap().lookup_char_pos(topmost.call_site.lo);

//     base::MacExpr::new(cx.expr_usize(topmost.call_site, loc.line))
// }

#[macro_export]
macro_rules! expand_line(
    () => (
        line!() as u32
    );
);


pub mod expect;
mod matchers {
    pub mod equals;
    pub mod gt;
    pub mod ge;
    pub mod lt;
    pub mod le;
    pub mod contain;
    pub mod be_true;
    pub mod be_false;
    pub mod be_some;
    pub mod be_none;
    pub mod matcher;
}
