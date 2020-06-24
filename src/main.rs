mod parser;
mod renderer;
mod partials;

use parser::parse;
use std::fs;

fn main() {
    let json = fs::read_to_string("src.json").unwrap();
    let result = parse(json).unwrap();
    let rendered = renderer::render(&String::from("ddda599f-ff69-4974-9dec-86f6abf3209a"), &result.record_map.block).unwrap();
    println!("{:?}", result.record_map.block.keys());
    println!("{}", rendered.into_string());
}
