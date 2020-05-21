pub extern crate wasmtime;

#[macro_export]
macro_rules! wasm {
    ($mod_name:ident $ref:expr) => {
        include!(concat!(env!("OUT_DIR"), "/polyglot/", $ref, "/mod.rs"));
    };
}
