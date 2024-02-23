use stringcase::{
    camel_case, cobol_case, kebab_case, macro_case, pascal_case, snake_case, train_case,
};

#[test]
fn it_should_convert_to_camel_case() {
    let converted = camel_case("foo.bar.baz");
    assert_eq!(converted, "fooBarBaz");
}

#[test]
fn it_should_convert_to_cobol_case() {
    let converted = cobol_case("foo.bar.baz");
    assert_eq!(converted, "FOO-BAR-BAZ");
}

#[test]
fn it_should_convert_to_kebab_case() {
    let converted = kebab_case("fooBarBaz");
    assert_eq!(converted, "foo-bar-baz");
}

#[test]
fn it_should_convert_to_macro_case() {
    let converted = macro_case("fooBarBaz");
    assert_eq!(converted, "FOO_BAR_BAZ");
}

#[test]
fn it_should_convert_to_pascal_case() {
    let converted = pascal_case("foo-bar-baz");
    assert_eq!(converted, "FooBarBaz");
}

#[test]
fn it_should_convert_to_snake_case() {
    let converted = snake_case("fooBarBaz");
    assert_eq!(converted, "foo_bar_baz");
}

#[test]
fn it_should_convert_to_train_case() {
    let converted = train_case("fooBarBaz");
    assert_eq!(converted, "Foo-Bar-Baz");
}
