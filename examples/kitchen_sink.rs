#![feature(plugin)]
#[plugin] extern crate json_macros;

extern crate "rustc-serialize" as rustc_serialize;

pub fn main() {
    let _x = json!();
    println!("{}", _x.pretty().to_string());
}
