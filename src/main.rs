extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[grammar = "./pest-src/tiger.pest"]
pub struct TigerParser;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut buffer = String::new();
    File::open(&args[1])?.read_to_string(&mut buffer)?;
    let parse_result = TigerParser::parse(Rule::main, &buffer[..]);
    println!("{:#?}", parse_result);
    Ok(())
}
