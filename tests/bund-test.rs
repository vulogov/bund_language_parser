const TEST1: &str = r#"
//
//
//
"#;

const TEST2: &str = r#"
//
// This is BUND Hello World program
//
"Hello world!" println
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

    #[test]
    fn test_parse_hello_world1() {
        let res = bund_parse(TEST2).expect("Fail to parse BUND");
        assert_eq!(res[0].cast_string().unwrap(), "Hello world!");
    }

    #[test]
    fn test_parse_hello_world2() {
        let res = bund_parse(TEST2).expect("Fail to parse BUND");
        assert_eq!(res[1].cast_string().unwrap(), "println");
    }

}
