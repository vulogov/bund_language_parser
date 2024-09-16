const TEST1I: &str = r#"
42
"#;

const TEST2I: &str = r#"
// Three integers
41 42 43
"#;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_language_parser::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_parse_single_int() {
        let res = bund_parse(TEST1I).expect("Fail to parse BUND number");
        assert_eq!(res[0].cast_int().unwrap(), 42 as i64);
    }
    #[test]
    fn test_parse_multiple_int0() {
        let res = bund_parse(TEST2I).expect("Fail to parse BUND number");
        assert_eq!(res.len(), 4);
    }
    #[test]
    fn test_parse_multiple_int1() {
        let res = bund_parse(TEST2I).expect("Fail to parse BUND number");
        assert_eq!(res[1].cast_int().unwrap(), 42 as i64);
    }
}
