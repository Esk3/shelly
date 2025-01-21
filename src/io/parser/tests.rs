use super::*;

#[test]
fn escaper_test() {
    let abc = Escaper::new(b"abc").next().unwrap();
    assert_eq!(abc, b"abc");

    let single = Escaper::new(b"hell'world what is good'").next().unwrap();
    assert_eq!(single, b"hellworld what is good");

    let single = Escaper::new(br#"hell"world what is good""#).next().unwrap();
    assert_eq!(single, b"hellworld what is good");
}

#[test]
fn handles_multiple_whitespace() {
    let arg = b"hello    world";
    let result = Escaper::new(arg).collect::<Vec<_>>();
    assert_eq!(result, [b"hello", b"world"]);
}

#[test]
#[ignore = "todo"]
fn backslash_escaped_quote_does_not_end_string() {
    let arg = br#"echo "hello wo \"ld"#;
    let result = Escaper::new(arg)
        .map(|s| String::from_utf8(s).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(result, ["echo", r#"hello wo \"ld"#]);
}
