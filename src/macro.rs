#![crate_name="json_macros"]
#![crate_type="dylib"]
#![feature(phase, plugin_registrar, macro_rules, quote)]

#[phase(plugin, link)] extern crate log;
extern crate rustc;
extern crate syntax;
extern crate serialize;

use std::gc::Gc;
use syntax::ast::TokenTree;
use syntax::codemap::Span;
use rustc::plugin::Registry;

use syntax::ast;
use syntax::ext::base::{ExtCtxt, MacResult, MacExpr, DummyResult};
use syntax::parse::token;
use syntax::print::pprust;

type GcExpr = Gc<ast::Expr>;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("json", expand);
}

fn expand(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult> {
    debug!("JSON token tree {}", tts);

    let tt = tts.get(0).expect("FIXME"); // FIXME
    let expr = match tt_to_expr(cx, sp, tt) {
        Some(e) => e,
        None => return DummyResult::expr(sp)
    };
    MacExpr::new(expr)
}

fn tt_to_expr(cx: &ExtCtxt, sp: Span, tt: &TokenTree) -> Option<GcExpr> {
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
                // object
                ast::TTTok(sp, token::LBRACE) => {
                    let items = match parse_object(cx, sp, toks.as_slice()) {
                        Some(i) => i,
                        None => return None
                    };
                    let ob = quote_expr!(cx, _ob);
                    let mut stmts = vec![];
                    for &(key, value) in items.iter() {
                        stmts.push(quote_stmt!(cx, $ob.insert($key, $value)));
                    }

                    let res = quote_expr!(cx, {
                        {
                            let mut $ob = ::std::collections::TreeMap::new();
                            $stmts;
                            ::serialize::json::Object($ob)
                        }
                    });
                    Some(res)
                }
                ref tt => {
                    let pp = pprust::tt_to_string(tt);
                    let err = format!("unexpected `{}` in JSON", pp);
                    cx.span_err(best_span(sp, tt), err.as_slice());
                    None
                }
            }
        }
        // vvv are these code paths even reachable? copied this from
        // brainfuck_macros
        ast::TTSeq(sp, _, _, _) => {
            cx.span_err(sp, "`json!` doesn't support sequences");
            None
        }
        ast::TTNonterminal(sp, _) => {
            cx.span_err(sp, "`json!` doesn't support non-terminals");
            None
        }
    }
}

fn parse_array(cx: &ExtCtxt, sp: Span, tts: &[TokenTree]) -> Option<Vec<GcExpr>> {
    let mids = tts.slice(1, tts.len() - 1); // all but the []
    let mut exprs = Vec::with_capacity(mids.len() / 2);
    for (i, tt) in mids.iter().enumerate() {
        if i % 2 == 1 {
            match tt {
                &ast::TTTok(_, token::COMMA) => {
                    continue;
                }
                _ => {
                    expected_but_found(cx, sp, "`,`", tt);
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

#[allow(dead_code)]
fn parse_object(cx: &ExtCtxt, sp: Span, tts: &[TokenTree]) -> Option<Vec<(GcExpr, GcExpr)>> {
    use syntax::ast::TTTok;
    use tok = syntax::parse::token;

    macro_rules! comma {
        () => {
            ::syntax::ast::TTTok(_, ::syntax::parse::token::COMMA)
        }
    }

    macro_rules! colon {
        () => {
            ::syntax::ast::TTTok(_, ::syntax::parse::token::COLON)
        };
        ($sp:ident) => {
            ::syntax::ast::TTTok($sp, ::syntax::parse::token::COLON)
        }
    }

    let mids = tts.slice(1, tts.len() - 1); // all but the {}
    let mut items = Vec::with_capacity(mids.len() / 4);
    if tts.len() == 0 {
        return Some(items);
    }

    // horrible
    for entry in mids.chunks(4) {
        let item = match entry {
            // "foo": VALUE | "foo": VALUE,
            [TTTok(_, tok::LIT_STR(ref n)), colon!(), ref v] |
            [TTTok(_, tok::LIT_STR(ref n)), colon!(), ref v, comma!()] => {
                let k = n.as_str();
                let v = tt_to_expr(cx, sp, v);
                if v.is_none() {
                    return None;
                }
                let k = quote_expr!(cx, $k.to_string());
                let v = quote_expr!(cx, $v);
                (k, v)
            }
            // "foo": VALUE X
            [TTTok(_, tok::LIT_STR(_)), colon!(), _, ref tt] => {
                expected_but_found(cx, sp, "`,`", tt);
                return None;
            }
            [TTTok(_, tok::LIT_STR(_)), colon!(sp)] => {
                cx.span_err(sp, "found `:` but no value afterwards");
                return None;
            }
            [TTTok(_, tok::LIT_STR(_)), ref tt, ..] => {
                expected_but_found(cx, sp, "`:`", tt);
                return None;
            }
            [TTTok(sp, tok::LIT_STR(_))] => {
                cx.span_err(sp, "found name but no colon-value afterwards");
                return None;
            }
            [ref tt, ..] => {
                expected_but_found(cx, sp, "string literal", tt);
                return None;
            }
            [] => unreachable!(), // chunks() never returns an empty slice
            // _ => unimplemented!()
        };
        items.push(item);
    }

    Some(items)
}

fn token_to_expr(cx: &ExtCtxt, sp: Span, tok: &token::Token) -> Option<GcExpr> {
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
        token::LIT_FLOAT(ref _n) => {
            cx.span_err(sp, format!("json! does not yet support float literals").as_slice());
            None
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

fn best_span(sp: Span, tt: &TokenTree) -> Span {
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

fn expected_but_found(cx: &ExtCtxt, sp: Span, expected: &str, found: &TokenTree) {
    let pp = pprust::tt_to_string(found);
    let err = format!("expected {} but found: `{}`", expected, pp);
    cx.span_err(best_span(sp, found), err.as_slice());
}
