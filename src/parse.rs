extern crate pest;
use crate::Rule;
use crate::vm::*;
use rust_dynamic::value::Value;
use easy_error::{Error};

pub fn parse_pair(p: pest::iterators::Pair<Rule>, state: &mut Vec<Value>) -> Result<Value, Error> {
    let rule  = &p.as_rule();
    let token = &p.as_span();
    match rule {
        Rule::integer => {
            return integer::process_token(&p, &token.as_str().to_string());
        }
        Rule::float => {
            return float::process_token(&p, &token.as_str().to_string());
        }
        Rule::string => {
            return string::process_token(&p, &token.as_str().to_string());
        }
        Rule::literal => {
            return literal::process_token(&p, &token.as_str().to_string());
        }
        Rule::atom => {
            return atom::process_token(&p, &token.as_str().to_string());
        }
        Rule::name => {
            return name::process_token(&p, &token.as_str().to_string());
        }
        Rule::ptr => {
            return ptr::process_token(&p, &token.as_str().to_string());
        }
        Rule::command => {
            return command::process_token(&p, &token.as_str().to_string());
        }
        Rule::lambda => {
            return lambda::process_token(&p, &token.as_str().to_string(), state);
        }
        Rule::list => {
            return list::process_token(&p, &token.as_str().to_string(), state);
        }
        Rule::ctx => {
            return ctx::process_token(&p, &token.as_str().to_string(), state);
        }
        Rule::stack => {
            return stack::process_token(&p, &token.as_str().to_string());
        }
        Rule::EOI => {
            return eoi::process_token(&p, &token.as_str().to_string());
        }
        _ => {
            return unknown::process_token(&p, &token.as_str().to_string());
        }
    }
}
