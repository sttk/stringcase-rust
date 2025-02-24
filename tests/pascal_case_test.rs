use stringcase::{pascal_case, pascal_case_with_options, Options};

#[test]
fn it_should_convert_to_pascal_case() {
    let converted = pascal_case("foo_bar100%BAZQux");
    assert_eq!(converted, "FooBar100BazQux");
}

#[test]
fn it_should_convert_to_pascal_case_with_sep() {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "_",
        keep: "",
    };
    let converted = pascal_case_with_options("foo_bar100%BAZQux", &opts);
    assert_eq!(converted, "FooBar100%BazQux");
}

#[test]
fn it_should_convert_to_pascal_case_with_keep() {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "%",
    };
    let converted = pascal_case_with_options("foo_bar100%BAZQux", &opts);
    assert_eq!(converted, "FooBar100%BazQux");
}
