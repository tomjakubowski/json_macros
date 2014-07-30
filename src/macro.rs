#![crate_name="json_macros"]
#![crate_type="dylib"]
#![feature(phase, plugin_registrar, macro_rules, quote)]

#[phase(plugin, link)] extern crate log;
extern crate rustc;
extern crate syntax;
extern crate serialize;

use std::gc::Gc;
use rustc::plugin::Registry;

use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{ExtCtxt, MacResult, MacExpr, DummyResult};
use syntax::parse::token;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("json", expand);
}

fn expand(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    debug!("JSON token tree {}", tts);

    let tt = tts.get(0).expect("FIXME"); // FIXME
    let expr = match tt_to_expr(cx, sp, tt) {
        Some(e) => e,
        None => return DummyResult::expr(sp)
    };
    MacExpr::new(expr)
}

fn tt_to_expr(cx: &mut ExtCtxt, sp: codemap::Span,
              tt: &ast::TokenTree) -> Option<Gc<ast::Expr>> {
    match *tt {
        ast::TTTok(sp, ref tok) => token_to_expr(cx, sp, tok),
        ast::TTDelim(ref toks) => {
            match (**toks)[0] {
                // array
                ast::TTTok(_, token::LBRACKET) => {
                    cx.span_err(sp, "arrays not implemented");
                    None
                }
                ast::TTTok(_, token::LBRACE) => {
                    cx.span_err(sp, "objects not implemented");
                    None
                }
                _ => {
                    cx.span_err(sp, "something something FIXME");
                    None
                }
            }
        }
        _ => {
            cx.span_err(sp, "something something FIXME");
            None
        }
    }
}

fn token_to_expr(cx: &mut ExtCtxt, sp: codemap::Span,
                 tok: &token::Token) -> Option<Gc<ast::Expr>> {
    use std::from_str::FromStr;
    use syntax::print::pprust;

    match *tok {
        token::LIT_STR(ref n) => {
            let s = n.as_str();
            Some(quote_expr!(cx, {
                ::serialize::json::String($s.to_string())
            }))
        }
        token::LIT_INTEGER(ref n) => {
            let s = n.as_str();
            let n: i64 = FromStr::from_str(s).unwrap(); // FIXME: is i64 right?
            Some(quote_expr!(cx, {
                ::serialize::json::Number($n as f64)
            }))
        }
        token::IDENT(ref n, false) if n.as_str() == "null" => {
            Some(quote_expr!(cx, {
                ::serialize::json::Null
            }))
        }
        ref t @ token::IDENT(..) if token::is_keyword(token::keywords::True, t) => {
            Some(quote_expr!(cx, { ::serialize::json::Boolean(true) }))
        }
        ref t @ token::IDENT(..) if token::is_keyword(token::keywords::False, t) => {
            Some(quote_expr!(cx, { ::serialize::json::Boolean(false) }))
        }
        _ => {
            let tt = ast::TTTok(sp, tok.clone());
            let s = pprust::tt_to_string(&tt);
            cx.span_err(sp, format!("couldn't interpret `{}` as JSON", s).as_slice());
            None
        }
    }
}
