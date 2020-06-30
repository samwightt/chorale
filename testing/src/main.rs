use base::parser::parse;
use base::renderer::Renderer;
use std::fs;
use ui::{Blocks, Inline, Wrapper};
use api::load_page_chunks::load;

#[tokio::main]
async fn main() {
    println!("Starting now!");
    let result = load("ddda599f-ff69-4974-9dec-86f6abf3209a").await.unwrap();
    let renderer = Renderer::new(&result.record_map.block, Blocks {}, Inline {}, Wrapper {});
    println!("Starting rendering... now!!");
    let html = renderer
        .render("ddda599f-ff69-4974-9dec-86f6abf3209a")
        .to_string();

    fs::write("output.html", &html).unwrap();
}
