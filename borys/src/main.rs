use std::fs;

mod parser;

use anyhow::Result;

use crate::parser::{encode_string, parse_string};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let token = dotenv::var("TOKEN")?;
    eprintln!("Token: {:?}", token);

    let client = reqwest::Client::new();
    let res = client
        .post("https://boundvariable.space/communicate")
        .body(encode_string("get lambdaman6"))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    eprintln!("Res: {:?}", res);
    let body = res.text().await?;
    eprintln!("Body: {:?}", body);
    // save to file
    fs::write("inputs/last_response.txt", &body)?;
    let parsed = parse_string(&body);
    eprintln!("Parsed: {:?}", parsed);
    // test();

    Ok(())
}
