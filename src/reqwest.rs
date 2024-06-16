use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub async fn example() -> anyhow::Result<()> {
    let url = "https://www.rust-lang.org";
    let res = reqwest::get(url).await?;

    let status = res.status();
    println!("response: {status:?}");

    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?;
    println!("body: {res:?}");

    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let res = client
        .post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?;
    println!("response body: {res:?}");

    let res = client.get("https://httpbin.org/ip")
        .send()
        .await?;

    // https://dev.to/pintuch/rust-reqwest-examples-10ff
    // deserialize json to struct using serde
    let ip = res.json::<Ip>().await?;
    println!("ip: {ip:?}");

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Ip {
    origin: String
}
