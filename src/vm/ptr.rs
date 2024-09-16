extern crate log;
extern crate pest;
use crate::Rule;
use rust_dynamic::value::Value;
use easy_error::{Error};

pub fn process_token(_p: &pest::iterators::Pair<Rule>, t: &String) -> Result<Value, Error> {
    let the_ptr: &str = &t.as_str()[1..];
    Ok(Value::ptr(the_ptr.trim().to_string(), Vec::new()))
}
