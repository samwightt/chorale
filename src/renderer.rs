use crate::parser::*;
use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use anyhow::{anyhow, Result};

// pub fn render(data: LoadPageChunkData, id: String) -> Result<()> {
//     let root = data.record_map.block.get(&id).ok_or(anyhow!("Could not find root block."))?;
//     if let RootBlockType::Page { 
//         data, 
//         format, 
//         file_ids, 
//         permissions } = &root.value {
//        println!("It's a page!");
//     }
//     Ok(())
// }