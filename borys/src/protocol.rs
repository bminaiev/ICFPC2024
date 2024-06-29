use std::fs;

use crate::parser::{encode_string, eval, parse_string};
use anyhow::Result;

pub async fn send_msg(msg: &str) -> Result<()> {
    let token = dotenv::var("TOKEN")?;
    eprintln!("Token: {:?}", token);

    let client = reqwest::Client::new();
    let res = client
        .post("https://boundvariable.space/communicate")
        .body(encode_string(msg))
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
    let evaluated = eval(&parsed);
    eprintln!("Evaluated: {:?}", evaluated);

    Ok(())
}
