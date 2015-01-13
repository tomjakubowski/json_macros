#![crate_name="json_macros"]
#![crate_type="dylib"]
#![feature(plugin_registrar, quote)]
#![allow(unstable)]

extern crate rustc;
extern crate syntax;
extern crate "rustc-serialize" as rustc_serialize;

use syntax::ast::TokenTree;
use syntax::codemap::Span;

use syntax::ext::base::{ExtCtxt, MacResult, MacExpr};
use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("json", expand);
}

fn expand<'cx>(cx: &'cx mut ExtCtxt, _: Span, _: &[TokenTree]) -> Box<MacResult + 'cx> {
    let expr = quote_expr!(cx, {
        ::rustc_serialize::json::Json::Null
    });
    MacExpr::new(expr)
}
