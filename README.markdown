# json_macros

Construct JSON objects in Rust from JSON-like literals.

## Example

```rust
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
        "waldo": (192 - foo) // wrap in parens to splice ToJson expressions directly
    }).to_pretty_str());
}
```

## Caveats

* Suffixed numeric literals and negative numbers are currently broken, but can
  be worked around by wrapping them in `()`, as in `json!({ "a": (-1234i32) })`.
