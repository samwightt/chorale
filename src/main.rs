mod parser;
mod renderer;

use parser::parse;
use std::fs;
use serde_json::Value;

fn main() {
    let json = fs::read_to_string("src.json").unwrap();
    let result = parse(json).unwrap();

    let page = &result.record_map.block;
    // match &page.value {
    //     Value::Null => println!("Null!"),
    //     Value::Bool(b) => println!("Bool: {}", b),
    //     Value::Number(n) => println!("Number: {}", n),
    //     Value::String(s) => println!("String: {}", s),
    //     Value::Array(a) => println!("Array: {:?}", a),
    //     Value::Object(m) => println!("Map: {:?}", m)
    // }

    println!("{:?}", result.record_map.block.keys());
}
