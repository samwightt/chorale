mod parser;
mod renderer;
mod partials;

use parser::parse;
use std::fs;

fn main() {
    let json = fs::read_to_string("src.json").unwrap();
    let result = parse(json).unwrap();
    let rendered = renderer::render(&String::from("ef28925f-6389-4c1d-962d-a11c86879897"), &result.record_map.block).unwrap();
    println!("{:?}", result.record_map.block.keys());
    println!("{}", rendered.into_string());
}
