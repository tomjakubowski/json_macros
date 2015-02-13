use syntax::ast::TokenTree;
use syntax::codemap::Span;
use syntax::ptr::P;

use syntax::ast::Expr;
use syntax::ext::base::{ExtCtxt, MacResult, MacExpr};
use syntax::parse::parser::Parser;
use syntax::parse::token::Token;

pub fn expand<'cx>(cx: &'cx mut ExtCtxt, _: Span,
               tts: &[TokenTree]) -> Box<MacResult + 'cx> {
    let mut parser = cx.new_parser_from_tts(tts);
    let expr = parse_json(cx, &mut parser);
    if &parser.token != &Token::Eof {
        cx.span_fatal(parser.span, "expected end of `json!` macro invocation");
    }
    MacExpr::new(expr)
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
            parser.bump();
            let r_bracket = Token::CloseDelim(DelimToken::Bracket);
            let exprs = parser.parse_seq_to_end(&r_bracket, comma_sep!(), |p| {
                parse_json(cx, p)
            });
            let exprs = cx.expr_vec(orig_span, exprs);
            quote_expr!(cx, {{
                use ::std::boxed::Box;
                let xs: Box<[_]> = Box::new($exprs);
                ::rustc_serialize::json::Json::Array(xs.into_vec())
            }})
        },
        &Token::OpenDelim(DelimToken::Brace) => {
            parser.bump();
            let r_brace = Token::CloseDelim(DelimToken::Brace);
            let kvs = parser.parse_seq_to_end(&r_brace, comma_sep!(), |p| {
                let (istr, _) = p.parse_str();
                let s = &*istr;
                p.expect(&Token::Colon);
                let key = quote_expr!(cx, {{
                    use ::std::borrow::ToOwned;
                    $s.to_owned()
                }});
                (key, parse_json(cx, p))
            });
            let ob = quote_expr!(cx, _ob);
            let mut stmts = vec![];
            for &(ref key, ref value) in kvs.iter() {
                stmts.push(quote_stmt!(cx, $ob.insert($key, $value)));
            }
            quote_expr!(cx, {{
                let mut $ob = ::std::collections::BTreeMap::new();
                $stmts;
                ::rustc_serialize::json::Json::Object($ob)
            }})
        },
        &Token::OpenDelim(DelimToken::Paren) => {
            let expr = parser.parse_expr();
            quote_expr!(cx, {{
                use ::rustc_serialize::json::ToJson;
                ($expr).to_json()
            }})
        },
        &Token::Ident(id, IdentStyle::Plain) if id.as_str() == "null" => {
            parser.bump();
            quote_expr!(cx, { ::rustc_serialize::json::Json::Null })
        },
        _ => { // TODO: investigate can_begin_expr (maybe eliminate need for parens)?
            let expr = parser.parse_literal_maybe_minus();
            quote_expr!(cx, {{
                use ::rustc_serialize::json::ToJson;
                ($expr).to_json()
            }})
        }
    }
}
