use crate::parser::*;
use anyhow::{anyhow, Result};
use horrorshow::{RenderOnce, RenderBox, RenderMut, Render, box_html, html};

fn example() -> Box<dyn Render> {
    box_html! {
        h1 {
            : "Hello ";
            : "There";
        }
    }
}

pub fn render(data: LoadPageChunkData, id: String) -> Result<()> {
    let root = data.record_map.block.get(&id).ok_or(anyhow!("Could not find root block."))?;
    if let Either::Left(RootBlockType::Page { 
    data, 
    format, 
    file_ids
    }) = &root.value {
        let result = example();
        let other = html! {
            div {
                : &result;
            }
        };
        println!("{}", other.to_string());
        Ok(())
    }
    else {
        Err(anyhow!("Root type must be a page."))
    }
}