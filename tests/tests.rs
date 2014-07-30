#![feature(phase)]
#[phase(plugin)] extern crate json_macros;
extern crate serialize;

#[test]
fn test_string_lit() {
    assert_eq!(json!("foo").as_string(), Some("foo"));
}

#[test]
fn test_num_lit() {
    assert_eq!(json!(1234).as_number(), Some(1234f64));
    // assert_eq!(json!(12345.).as_number(), Some(12345.6789f64));
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
