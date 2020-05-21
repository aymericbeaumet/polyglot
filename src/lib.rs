pub extern crate wasmtime;

use glob::glob;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

#[derive(Default)]
pub struct BuildOptions {}

pub fn build() -> Result<(), Box<dyn Error>> {
    build_with(&std::env::current_dir()?, &BuildOptions::default())
}

pub fn build_with(root: &std::path::Path, _options: &BuildOptions) -> Result<(), Box<dyn Error>> {
    // TODO: find a more robust approach
    let polyglot_re = Regex::new(r#"^polyglot::([^!]+)!\(([^ ]+) "([^)]+)"\);$"#).unwrap();

    let pattern = format!("{}/**/*.rs", root.to_str().unwrap());

    for filepath in glob(&pattern).unwrap() {
        let filepath = filepath?;

        // TODO: leverage .gitignore for this
        let entry_str = filepath.to_str().unwrap();
        if entry_str.ends_with("/build.rs") || entry_str.contains("/target/") {
            continue;
        }

        let handle = File::open(&filepath)?;
        let lines = BufReader::new(handle).lines();
        for line in lines {
            let dir = Path::parent(&filepath).unwrap();

            if let Some(captures) = polyglot_re.captures(&line?) {
                let language = &captures[1];
                let mod_name = &captures[2];
                let relative_path = &captures[3];
                let absolute_path = dir.join(relative_path);
                println!(
                    "{} {} {} {:?}",
                    language, mod_name, relative_path, absolute_path
                );
            }
        }
    }

    Ok(())
}
