const TEST1S: &str = r#"
"42.0"
"#;

const TEST2S: &str = r#"
"The answer on life universe and everything is 42"
"#;

const TEST3S: &str = r#"
'42'
"#;

const TEST4S: &str = r#"
42 '42' 42.0
"#;

const TEST5S: &str = r#"
:HELLO
"#;

const TEST6S: &str = r#"
:HELLO :WORLD "Oh, dear!"
"#;

const TEST7S: &str = r#"
:ПриветМир
"#;

const TEST8S: &str = r#"
"Отведай ещё этих французских булок"
"#;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_language_parser::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_parse_single_string() {
        let res = bund_parse(TEST1S).expect("Fail to parse BUND string");
        assert_eq!(res[0].cast_string().unwrap(), "42.0");
    }

    #[test]
    fn test_parse_single_string_with_spaces() {
        let res = bund_parse(TEST2S).expect("Fail to parse BUND string");
        assert_eq!(res[0].cast_string().unwrap(), "The answer on life universe and everything is 42");
    }

    #[test]
    fn test_parse_single_literal() {
        let res = bund_parse(TEST3S).expect("Fail to parse BUND string");
        assert_eq!(res[0].cast_string().unwrap(), "42");
    }

    #[test]
    fn test_parse_multiple_literals() {
        let res = bund_parse(TEST4S).expect("Fail to parse BUND string");
        assert_eq!(res[1].cast_string().unwrap(), "42");
    }

    #[test]
    fn test_parse_single_atom1() {
        let res = bund_parse(TEST5S).expect("Fail to parse BUND string");
        assert_eq!(res[0].cast_string().unwrap(), "HELLO");
    }

    #[test]
    fn test_parse_single_atom2() {
        let res = bund_parse(TEST7S).expect("Fail to parse BUND string");
        assert_eq!(res[0].cast_string().unwrap(), "ПриветМир");
    }

    #[test]
    fn test_parse_second_atom() {
        let res = bund_parse(TEST6S).expect("Fail to parse BUND string");
        assert_eq!(res[1].cast_string().unwrap(), "WORLD");
    }

    #[test]
    fn test_parse_unicode_string() {
        let res = bund_parse(TEST8S).expect("Fail to parse BUND string");
        assert_eq!(res[0].cast_string().unwrap(), "Отведай ещё этих французских булок");
    }
}
