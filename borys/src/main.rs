use std::fs;

mod parser;
mod protocol;
pub mod spaceship;
pub mod tsp;
mod viz;

pub const TEST_ID: usize = 24;

use anyhow::Result;

use crate::{
    parser::{encode_string, eval, parse_string},
    protocol::send_msg,
    spaceship::spaceship_solve,
};

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
    if dotenv::var("SPACESHIP_DRAW").is_ok() {
        spaceship::spaceship_draw();
        return Ok(());
    }
    println!("Hello, world!");
    viz::viz_main().unwrap();
    // if spaceship_solve().await {
    //     return Ok(());
    // }

    // test();
    // send_msg("get spaceship").await?;

    Ok(())
}
