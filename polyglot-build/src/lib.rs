use glob::glob;
use regex::Regex;
use std::env;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{fs, fs::File};

#[derive(Default)]
pub struct BuildOptions {}

pub fn build() -> Result<(), Box<dyn Error>> {
    build_with(&env::current_dir()?, &BuildOptions::default())
}

pub fn build_with(root: &Path, _options: &BuildOptions) -> Result<(), Box<dyn Error>> {
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
                generate_mod(
                    &captures[1],
                    &captures[2],
                    &captures[3],
                    &dir.join(&captures[3]),
                )?;
            }
        }
    }

    Ok(())
}

pub fn generate_mod(
    language: &str,
    mod_name: &str,
    relative_path: &str,
    absolute_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let outdir = env::var_os("OUT_DIR").unwrap();
    let outfile = Path::new(&outdir)
        .join("polyglot")
        .join(relative_path)
        .join("mod.rs");

    let contents = format!(
        r#"
mod {} {{
    use ::polyglot::wasmtime;

    pub fn add(a: i32, b: i32) -> i32 {{
        let store = wasmtime::Store::default();

        let module = wasmtime::Module::from_file(&store, "{}").unwrap();

        let instance = wasmtime::Instance::new(&module, &[]).unwrap();

        let answer = instance.get_func("add").unwrap();

        let answer = answer.get2::<i32, i32, i32>().unwrap();

        answer(a, b).unwrap()
    }}
}}
"#,
        mod_name,
        absolute_path.to_str().unwrap(),
    );

    fs::create_dir_all(&Path::parent(&outfile).unwrap())?;
    fs::write(&outfile, contents)?;

    Ok(())
}
