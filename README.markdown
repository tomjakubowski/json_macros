# json_macros

[![Crates.io](https://img.shields.io/crates/v/json_macros.svg)]()
[![Build Status](https://travis-ci.org/tomjakubowski/json_macros.svg?branch=master)]
(https://travis-ci.org/tomjakubowski/json_macros)

Construct JSON objects in Rust from JSON-like literals.

## Dependency

Add to your `Cargo.toml`:

```toml
[dependencies.json_macros]
git = "https://github.com/tomjakubowski/json_macros"
```

Or, from the registry:
```toml
[dependencies]
# ...
json_macros = "*"
```

You'll also need to link with the `rustc-serialize` crate, where the Rust
JSON types live.

## Example

```rust
#![feature(plugin)]
#![plugin(json_macros)]

extern crate "rustc-serialize" as rustc_serialize;

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
    }).pretty().to_string());
}
```
