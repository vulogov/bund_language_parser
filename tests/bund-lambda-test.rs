const TEST1L: &str = r#"
{ 42 }
"#;

const TEST2L: &str = r#"
{ 42 :42 42.0 }
"#;

const TEST3L: &str = r#"
{ dup }
"#;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_language_parser::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_parse_single_lambda1() {
        let res = bund_parse(TEST1L).expect("Fail to parse BUND lambda");
        assert_eq!(res[0].type_name(), "Lambda");
    }

    #[test]
    fn test_parse_single_lambda2() {
        let res = bund_parse(TEST1L).expect("Fail to parse BUND lambda");
        let data = res[0].cast_lambda().unwrap();
        assert_eq!(data[0].cast_int().unwrap(), 42 as i64);
    }

    #[test]
    fn test_parse_multi_lambda1() {
        let res = bund_parse(TEST2L).expect("Fail to parse BUND lambda");
        let data = res[0].cast_lambda().unwrap();
        assert_eq!(data[1].cast_string().unwrap(), "42");
    }

    #[test]
    fn test_parse_single_lambda3() {
        let res = bund_parse(TEST3L).expect("Fail to parse BUND lambda");
        let data = res[0].cast_lambda().unwrap();
        assert_eq!(data[0].cast_string().unwrap(), "dup");
    }

    #[test]
    fn test_parse_single_lambda4() {
        let res = bund_parse(TEST3L).expect("Fail to parse BUND lambda");
        let data = res[0].cast_lambda().unwrap();
        assert_eq!(data[0].type_name(), "Call");
    }
}
