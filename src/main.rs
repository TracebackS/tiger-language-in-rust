extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::error::Error;
use std::io::{self, Read};
use std::fs::File;

#[derive(Parser)]
#[grammar = "../pest-src/csv.pest"]
pub struct CSVParser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let mut fd = File::open("prog.src")?;
    fd.read_to_string(&mut buffer)?;
    let parse_result = CSVParser::parse(Rule::field, &buffer[..]);
    println!("{:?}", parse_result);
    Ok(())
}
