use crate::parser::*;
use anyhow::{anyhow, Result};
use typed_html::{html, dom::DOMTree};

pub fn render(data: LoadPageChunkData, id: String) -> Result<()> {
    let root = data.record_map.block.get(&id).ok_or(anyhow!("Could not find root block."))?;
    if let Either::Left(RootBlockType::Page { 
        data, 
        format, 
        file_ids
        }) = &root.value {
        let hello: DOMTree<String> = html!(
            <h1>"This works!"</h1>
        );
        println!("{}", hello.to_string());
        Ok(())
    }
    else {
        Err(anyhow!("Root type must be a page."))
    }
}