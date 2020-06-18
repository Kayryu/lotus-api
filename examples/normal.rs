// jsonrpc
use serde::{Deserialize, Serialize};

use serde_json::Value;
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    jsonrpc: String,
    id: i32,
    method: String,
    params: Value,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let new_post = Post {
        id: 1,
        jsonrpc: "2.0".into(),
        method: "Filecoin.Version".into(),
        params: Value::Null,
    };

    let res = reqwest::Client::new()
        .post("http://127.0.0.1:1234/rpc/v0")
        .json(&new_post)
        .send()
        .await?;

    println!("{:#?}", res.text().await?);
    Ok(())
}
