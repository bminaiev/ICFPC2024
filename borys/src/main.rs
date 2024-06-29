use std::fs;

mod parser;

use anyhow::Result;

use crate::parser::{encode_string, eval, parse_string};

fn simple_converter() {
    // read from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let parsed = parse_string(&input);
    eprintln!("Parsed:\n{:?}\n\n", parsed);
    let evaluated = eval(&parsed);
    println!("{:?}", evaluated);
}

#[tokio::main]
async fn main() -> Result<()> {
    // if CONV env variable is set, run the converter
    if dotenv::var("CONV").is_ok() {
        simple_converter();
        return Ok(());
    }
    println!("Hello, world!");

    let token = dotenv::var("TOKEN")?;
    eprintln!("Token: {:?}", token);

    let client = reqwest::Client::new();
    let res = client
        .post("https://boundvariable.space/communicate")
        .body(encode_string("get lambdaman21"))
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
    // test();

    Ok(())
}
