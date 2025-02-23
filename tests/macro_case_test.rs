use stringcase::{macro_case, macro_case_with_options, Options};

#[test]
fn it_should_convert_to_macro_case() {
    let converted = macro_case("foo-bar100%-baz-qux");
    assert_eq!(converted, "FOO_BAR100_BAZ_QUX");
}

#[test]
fn it_should_convert_to_macro_case_with_sep() {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "_",
        keep: "",
    };
    let converted = macro_case_with_options("foo_bar100%BAZQux", &opts);
    assert_eq!(converted, "FOO_BAR100%_BAZ_QUX");
}

#[test]
fn it_should_convert_to_macro_case_with_keep() {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "%",
    };

    let converted = macro_case_with_options("foo_bar100%BAZQux", &opts);
    assert_eq!(converted, "FOO_BAR100%_BAZ_QUX");
}

#[test]
fn it_should_convert_to_macro_case_with_nums_as_word() {
    let opts = Options {
        separate_before_non_alphabets: true,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "",
    };

    let converted = macro_case_with_options("foo-bar100%-baz-qux", &opts);
    assert_eq!(converted, "FOO_BAR_100_BAZ_QUX");
}
