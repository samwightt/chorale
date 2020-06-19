mod parser;
mod renderer;

use parser::parse;
use std::fs;

fn main() {
    let json = fs::read_to_string("src.json").unwrap();
    let result = parse(json).unwrap();
    println!("{:?}", result.record_map.block.keys());
}
