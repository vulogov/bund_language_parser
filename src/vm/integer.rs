extern crate log;
extern crate pest;
use crate::Rule;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn process_token(_p: &pest::iterators::Pair<Rule>, t: &String) -> Result<Value, Error> {
    let num: Result<i64, lexical_core::Error> = lexical_core::parse(t.as_bytes());
    match num {
        Ok(num_value) => {
            return Ok(Value::from_int(num_value));
        }
        Err(err) => {
            bail!("Error converting INT to VALUE: {}", err);
        }
    }
}
