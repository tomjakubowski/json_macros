use syntax::ast::TokenTree;
use syntax::codemap::Span;
use syntax::ptr::P;

use syntax::ast::Expr;
use syntax::ext::base::{ExtCtxt, MacResult, MacEager};
use syntax::parse::parser::Parser;
use syntax::parse::token::Token;

pub fn expand<'cx>(cx: &'cx mut ExtCtxt, _: Span, tts: &[TokenTree]) -> Box<MacResult + 'cx> {
    let mut parser = cx.new_parser_from_tts(tts);
    let expr = parse_json(cx, &mut parser);
    if &parser.token != &Token::Eof {
        cx.span_fatal(parser.span, "expected end of `json!` macro invocation");
    }
    MacEager::expr(expr)
}

fn parse_json(cx: &ExtCtxt, parser: &mut Parser) -> P<Expr> {
    use syntax::ext::build::AstBuilder;
    use syntax::parse::token::{DelimToken, IdentStyle};

    macro_rules! comma_sep {
        () =>  {
            ::syntax::parse::common::SeqSep {
                sep: Some(Token::Comma),
                trailing_sep_allowed: true // we could be JSON pedants...
            }
        }
    }

    let orig_span = parser.span;

    match &parser.token {
        &Token::OpenDelim(DelimToken::Bracket) => {
            let _ = parser.bump();
            let r_bracket = Token::CloseDelim(DelimToken::Bracket);
            let exprs = parser.parse_seq_to_end(&r_bracket,
                                                comma_sep!(),
                                                |p| Ok(parse_json(cx, p)))
                .ok()
                .unwrap();
            let exprs = cx.expr_vec(orig_span, exprs);
            quote_expr!(cx, {
                use ::std::boxed::Box;
                let xs: Box<[_]> = Box::new($exprs);
                serde_json::Value::Array(xs.into_vec())
            })
        }
        &Token::OpenDelim(DelimToken::Brace) => {
            let _ = parser.bump();
            let r_brace = Token::CloseDelim(DelimToken::Brace);
            let kvs = parser.parse_seq_to_end(&r_brace, comma_sep!(), |p| {
                let (istr, _) = p.parse_str().ok().unwrap();
                let s = &*istr;
                let _ = p.expect(&Token::Colon);
                let key = quote_expr!(cx, {
                    use ::std::borrow::ToOwned;
                    $s.to_owned()
                });
                Ok((key, parse_json(cx, p)))
            })
                .ok()
                .unwrap();
            let mut insertions = vec![];
            // Can't use `quote_stmt!()` and interpolate a vector of
            // statements, seemingly.  Should consider filing a bug
            // upstream.
            for &(ref key, ref value) in kvs.iter() {
                insertions.push(quote_expr!(cx, {
                    _ob.insert($key, $value);
                }));
            }
            let expr = quote_expr!(cx, {
                let mut _ob = ::std::collections::BTreeMap::new();
                $insertions;
                ::serde_json::Value::Object(_ob)
            });
            expr
        }
        &Token::OpenDelim(DelimToken::Paren) => {
            let expr = parser.parse_expr().unwrap();
            quote_expr!(cx, {{
                ::serde_json::to_value(&$expr)
            }})
        }
        &Token::Ident(id, IdentStyle::Plain) if id.name.as_str() == "null" => {
            let _ = parser.bump();
            quote_expr!(cx, {
                ::serde_json::Value::Null
            })
        }
        _ => {
            // TODO: investigate can_begin_expr (maybe eliminate need for parens)?
            let expr = parser.parse_pat_literal_maybe_minus().ok().unwrap();
            quote_expr!(cx, {{
                ::serde_json::to_value(&$expr)
            }})
        }
    }
}
