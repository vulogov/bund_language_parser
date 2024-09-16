extern crate pest;
use pest::{Parser};
use pest_derive::*;
use easy_error::{Error, bail};
use rust_dynamic::value::Value;

pub mod parse;
pub mod vm;

#[derive(Parser)]
#[grammar = "bund.pest"]
pub struct BundParser;

pub fn bund_parse(source: &str) -> Result<Vec<Value>, Error> {
    let mut res: Vec<Value> = Vec::new();
    let pairs = BundParser::parse(Rule::program, source);
    match pairs {
        Ok(ast_pairs) => {
            for pair in ast_pairs {
                match parse::parse_pair(pair) {
                    Ok(value) => {
                        res.push(value);
                    }
                    Err(err) => {
                        bail!("Error parsing token: {}", err);
                    }
                }
            }

        }
        Err(err) => {
            bail!("{}", err);
        }
    }
    Ok(res)
}
