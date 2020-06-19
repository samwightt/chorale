mod parser;
mod renderer;
mod partials;

use parser::parse;
use std::fs;

fn main() {
    let json = fs::read_to_string("src.json").unwrap();
    let result = parse(json).unwrap();
    renderer::render(result, String::from("ef28925f-6389-4c1d-962d-a11c86879897"));
}
