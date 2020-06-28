use anyhow::Result;

pub async fn load() -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.post("https://www.notion.so/api/v3/loadPageChunk")
        .body(r#"{"pageId":"ddda599f-ff69-4974-9dec-86f6abf3209a","limit":50,"cursor":{"stack":[[{"table":"block","id":"ddda599f-ff69-4974-9dec-86f6abf3209a","index":0}]]},"chunkNumber":0,"verticalColumns":false}"#)
        .header("content-type", "application/json")
        .send().await?;

    println!("{:?}", res.text().await?);
    Ok(())
}