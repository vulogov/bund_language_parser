const TEST1F: &str = r#"
42.0
"#;

const TEST2F: &str = r#"
// Three integers
41 42.0 43
"#;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_language_parser::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_parse_single_float() {
        let res = bund_parse(TEST1F).expect("Fail to parse BUND number");
        assert_eq!(res[0].cast_float().unwrap(), 42 as f64);
    }
    #[test]
    fn test_parse_multiple_float0() {
        let res = bund_parse(TEST2F).expect("Fail to parse BUND number");
        assert_eq!(res.len(), 4);
    }
    #[test]
    fn test_parse_multiple_float1() {
        let res = bund_parse(TEST2F).expect("Fail to parse BUND number");
        assert_eq!(res[1].cast_float().unwrap(), 42 as f64);
    }
}
