const TEST1: &str = r#"
//
//
//
"#;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_language_parser::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_parse_comments() {
        let res = bund_parse(TEST1).expect("Fail to parse BUND comments");
        assert_eq!(res[0].type_name(), "Exit");
    }
    
}
