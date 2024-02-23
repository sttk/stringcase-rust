use stringcase::{pascal_case, pascal_case_with_sep, pascal_case_with_keep};

#[test]
fn it_should_convert_to_pascal_case() {
    let converted = pascal_case("foo_bar100%BAZQux");
    assert_eq!(converted, "FooBar100BazQux");
}

#[test]
fn it_should_convert_to_pascal_case_with_sep() {
    let converted = pascal_case_with_sep("foo_bar100%BAZQux", "_");
    assert_eq!(converted, "FooBar100%BazQux");
}

#[test]
fn it_should_convert_to_pascal_case_with_keep() {
    let converted = pascal_case_with_keep("foo_bar100%BAZQux", "%");
    assert_eq!(converted, "FooBar100%BazQux");
}
