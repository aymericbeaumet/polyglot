use std::io::prelude::Read;

polyglot::go!(blackfriday "github.com/russross/blackfriday/v2");

fn main() {
    let mut file = std::fs::File::open("sample.md").unwrap();
    let mut input = Vec::new();
    file.read_to_end(&mut input).unwrap();

    let out = blackfriday::Run(&input);

    let s = std::str::from_utf8(&out).unwrap();
    println!("{}", s);
}
