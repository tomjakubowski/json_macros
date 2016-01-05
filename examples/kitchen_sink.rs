#![feature(plugin)]
#![plugin(json_macros)]

#[cfg(feature="with-rustc-serialize")]
extern crate rustc_serialize;

#[cfg(feature="with-serde")]
extern crate serde_json;

#[cfg(feature="with-rustc-serialize")]
fn make_pretty_json(x: i32) -> String {
    json!({ // object literal
        "foo": "foooooo", // string literal keys and values
        "bar": [true, null, 123, 123.4], // array, boolean, null, numeric literals
        "quux": { // nest as deeply as you like
            "a": [1, 2, 3, 4],
            "b": { "a": null },
            "c": false
        },
        "waldo": (192 - x) // wrap in parens to splice ToJson expressions directly
    }).pretty().to_string()
}

#[cfg(feature="with-serde")]
fn make_pretty_json(x: i32) -> String {
    serde_json::to_string_pretty(&json!({ // object literal
        "foo": "foooooo", // string literal keys and values
        "bar": [true, null, 123, 123.4], // array, boolean, null, numeric literals
        "quux": { // nest as deeply as you like
            "a": [1, 2, 3, 4],
            "b": { "a": null },
            "c": false
        },
        "waldo": (192 - x) // wrap in parens to splice ToJson expressions directly
    })).unwrap()
}


pub fn main() {
    // See implementation for serde/rustc-serialize features above.
    println!("{}", make_pretty_json(1));
}
