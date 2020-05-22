use codegen::Scope;
use polyglot::wasmtime::{self, ExternType, ValType};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn generate_mod(
    mod_name: &str,
    relative_path: &str,
    absolute_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let outdir = env::var_os("OUT_DIR").unwrap();
    let outfile = Path::new(&outdir)
        .join("polyglot")
        .join(relative_path)
        .join("mod.rs");

    let wasm_store = wasmtime::Store::default();
    let wasm_module = wasmtime::Module::from_file(&wasm_store, absolute_path).unwrap();

    let mut scope = Scope::new();
    let module = scope.new_module(mod_name);

    for export in wasm_module.exports() {
        if let ExternType::Func(func) = export.ty() {
            let mut param_names = vec![];
            let mut param_types = vec![];
            let mut ret_types = vec![];

            let function_name = export.name();
            let function = module.new_fn(function_name).vis("pub(super)");
            for (c, param) in (b'a'..).zip(func.params().iter()) {
                let param_name = String::from_utf8(vec![c]).unwrap();
                let param_type = type_as_string_from_wasmtime_value_type(param);
                param_types.push(param_type.clone());
                function.arg(&param_name, param_type);
                param_names.push(param_name);
            }
            for (idx, ret) in func.results().iter().enumerate() {
                if idx > 0 {
                    unreachable!("multiple return values not supported");
                }
                let ret_type = type_as_string_from_wasmtime_value_type(ret);
                function.ret(ret_type);
                ret_types.push(ret_type);
            }

            function.line("let store = ::polyglot::wasmtime::Store::default();");

            function.line(format!(
                r#"let module = ::polyglot::wasmtime::Module::from_file(&store, "{}").unwrap();"#,
                absolute_path.to_str().unwrap(),
            ));

            function
                .line("let instance = ::polyglot::wasmtime::Instance::new(&module, &[]).unwrap();");

            function.line(format!(
                r#"let func = instance.get_func("{}").unwrap();"#,
                function_name,
            ));

            let mut line = format!("let func = func.get{}::<", param_types.len());
            for (i, p) in param_types.iter().chain(ret_types.iter()).enumerate() {
                if i > 0 {
                    line.push_str(", ");
                }
                line.push_str(p);
            }
            line.push_str(">().unwrap();");
            function.line(line);

            let mut line = "func(".to_owned();
            for (i, n) in param_names.iter().enumerate() {
                if i > 0 {
                    line.push_str(", ");
                }
                line.push_str(n);
            }
            line.push_str(").unwrap()");
            function.line(line);
        }
    }

    fs::create_dir_all(&Path::parent(&outfile).unwrap())?;
    fs::write(&outfile, scope.to_string())?;

    println!("OUT {:?}", &outfile);

    Ok(())
}

fn type_as_string_from_wasmtime_value_type(v: &ValType) -> &'static str {
    match v {
        ValType::I32 => "i32",
        ValType::I64 => "i64",
        ValType::F32 => "f32",
        ValType::F64 => "f64",
        _ => unimplemented!("todo: {:?}", v),
    }
}
