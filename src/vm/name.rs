extern crate log;
extern crate pest;
use crate::Rule;
use rust_dynamic::value::Value;
use easy_error::{Error};

pub fn process_token(_p: &pest::iterators::Pair<Rule>, t: &String) -> Result<Value, Error> {
    let the_name: &str = &t.as_str();
    Ok(Value::call(the_name.trim().to_string(), Vec::new()))
}
