const TEST1C: &str = r#"
( 42 )
"#;

const TEST2C: &str = r#"
@ИмяСтека
"#;


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_language_parser::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_parse_single_ctx1() {
        let res = bund_parse(TEST1C).expect("Fail to parse BUND ctx");
        assert_eq!(res[0].type_name(), "Context");
    }

    #[test]
    fn test_parse_single_ctx2() {
        let res = bund_parse(TEST1C).expect("Fail to parse BUND ctx");
        assert_eq!(res[1].cast_int().unwrap(), 42 as i64);
    }

    #[test]
    fn test_parse_single_stack1() {
        let res = bund_parse(TEST2C).expect("Fail to parse BUND ctx");
        assert_eq!(res[0].type_name(), "Context");
    }

    #[test]
    fn test_parse_single_stack2() {
        let res = bund_parse(TEST2C).expect("Fail to parse BUND ctx");
        assert_eq!(res[0].cast_string().unwrap(), "ИмяСтека");
    }

}
