polyglot::wasm!(lib "lib.wat");

fn main() {
    println!("{}", lib::add(1, 2))
}
