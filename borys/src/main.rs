use std::fs;

pub mod array_2d;
mod lambdaman;
pub mod local_solver;
mod parser;
mod protocol;
pub mod simulated_annealing;
pub mod spaceship;
pub mod tsp;
mod viz;
mod viz_lambda;
pub mod zoomer;

pub const TEST_ID: usize = 11;

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
    viz_lambda::viz_lambda_main().unwrap();
    // if spaceship_solve().await {
    //     return Ok(());
    // }

    // test();
    // send_msg("get spaceship").await?;

    // lambdaman::lambda_solver();
    Ok(())
}
