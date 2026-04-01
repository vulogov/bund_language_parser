extern crate pest;
use crate::bund_parse;
use easy_error::{bail, Error};
use rust_dynamic::value::Value;

fn bund_vec_to_list(state: &mut Vec<Value>) -> Result<Value, Error> {
    let mut res = Value::list();
    for e in state.iter() {
        res = res.push(e.clone());
    }
    Ok(res)
}

pub fn compile<N: AsRef<str> + ToString>(source: N) -> Result<Value, Error> {
    match bund_parse(&source.to_string()) {
        Ok(mut tokens) => bund_vec_to_list(&mut tokens),
        Err(err) => {
            bail!("Error compiling BUND code: {}", err)
        }
    }
}

pub fn compile_to_binary<N: AsRef<str> + ToString>(source: N) -> Result<Vec<u8>, Error> {
    match compile(source) {
        Ok(the_list) => the_list.to_binary(),
        Err(err) => {
            bail!("{}", err)
        }
    }
}
