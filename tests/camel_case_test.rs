use stringcase::{camel_case, camel_case_with_options, Options};

#[test]
fn it_should_convert_to_camel_case() {
    let converted = camel_case("foo_bar100%BAZQux");
    assert_eq!(converted, "fooBar100BazQux");
}

#[test]
fn it_should_convert_to_camel_case_with_sep() {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "_",
        keep: "",
    };
    let converted = camel_case_with_options("foo_bar100%BAZQux", &opts);
    assert_eq!(converted, "fooBar100%BazQux");
}

#[test]
fn it_should_convert_to_camel_case_with_keep() {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "%",
    };
    let converted = camel_case_with_options("foo_bar100%BAZQux", &opts);
    assert_eq!(converted, "fooBar100%BazQux");
}
