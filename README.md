# json_macros

[![Crates.io](https://img.shields.io/crates/v/json_macros.svg)](https://crates.io/crates/json_macros)
[![Build Status](https://travis-ci.org/tomjakubowski/json_macros.svg?branch=master)]
(https://travis-ci.org/tomjakubowski/json_macros)

Construct [`serde_json`](https://github.com/serde-rs/json) `Value`s
(JSON objects) in Rust from JSON-like literals.

## Dependency

If you have [`cargo-edit`](https://github.com/killercup/cargo-edit) installed:
```shell
$ cargo add json_macros
```

Otherwise, add to your `Cargo.toml`:
```toml
[dependencies]
# ...
json_macros = "*"
```

Or:
```toml
[dependencies.json_macros]
git = "https://github.com/tomjakubowski/json_macros"
```

You'll also need to link with the `serde_json` crate, where the Rust
JSON types live.

## Example

```rust
#![feature(plugin)]
#![plugin(json_macros)]

extern crate serde_json;

pub fn main() {
    let x = 123i32;
    println!("{}", serde_json::to_string_pretty(&json!({ // object literal
        "foo": "foooooo", // string literal keys and values
        "bar": [true, null, 123, 123.4], // array, boolean, null, numeric literals
        "quux": { // nest as deeply as you like
          "a": [1, 2, 3, 4],
          "b": { "a": null },
          "c": false
        },
        "waldo": (192 - x) // wrap in parens to splice ToJson expressions directly
    })).unwrap());
}
```
