const TEST1LS: &str = r#"
[ 42 ]
"#;

const TEST2LS: &str = r#"
[ 42 :42 42.0 ]
"#;

const TEST3LS: &str = r#"
[ dup ]
"#;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_language_parser::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_parse_single_list1() {
        let res = bund_parse(TEST1LS).expect("Fail to parse BUND list");
        assert_eq!(res[0].type_name(), "List");
    }

    #[test]
    fn test_parse_single_list2() {
        let res = bund_parse(TEST1LS).expect("Fail to parse BUND list");
        let data = res[0].cast_list().unwrap();
        assert_eq!(data[0].cast_int().unwrap(), 42 as i64);
    }

    #[test]
    fn test_parse_multi_list1() {
        let res = bund_parse(TEST2LS).expect("Fail to parse BUND list");
        let data = res[0].cast_list().unwrap();
        assert_eq!(data[1].cast_string().unwrap(), "42");
    }

    #[test]
    fn test_parse_single_list3() {
        let res = bund_parse(TEST3LS).expect("Fail to parse BUND  list");
        let data = res[0].cast_list().unwrap();
        assert_eq!(data[0].cast_string().unwrap(), "dup");
    }

    #[test]
    fn test_parse_single_list4() {
        let res = bund_parse(TEST3LS).expect("Fail to parse BUND list");
        let data = res[0].cast_list().unwrap();
        assert_eq!(data[0].type_name(), "Call");
    }
}
