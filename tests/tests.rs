#![feature(phase)]
#[phase(plugin)] extern crate json_macros;
extern crate serialize;

use std::collections::TreeMap;
use serialize::json::{mod, ToJson};

#[test]
fn test_string_lit() {
    assert_eq!(json!("foo").as_string(), Some("foo"));
}

#[test]
fn test_num_lit() {
    assert_eq!(json!(1234).as_i64(), Some(1234));
    assert_eq!(json!(12345.).as_f64(), Some(12345.));
}

#[test]
fn test_null_lit() {
    assert!(json!(null).is_null());
}

#[test]
fn test_bool_lit() {
    assert_eq!(json!(true).as_boolean(), Some(true));
    assert_eq!(json!(false).as_boolean(), Some(false));
}

#[test]
fn test_array_lit() {
    assert_eq!(json!([]), json::List(vec![]));

    let foobar = json::List(vec!["foo".to_string().to_json(),
                                 "bar".to_string().to_json()]);
    assert_eq!(json!(["foo", "bar"]), foobar);

    let foobar = json::List(vec!["foo".to_string().to_json(),
                                 vec!["bar".to_string().to_json()].to_json()]);
    assert_eq!(json!(["foo", ["bar"]]), foobar);
}

#[test]
fn test_object_lit() {
    let empty = TreeMap::new();
    assert_eq!(json!({}), json::Object(empty));

    let mut foo_bar = TreeMap::new();
    foo_bar.insert("foo".to_string(), json!("bar"));
    assert_eq!(json!({"foo": "bar"}), json::Object(foo_bar));

    let mut foo_bar_baz_123 = TreeMap::new();
    foo_bar_baz_123.insert("foo".to_string(), json!("bar"));
    foo_bar_baz_123.insert("baz".to_string(), json!(123));
    assert_eq!(json!({
        "foo": "bar",
        "baz": 123
    }), json::Object(foo_bar_baz_123));

    let mut nested = TreeMap::new();
    let mut bar_baz = TreeMap::new();
    bar_baz.insert("bar".to_string(), json!("baz"));
    nested.insert("foo".to_string(), json::Object(bar_baz));
    nested.insert("quux".to_string(), json::Null);
    assert_eq!(json!({
        "foo": { "bar": "baz" },
        "quux": null,
    }), json::Object(nested));
}

#[test]
fn test_expr_insertion() {
    let hello = "hello world!";
    let json = json!({
        "message": (hello.into_string())
    });
    assert_eq!(json.find("message").and_then(|j| j.as_string()),
               Some(hello));
}
