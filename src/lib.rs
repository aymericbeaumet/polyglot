#[macro_export]
macro_rules! go_import {
    ($e:expr) => {
        mod blackfriday {
            pub fn Run(input: &[u8]) -> &[u8] {
                &[]
            }
        }
    };

    ($a:ident $e:expr) => {
        mod $a {
            pub fn Run(input: &[u8]) -> &[u8] {
                &[]
            }
        }
    };
}
