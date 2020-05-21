#[macro_export]
macro_rules! go {
    ($a:ident $e:expr) => {
        mod $a {
            pub fn Run(input: &[u8]) -> &[u8] {
                &[]
            }
        }
    };
}

#[macro_export]
macro_rules! wasm {
    ($a:ident $e:expr) => {
        mod $a {
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        }
    };
}
