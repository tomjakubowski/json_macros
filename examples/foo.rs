#![feature(phase)]
#[phase(plugin)] extern crate json_macros;

extern crate serialize;

pub fn main() {
    let foo = 123u32;
    println!("{}", json!({
        "foo": (foo + 123)
    }).to_pretty_str());
}
