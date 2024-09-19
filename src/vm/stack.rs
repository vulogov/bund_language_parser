extern crate log;
extern crate pest;
use crate::Rule;
use rust_dynamic::value::Value;
use easy_error::{Error};

pub fn process_token(_p: &pest::iterators::Pair<Rule>, t: &String) -> Result<Value, Error> {
    let the_stack: &str = &t.as_str()[1..];
    Ok(Value::named_context(the_stack.trim().to_string()))
}
