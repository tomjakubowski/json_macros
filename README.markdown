# json_macros

[![Crates.io](https://img.shields.io/crates/v/json_macros.svg)](https://crates.io/crates/json_macros)
[![Build Status](https://travis-ci.org/tomjakubowski/json_macros.svg?branch=master)]
(https://travis-ci.org/tomjakubowski/json_macros)

``` rust
let properties = json! {
    "menu": {
        "id": "file",
        "value": "File",
        "popup": {
            "menuitem": [
                {"value": "New", "onclick": "CreateNewDoc()"},
                {"value": "Open", "onclick": "OpenDoc()"},
                {"value": "Close", "onclick": "CloseDoc()"}
            ]
        }
    }
};

let menu_value = properties.find_path(["menu", "value"])
    .map(|x| x.as_string());

assert_eq!(menu_value, Some("File"));
```

Use JSON-like literals in Rust to construct [`serde_json`][] `Value`s
or [`rustc-serialize`][] `Json` values.

Because `json_macros` is a compiler plugin, it's only compatible with
the Rust [nightly channel][rust-nightly].

Depending on your project's needs, you may ask `json_macros` to
generate code that constructs [`serde_json`][] values or code that
constructs [`rustc-serialize`][] values.

## Using json_macros with rustc-serialize

By default, `json_macros` generates code for `rustc-serialize`.  In a
future release, the default may switch to `serde_json`, but
`json_macros` should be at least optionally compatible with
`rustc-serialize` for as long as that crate is supported.

To use `json_macros` with `rustc-serialize`, add both packages as
dependencies to your `Cargo.toml`.

```toml
[dependencies]
json_macros = "^0.3"
rustc-serialize = "^0.3"
```

Your crate will also need to link with `rustc_serialize` and `use` it
in any submodule that uses the `json!()` macro.

```rust
extern crate rustc_serialize;

// ...

mod foo {
    use rustc_serialize;
    // ...
}
```

### Example

```rust
#![feature(plugin)]
#![plugin(json_macros)]

extern crate rustc_serialize;

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

## Using json_macros with serde_json

To use `json_macros` with `serde_json`, add both packages as
dependencies to your `Cargo.toml`.  Enable the `with-serde_json`
feature for `json_macros` and disable the default features so as to
not depend on `rustc-serialize`.

```toml
[dependencies]
rustc-serialize = "^0.3"

[dependencies.json_macros]
version = "^0.3"
default-features = false
features = ["with-serde"]
```

Your crate will also need to link with `serde_json` and `use` it in
any submodule that uses the `json!()` macro.

```rust
extern crate serde_json;

// ...

mod foo {
    use serde_json;
    // ...
}
```

### Example

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

[`serde_json`]: <https://github.com/serde-rs/json>
[`rustc-serialize`]: <https://doc.rust-lang.org/rustc-serialize/rustc_serialize/index.html>
[rust-nightly]: <http://doc.rust-lang.org/book/nightly-rust.html>
