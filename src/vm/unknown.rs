extern crate log;
extern crate pest;
use crate::Rule;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn process_token(p: &pest::iterators::Pair<Rule>, _t: &String) -> Result<Value, Error> {
    bail!("Received unknown token of type: {:#?}", p.as_rule());
}
