# json_macros

Construct JSON objects in Rust from JSON-like literals.

## Example

```rust
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
```

## Caveats

* Suffixed and negative numeric literals are currently broken, but can
  be worked around by wrapping them in `()`, as in `json!({ "a": (-1234i32) })`.
