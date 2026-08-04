use stringcase::{title_case, title_case_with_options, Options};

#[test]
fn it_should_convert_to_title_case() {
    let converted = title_case("fooBar100%BAZQux");
    assert_eq!(converted, "Foo Bar100 Baz Qux");
}

#[cfg(test)]
mod tests_of_title_case_with_options {
    use super::*;

    #[test]
    fn separate_before_non_alphabets() {
        let opts = Options::new(true, false, "", "%");
        let converted = title_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo Bar 100%baz Qux");
    }

    #[test]
    fn separate_after_non_alphabets() {
        let opts = Options::new(false, true, "", "%");
        let converted = title_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo Bar100% Baz Qux");
    }

    #[test]
    fn separate_before_and_after_non_alphabets() {
        let opts = Options::new(true, true, "", "%");
        let converted = title_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo Bar 100% Baz Qux");
    }

    #[test]
    fn not_separate_before_and_after_non_alphabets() {
        let opts = Options::new(false, false, "", "%");
        let converted = title_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo Bar100%baz Qux");
    }

    #[test]
    fn specify_separators() {
        let opts = Options {
            separators: "-_",
            ..Default::default()
        };
        let converted = title_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo Bar100% Baz Qux");
    }

    #[test]
    fn specify_kept_characters() {
        let opts = Options {
            keep: "_$",
            ..Default::default()
        };
        let converted = title_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo_ Bar100 Baz Qux");
    }
}
