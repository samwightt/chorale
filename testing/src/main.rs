use base::parser::parse;
use base::renderer::Renderer;
use std::fs;
use ui::{Blocks, Inline, Wrapper};
use api::load_page_chunks::load;

#[tokio::main]
async fn main() {
    println!("Starting now!");
    load().await.unwrap();
    // let json = fs::read_to_string("src.json").unwrap();
    // let result = parse(json).unwrap();
    // let renderer = Renderer::new(&result.record_map.block, Blocks {}, Inline {}, Wrapper {});
    // let html = renderer
    //     .render("ddda599f-ff69-4974-9dec-86f6abf3209a")
    //     .to_string();

    // fs::write("output.html", &html).unwrap();
}
