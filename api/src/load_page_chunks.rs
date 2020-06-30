use anyhow::Result;
use base::parser::{LoadPageChunkData, parse};

pub async fn load(id: &str) -> Result<LoadPageChunkData> {
    let client = reqwest::Client::new();
    let body = r#"{"pageId":""#.to_string() + id + r#"","limit":100000,"cursor":{"stack":[[{"table":"block","id":""# + id + r#"","index":0}]]},"chunkNumber":0,"verticalColumns":false}"#;
    let res = client.post("https://www.notion.so/api/v3/loadPageChunk")
        .body(body)
        .header("content-type", "application/json")
        .send().await?
        .text().await?;

    Ok(parse(res)?)
}