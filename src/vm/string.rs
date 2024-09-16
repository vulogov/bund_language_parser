extern crate log;
extern crate pest;
use crate::Rule;
use rust_dynamic::value::Value;
use easy_error::{Error};

pub fn process_token(_p: &pest::iterators::Pair<Rule>, t: &String) -> Result<Value, Error> {    
    let the_str: &str = &t.as_str()[1..t.len() - 1];
    Ok(Value::from_string(the_str))
}
