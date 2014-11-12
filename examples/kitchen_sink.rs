#![feature(phase)]
#[phase(plugin)] extern crate json_macros;

extern crate serialize;

pub fn main() {
    let x = 123i32;
    println!("{}", json!({ // object literal
        "foo": "foooooo", // string literal keys and values
        "bar": [true, null, 123, 123.4], // array, boolean, null, numeric literals
        "quux": { // nest as deeply as you like
            "a": [1, 2, 3, 4],
            "b": { "a": null },
            "c": false
        },
        "waldo": (192 - x) // wrap in parens to splice ToJson expressions directly
    }).to_pretty_str());
}
