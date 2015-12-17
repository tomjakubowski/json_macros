#![feature(plugin)]
#![plugin(json_macros)]

extern crate serde_json;

pub fn main() {
    let x = 123i32;
    println!("{}",
             serde_json::to_string_pretty(&json!({ // object literal
        "foo": "foooooo", // string literal keys and values
        "bar": [true, null, 123, 123.4], // array, boolean, null, numeric literals
        "quux": { // nest as deeply as you like
            "a": [1, 2, 3, 4],
            "b": { "a": null },
            "c": false
        },
        "waldo": (192 - x) // wrap in parens to splice ToJson expressions directly
    }))
                 .unwrap());
}
