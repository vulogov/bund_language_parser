const TEST1N: &str = r#"
HELLO
"#;

const TEST2N: &str = r#"
1 HELLO "WORLD"
"#;

const TEST3N: &str = r#"
+HELLO
"#;

const TEST4N: &str = r#"
= 5 6
"#;

const TEST5N: &str = r#"
2 >>== 4
"#;

const TEST6N: &str = r#"
"Hello" -WORLD 42.0
"#;

const TEST7N: &str = r#"
"Hello" `-WORLD 42.0
"#;

const TEST8N: &str = r#"
"Hello" : HELLO ;  42.0
"#;

const TEST9N: &str = r#"
λ 1 2 3
"#;

const TEST10N: &str = r#"
HELLO_WORLD
"#;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_language_parser::*;
    use rust_dynamic::value::Value;

    #[test]
    fn test_parse_single_name() {
        let res = bund_parse(TEST1N).expect("Fail to parse BUND name");
        assert_eq!(res[0].cast_string().unwrap(), "HELLO");
    }

    #[test]
    fn test_parse_multiple_with_name() {
        let res = bund_parse(TEST2N).expect("Fail to parse BUND name");
        assert_eq!(res[1].cast_string().unwrap(), "HELLO");
    }

    #[test]
    fn test_parse_name_1() {
        let res = bund_parse(TEST3N).expect("Fail to parse BUND name");
        assert_eq!(res[0].cast_string().unwrap(), "+HELLO");
    }

    #[test]
    fn test_parse_name_2() {
        let res = bund_parse(TEST4N).expect("Fail to parse BUND name");
        assert_eq!(res[0].cast_string().unwrap(), "=");
    }

    #[test]
    fn test_parse_name_3() {
        let res = bund_parse(TEST5N).expect("Fail to parse BUND name");
        assert_eq!(res[1].cast_string().unwrap(), ">>==");
    }

    #[test]
    fn test_parse_name_4() {
        let res = bund_parse(TEST6N).expect("Fail to parse BUND name");
        assert_eq!(res[1].cast_string().unwrap(), "-WORLD");
    }

    #[test]
    fn test_parse_ptr_1() {
        let res = bund_parse(TEST7N).expect("Fail to parse BUND name");
        assert_eq!(res[1].cast_string().unwrap(), "-WORLD");
    }

    #[test]
    fn test_parse_command_1() {
        let res = bund_parse(TEST8N).expect("Fail to parse BUND name");
        assert_eq!(res[1].cast_string().unwrap(), ":");
    }

    #[test]
    fn test_parse_command_2() {
        let res = bund_parse(TEST8N).expect("Fail to parse BUND name");
        assert_eq!(res[3].cast_string().unwrap(), ";");
    }

    #[test]
    fn test_parse_name_symbol() {
        let res = bund_parse(TEST9N).expect("Fail to parse BUND name");
        assert_eq!(res[0].cast_string().unwrap(), "λ");
    }

    #[test]
    fn test_parse_name_with_undescore_symbol() {
        let res = bund_parse(TEST10N).expect("Fail to parse BUND name");
        assert_eq!(res[0].cast_string().unwrap(), "HELLO_WORLD");
    }

}
