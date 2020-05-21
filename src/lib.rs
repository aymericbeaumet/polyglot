pub extern crate wasmtime;

#[macro_export]
macro_rules! go {
    ($mod_name:ident $ref:expr) => {
        mod $mod_name {
            pub fn Run(input: &[u8]) -> &[u8] {
                &[]
            }
        }
    };
}

#[macro_export]
macro_rules! wasm {
    ($mod_name:ident $ref:expr) => {
        mod $mod_name {
            use $crate::wasmtime;

            pub fn add(a: i32, b: i32) -> i32 {
                let store = wasmtime::Store::default();

                let module = wasmtime::Module::from_file(&store, $ref).unwrap();

                let instance = wasmtime::Instance::new(&module, &[]).unwrap();

                let answer = instance.get_func("add").unwrap();

                let answer = answer.get2::<i32, i32, i32>().unwrap();

                answer(a, b).unwrap()
            }
        }
    };
}
