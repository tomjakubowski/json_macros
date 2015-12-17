#![feature(plugin)]
#![plugin(json_macros)]

extern crate serde;
extern crate serde_json;

use std::collections::BTreeMap;
use serde_json::value::{Value, to_value};

#[test]
fn test_string_lit() {
    assert_eq!(json!("foo").as_string(), Some("foo"));
}

#[test]
fn test_num_lit() {
    assert_eq!(json!(1234).as_i64(), Some(1234));
    assert_eq!(json!(-1234).as_i64(), Some(-1234));
    assert_eq!(json!(12345.).as_f64(), Some(12345.));
    assert_eq!(json!(-12345.6).as_f64(), Some(-12345.6));
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
    assert_eq!(json!([]), Value::Array(vec![]));
    assert_eq!(json!([null]), Value::Array(vec![to_value(&())]));

    let foobar = Value::Array(vec![to_value(&"foo"), to_value(&"bar")]);
    assert_eq!(json!(["foo", "bar"]), foobar);

    let foobar = Value::Array(vec![to_value(&"foo"),
                                   to_value(&vec![to_value(&"bar")]),
                                   to_value(&"baz")]);
    assert_eq!(json!(["foo", ["bar"], "baz"]), foobar);
}

#[test]
fn test_object_lit() {
    let empty = BTreeMap::new();
    assert_eq!(json!({}), Value::Object(empty));

    let mut foo_bar = BTreeMap::new();
    foo_bar.insert("foo".to_string(), json!("bar"));
    assert_eq!(json!({"foo": "bar"}), Value::Object(foo_bar));

    let mut foo_bar_baz_123 = BTreeMap::new();
    foo_bar_baz_123.insert("foo".to_string(), json!("bar"));
    foo_bar_baz_123.insert("baz".to_string(), json!(123));
    assert_eq!(json!({
        "foo": "bar",
        "baz": 123
    }),
               Value::Object(foo_bar_baz_123));

    let mut nested = BTreeMap::new();
    let mut bar_baz = BTreeMap::new();
    bar_baz.insert("bar".to_string(), json!("baz"));
    nested.insert("foo".to_string(), Value::Object(bar_baz));
    nested.insert("quux".to_string(), Value::Null);
    assert_eq!(json!({
        "foo": { "bar": "baz" },
        "quux": null
    }),
               Value::Object(nested));
}

#[test]
fn test_expr_insertion() {
    let hello = "hello world!";
    let json = json!({
        "message": (hello.to_string())
    });
    assert_eq!(json.find("message").and_then(|j| j.as_string()),
               Some(hello));
}
