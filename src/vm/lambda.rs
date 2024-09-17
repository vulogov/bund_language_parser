extern crate log;
extern crate pest;
use crate::Rule;
use crate::parse::parse_pair;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn process_token(p: &pest::iterators::Pair<Rule>, _t: &String, state: &mut Vec<Value>) -> Result<Value, Error> {
    let mut res: Vec<Value> = Vec::new();
    for pair in p.clone().into_inner() {
        match parse_pair(pair, state) {
            Ok(value) => {
                res.push(value)
            }
            Err(err) => {
                bail!("{}", err);
            }
        }
    }
    Ok(Value::to_lambda(res))
}
