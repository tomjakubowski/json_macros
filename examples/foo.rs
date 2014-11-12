#![feature(phase)]
#[phase(plugin)] extern crate json_macros;

extern crate serialize;

pub fn main() {
    let foo = 123i32;
    println!("{}", json!({ // object literal
        "foo": "foooooo", // string literal keys and values
        "bar": [true, null, 1234, 1234.5], // array, boolean, null, numeric literals
        "quux": { // nest as deeply as you like
            "a": [1, 2, 3, 4],
            "b": { "a": null },
            "c": false
        },
        "waldo": (foo - 123) // wrap in parentheses to splice ToJson expressions directly
    }).to_pretty_str());
}
