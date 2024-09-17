extern crate log;
extern crate pest;
use crate::Rule;
use crate::parse::parse_pair;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn process_token(p: &pest::iterators::Pair<Rule>, _t: &String, state: &mut Vec<Value>) -> Result<Value, Error> {
    state.push(Value::context());
    for pair in p.clone().into_inner() {
        match parse_pair(pair, state) {
            Ok(value) => {
                state.push(value)
            }
            Err(err) => {
                bail!("{}", err);
            }
        }
    }
    Ok(Value::call("endcontext".to_string(), Vec::new()))
}
