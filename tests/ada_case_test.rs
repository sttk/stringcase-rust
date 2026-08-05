use stringcase::{ada_case, ada_case_with_options, Options};

#[test]
fn it_should_convert_to_ada_case() {
    let converted = ada_case("fooBar100%BAZQux");
    assert_eq!(converted, "Foo_Bar100_Baz_Qux");
}

#[cfg(test)]
mod tests_of_ada_case_with_options {
    use super::*;

    #[test]
    fn separate_before_non_alphabets() {
        let opts = Options::new(true, false, "", "%");
        let converted = ada_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo_Bar_100%baz_Qux");
    }

    #[test]
    fn separate_after_non_alphabets() {
        let opts = Options::new(false, true, "", "%");
        let converted = ada_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo_Bar100%_Baz_Qux");
    }

    #[test]
    fn separate_before_and_after_non_alphabets() {
        let opts = Options::new(true, true, "", "%");
        let converted = ada_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo_Bar_100%_Baz_Qux");
    }

    #[test]
    fn not_separate_before_and_after_non_alphabets() {
        let opts = Options::new(false, false, "", "%");
        let converted = ada_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo_Bar100%baz_Qux");
    }

    #[test]
    fn specify_separators() {
        let opts = Options {
            separators: "-_",
            ..Default::default()
        };
        let converted = ada_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo_Bar100%_Baz_Qux");
    }

    #[test]
    fn specify_kept_characters() {
        let opts = Options {
            keep: "_$",
            ..Default::default()
        };
        let converted = ada_case_with_options("foo_bar100%BAZQux", &opts);
        assert_eq!(converted, "Foo__Bar100_Baz_Qux");
    }
}
