use generator::Converter;

#[test]
fn test() {
    let mut c = Converter::new();
    c.convert(
        r#"
            my_func(3)
        "#,
    );
    assert_eq!(c.get_a(), 4);
}
