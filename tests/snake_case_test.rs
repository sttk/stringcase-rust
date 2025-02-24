use stringcase::{snake_case, snake_case_with_options, Options};

#[test]
fn it_should_convert_to_snake_case() {
    let converted = snake_case("foo-bar100%-baz-qux");
    assert_eq!(converted, "foo_bar100_baz_qux");
}

#[test]
fn it_should_convert_to_snake_case_with_sep() {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "_",
        keep: "",
    };
    let converted = snake_case_with_options("foo_bar100%BAZQux", &opts);
    assert_eq!(converted, "foo_bar100%_baz_qux");
}

#[test]
fn it_should_convert_to_snake_case_with_keep() {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "%",
    };
    let converted = snake_case_with_options("foo_bar100%BAZQux", &opts);
    assert_eq!(converted, "foo_bar100%_baz_qux");
}

#[test]
fn it_should_convert_to_snake_case_with_nums_as_word() {
    let opts = Options {
        separate_before_non_alphabets: true,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "",
    };
    let converted = snake_case_with_options("foo-bar100%-baz-qux", &opts);
    assert_eq!(converted, "foo_bar_100_baz_qux");
}
