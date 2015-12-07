#![feature(plugin_registrar, quote)]
#![feature(rustc_private)]

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;
extern crate rustc_serialize;

use rustc_plugin::Registry;

mod plugin;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("json", plugin::expand);
}
