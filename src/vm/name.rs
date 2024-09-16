extern crate log;
extern crate pest;
use crate::Rule;
use rust_dynamic::value::Value;
use easy_error::{Error};

pub fn process_token(_p: &pest::iterators::Pair<Rule>, t: &String) -> Result<Value, Error> {
    Ok(Value::call(t.clone(), Vec::new()))
}
