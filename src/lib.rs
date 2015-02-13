#![feature(plugin_registrar, quote)]
#![feature(rustc_private)]

extern crate rustc;
extern crate syntax;
extern crate "rustc-serialize" as rustc_serialize;

use rustc::plugin::Registry;

mod plugin;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("json", plugin::expand);
}
