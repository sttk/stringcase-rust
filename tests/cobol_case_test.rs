use stringcase::{cobol_case, cobol_case_with_options, Options};

#[test]
fn it_should_convert_to_cobol_case() {
    let converted = cobol_case("foo_bar100%BAZQux");
    assert_eq!(converted, "FOO-BAR100-BAZ-QUX");
}

#[test]
fn it_should_convert_to_cobol_case_with_sep() {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "_",
        keep: "",
    };
    let converted = cobol_case_with_options("foo_bar100%BAZQux", &opts);
    assert_eq!(converted, "FOO-BAR100%-BAZ-QUX");
}

#[test]
fn it_should_convert_to_cobol_case_with_keep() {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "%",
    };
    let converted = cobol_case_with_options("foo_bar100%BAZQux", &opts);
    assert_eq!(converted, "FOO-BAR100%-BAZ-QUX");
}

#[test]
fn it_should_convert_to_cobol_case_with_nums_as_word() {
    let opts = Options {
        separate_before_non_alphabets: true,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "",
    };
    let converted = cobol_case_with_options("foo_bar100%BAZQux", &opts);
    assert_eq!(converted, "FOO-BAR-100-BAZ-QUX");
}
