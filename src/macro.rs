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
use syntax::print::pprust;

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

fn tt_to_expr(cx: &ExtCtxt, sp: codemap::Span,
              tt: &ast::TokenTree) -> Option<Gc<ast::Expr>> {
    use syntax::ext::build::AstBuilder;

    match *tt {
        ast::TTTok(sp, ref tok) => token_to_expr(cx, sp, tok),
        ast::TTDelim(ref toks) => {
            match (**toks)[0] {
                // array
                ast::TTTok(sp, token::LBRACKET) => {
                    let exprs = match parse_array(cx, sp, toks.as_slice()) {
                        Some(e) => e,
                        None => return None
                    };
                    let exprs = cx.expr_vec(sp, exprs);
                    Some(quote_expr!(cx, {
                        {
                            let mut _vec = Vec::from_slice($exprs.as_slice());
                            ::serialize::json::List(_vec)
                        }
                    }))
                }
                ast::TTTok(sp, token::LBRACE) => {
                    cx.span_err(sp, "JSON objects not implemented");
                    None
                }
                ref tt => {
                    let pp = pprust::tt_to_string(tt);
                    let err = format!("unexpected `{}` in JSON", pp);
                    cx.span_err(best_span(sp, tt), err.as_slice());
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

fn parse_array(cx: &ExtCtxt, sp: codemap::Span,
               toks: &[ast::TokenTree]) -> Option<Vec<Gc<ast::Expr>>> {
    let mids = toks.slice(1, toks.len() - 1); // all but the []
    let mut exprs = Vec::with_capacity(mids.len() / 2);
    for (i, tt) in mids.iter().enumerate() {
        if i % 2 == 1 {
            match tt {
                &ast::TTTok(_, token::COMMA) => {
                    continue;
                }
                _ => {
                    let pp = pprust::tt_to_string(tt);
                    let err = format!("expected `,` but found: `{}`", pp);
                    cx.span_err(best_span(sp, tt), err.as_slice());
                    return None;
                }
            }
        }
        let expr = tt_to_expr(cx, sp, tt);
        let expr = match expr {
            Some(e) => e,
            None => return None,
        };
        exprs.push(expr);
    }
    Some(exprs)
}

fn token_to_expr(cx: &ExtCtxt, sp: codemap::Span,
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
            cx.span_err(sp, format!("unexpected `{}` in JSON", s).as_slice());
            None
        }
    }
}

fn best_span(sp: codemap::Span, tt: &ast::TokenTree) -> codemap::Span {
    let sp = match *tt {
        ast::TTTok(tok_sp, _) => tok_sp,
        ast::TTDelim(ref tts) => {
            match (**tts)[0] {
                ast::TTTok(bra_sp, _) => bra_sp,
                _ => sp
            }
        }
        _ => sp // the span passed into the function!
    };
    sp
}
