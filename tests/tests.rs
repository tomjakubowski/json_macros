#![feature(plugin)]
#![feature(collections, core)]
#![plugin(json_macros)]

extern crate "rustc-serialize" as rustc_serialize;

use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};

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
    assert_eq!(json!([]), Json::Array(vec![]));
    assert_eq!(json!([null]), Json::Array(vec![().to_json()]));

    let foobar = Json::Array(vec!["foo".to_json(),
                                  "bar".to_json()]);
    assert_eq!(json!(["foo", "bar"]), foobar);

    let foobar = Json::Array(vec![
        "foo".to_json(),
        vec!["bar".to_json()].to_json(),
        "baz".to_json()
    ]);
    assert_eq!(json!(["foo", ["bar"], "baz"]), foobar);
}

#[test]
fn test_object_lit() {
    let empty = BTreeMap::new();
    assert_eq!(json!({}), Json::Object(empty));

    let mut foo_bar = BTreeMap::new();
    foo_bar.insert("foo".to_string(), json!("bar"));
    assert_eq!(json!({"foo": "bar"}), Json::Object(foo_bar));

    let mut foo_bar_baz_123 = BTreeMap::new();
    foo_bar_baz_123.insert("foo".to_string(), json!("bar"));
    foo_bar_baz_123.insert("baz".to_string(), json!(123));
    assert_eq!(json!({
        "foo": "bar",
        "baz": 123
    }), Json::Object(foo_bar_baz_123));

    let mut nested = BTreeMap::new();
    let mut bar_baz = BTreeMap::new();
    bar_baz.insert("bar".to_string(), json!("baz"));
    nested.insert("foo".to_string(), Json::Object(bar_baz));
    nested.insert("quux".to_string(), Json::Null);
    assert_eq!(json!({
        "foo": { "bar": "baz" },
        "quux": null
    }), Json::Object(nested));
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
