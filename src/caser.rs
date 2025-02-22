// Copyright (C) 2024-2025 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::camel_case::*;
use crate::cobol_case::*;
use crate::kebab_case::*;
use crate::macro_case::*;
use crate::options::Options;
use crate::pascal_case::*;
use crate::snake_case::*;
use crate::train_case::*;

/// `Caser` is the trait to attach methods for converting strings `&str` and
/// `String` to various cases.
///
/// By declarating this trait with `use` keyword, all conversion methods
/// provided by this library become available for `&str` and `String`.
pub trait Caser<T: AsRef<str>> {
    // camel case

    /// Converts the input string to camel case.
    ///
    /// It treats the end of a sequence of non-alphabetical characters as a word boundary,
    /// but not the beginning.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let camel = "foo_bar_baz".to_camel_case();
    ///     assert_eq!(camel, "fooBarBaz");
    /// ```
    fn to_camel_case(&self) -> String;

    /// Converts the input string to camel case with the specified options.
    ///
    /// ```rust
    ///     let opts = stringcase::Options{
    ///       separate_before_non_alphabets: true,
    ///       separate_after_non_alphabets: true,
    ///       separators: "",
    ///       keep: "",
    ///     };
    ///     let camel = stringcase::camel_case_with_options("foo_bar_100_baz", &opts);
    ///     assert_eq!(camel, "fooBar100Baz");
    /// ```
    fn to_camel_case_with_options(&self, opts: &Options) -> String;

    /// Converts the input string to camel case with the specified separator characters.
    #[deprecated(
        since = "0.4.0",
        note = "Should use to_camel_case_with_options instead"
    )]
    fn to_camel_case_with_sep(&self, seps: &str) -> String;

    /// Converts the input string to camel case with the specified characters to be kept.
    #[deprecated(
        since = "0.4.0",
        note = "Should use to_camel_case_with_options instead"
    )]
    fn to_camel_case_with_keep(&self, keeped: &str) -> String;

    // cobol case

    /// Converts the input string to cobol case.
    ///
    /// It treats the end of a sequence of non-alphabetical characters as a word boundary,
    /// but not the beginning.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let cobol = "foo_bar_baz".to_cobol_case();
    ///     assert_eq!(cobol, "FOO-BAR-BAZ");
    /// ```
    fn to_cobol_case(&self) -> String;

    /// Converts the input string to cobol case with the specified options.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let opts = stringcase::Options{
    ///       separate_before_non_alphabets: true,
    ///       separate_after_non_alphabets: true,
    ///       separators: "",
    ///       keep: "",
    ///     };
    ///     let cobol = "foo_bar_100_baz".to_cobol_case_with_options(&opts);
    ///     assert_eq!(cobol, "FOO-BAR-100-BAZ");
    /// ```
    fn to_cobol_case_with_options(&self, opts: &Options) -> String;

    /// Converts the input string to cobol case.
    ///
    /// It treats the begin and the end of a sequence of non-alphabetical characters as a word
    /// boundary.
    #[deprecated(
        since = "0.4.0",
        note = "Should use to_cobol_case_with_options instead"
    )]
    fn to_cobol_case_with_nums_as_word(&self) -> String;

    /// Converts the input string to cobol case with the specified separator characters.
    #[deprecated(
        since = "0.4.0",
        note = "Should use to_cobol_case_with_options instead"
    )]
    fn to_cobol_case_with_sep(&self, seps: &str) -> String;

    /// Converts the input string to cobol case with the specified characters to be kept.
    #[deprecated(
        since = "0.4.0",
        note = "Should use to_cobol_case_with_options instead"
    )]
    fn to_cobol_case_with_keep(&self, keeped: &str) -> String;

    // kebab case

    /// Converts the input string to kebab case.
    ///
    /// It treats the end of a sequence of non-alphabetical characters as a word boundary,
    /// but not the beginning.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let kebab = "fooBarBaz".to_kebab_case();
    ///     assert_eq!(kebab, "foo-bar-baz");
    /// ```
    fn to_kebab_case(&self) -> String;

    /// Converts the input string to kebab case with the specified options.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let opts = stringcase::Options{
    ///       separate_before_non_alphabets: true,
    ///       separate_after_non_alphabets: true,
    ///       separators: "",
    ///       keep: "",
    ///     };
    ///     let kebab = "foo_bar_100_baz".to_kebab_case_with_options(&opts);
    ///     assert_eq!(kebab, "foo-bar-100-baz");
    /// ```
    fn to_kebab_case_with_options(&self, opts: &Options) -> String;

    /// Converts the input string to kebab case.
    ///
    /// It treats the begin and the end of a sequence of non-alphabetical characters as a word
    /// boundary.
    #[deprecated(
        since = "0.4.0",
        note = "Should use to_kebab_case_with_options instead"
    )]
    fn to_kebab_case_with_nums_as_word(&self) -> String;

    /// Converts the input string to kebab case with the specified separator characters.
    #[deprecated(
        since = "0.4.0",
        note = "Should use to_kebab_case_with_options instead"
    )]
    fn to_kebab_case_with_sep(&self, seps: &str) -> String;

    /// Converts the input string to kebab case with the specified characters to be kept.
    #[deprecated(
        since = "0.4.0",
        note = "Should use to_kebab_case_with_options instead"
    )]
    fn to_kebab_case_with_keep(&self, keeped: &str) -> String;

    // macro case

    /// Converts a string to macro case.
    ///
    /// This method targets the upper and lower cases of ASCII alphabets for
    /// capitalization, and all characters except ASCII alphabets and ASCII numbers
    /// are replaced to underscores as word separators.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let macro_ = "foo-bar100%baz".to_macro_case();
    ///     assert_eq!(macro_, "FOO_BAR100_BAZ");
    /// ```
    fn to_macro_case(&self) -> String;

    /// Converts a string to macro case.
    ///
    /// This method targets the upper and lower cases of ASCII alphabets and
    /// ASCII numbers for capitalization, and all characters except ASCII
    /// alphabets and ASCII numbers are replaced to underscores as word separators.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let macro_ = "foo-bar100%baz".to_macro_case_with_nums_as_word();
    ///     assert_eq!(macro_, "FOO_BAR_100_BAZ");
    /// ```
    fn to_macro_case_with_nums_as_word(&self) -> String;

    /// Converts a string to macro case using the specified characters as
    /// separators.
    ///
    /// This method targets only the upper and lower cases of ASCII alphabets for
    /// capitalization, and the characters specified as the second argument of this
    /// method are regarded as word separators and are replaced to underscores.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let macro_ = "foo-bar100%baz".to_macro_case_with_sep("- ");
    ///     assert_eq!(macro_, "FOO_BAR100%_BAZ");
    /// ```
    fn to_macro_case_with_sep(&self, seps: &str) -> String;

    /// Converts a string to macro case using characters other than the specified
    /// characters as separators.
    ///
    /// This method targets only the upper and lower cases of ASCII alphabets for
    /// capitalization, and the characters other than the specified characters as
    /// the second argument of this method are regarded as word separators and are
    /// replaced to underscores.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let macro_ = "foo-bar100%baz".to_macro_case_with_keep("%");
    ///     assert_eq!(macro_, "FOO_BAR100%_BAZ");
    /// ```
    fn to_macro_case_with_keep(&self, keeped: &str) -> String;

    // pascal case

    /// Converts a string to pascal case.
    ///
    /// This method targets the upper and lower cases of ASCII alphabets for
    /// capitalization, and all characters except ASCII alphabets and ASCII numbers
    /// are eliminated as word separators.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let pascal = "foo-bar100%baz".to_pascal_case();
    ///     assert_eq!(pascal, "FooBar100Baz");
    /// ```
    fn to_pascal_case(&self) -> String;

    /// Converts a string to pascal case using the specified characters as
    /// separators.
    ///
    /// This method targets only the upper and lower cases of ASCII alphabets for
    /// capitalization, and the characters specified as the second argument of this
    /// method are regarded as word separators and are eliminated.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let pascal = "foo-bar100%baz".to_pascal_case_with_sep("- ");
    ///     assert_eq!(pascal, "FooBar100%Baz");
    /// ```
    fn to_pascal_case_with_sep(&self, seps: &str) -> String;

    /// Converts a string to pascal case using characters other than the specified
    /// characters as separators.
    ///
    /// This method targets only the upper and lower cases of ASCII alphabets for
    /// capitalization, and the characters other than the specified characters as
    /// the second argument of this method are regarded as word separators and are
    /// eliminated.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let pascal = "foo-bar100%baz".to_pascal_case_with_keep("%");
    ///     assert_eq!(pascal, "FooBar100%Baz");
    /// ```
    fn to_pascal_case_with_keep(&self, keeped: &str) -> String;

    // snake case

    /// Converts a string to snake case.
    ///
    /// This method targets the upper and lower cases of ASCII alphabets for
    /// capitalization, and all characters except ASCII alphabets and ASCII numbers
    /// are replaced to underscores as word separators.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let snake = "foo-bar100%baz".to_snake_case();
    ///     assert_eq!(snake, "foo_bar100_baz");
    /// ```
    fn to_snake_case(&self) -> String;

    /// Converts a string to snake case.
    ///
    /// This method targets the upper and lower cases of ASCII alphabets and
    /// ASCII numbers for capitalization, and all characters except ASCII
    /// alphabets and ASCII numbers are replaced to underscores as word separators.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let snake = "foo-bar100%baz".to_snake_case_with_nums_as_word();
    ///     assert_eq!(snake, "foo_bar_100_baz");
    /// ```
    fn to_snake_case_with_nums_as_word(&self) -> String;

    /// Converts a string to snake case using the specified characters as
    /// separators.
    ///
    /// This method targets only the upper and lower cases of ASCII alphabets for
    /// capitalization, and the characters specified as the second argument of this
    /// method are regarded as word separators and are replaced to underscores.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let snake = "foo-bar100%baz".to_snake_case_with_sep("- ");
    ///     assert_eq!(snake, "foo_bar100%_baz");
    /// ```
    fn to_snake_case_with_sep(&self, seps: &str) -> String;

    /// Converts a string to snake case using characters other than the specified
    /// characters as separators.
    ///
    /// This method targets only the upper and lower cases of ASCII alphabets for
    /// capitalization, and the characters other than the specified characters as
    /// the second argument of this method are regarded as word separators and
    /// are replaced to underscores.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let snake = "foo-bar100%baz".to_snake_case_with_keep("%");
    ///     assert_eq!(snake, "foo_bar100%_baz");
    /// ```
    fn to_snake_case_with_keep(&self, keeped: &str) -> String;

    // train case

    /// Converts a string to train case.
    ///
    /// This method targets the upper and lower cases of ASCII alphabets for
    /// capitalization, and all characters except ASCII alphabets and ASCII numbers
    /// are replaced to hyphens as word separators.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let train = "foo-bar100%baz".to_train_case();
    ///     assert_eq!(train, "Foo-Bar100-Baz");
    /// ```
    fn to_train_case(&self) -> String;

    /// Converts a string to train case.
    ///
    /// This method targets the upper and lower cases of ASCII alphabets and
    /// ASCII numbers for capitalization, and all characters except ASCII alphabets
    /// and ASCII numbers are replaced to hyphens as word separators.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let train = "foo-bar100%baz".to_train_case_with_nums_as_word();
    ///     assert_eq!(train, "Foo-Bar-100-Baz");
    /// ```
    fn to_train_case_with_nums_as_word(&self) -> String;

    /// Converts a string to train case using the specified characters as
    /// separators.
    ///
    /// This method targets only the upper and lower cases of ASCII alphabets for
    /// capitalization, and the characters specified as the second argument of this
    /// method are regarded as word separators and are replaced to hyphens.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let train = "foo-bar100%baz".to_train_case_with_sep("- ");
    ///     assert_eq!(train, "Foo-Bar100%-Baz");
    /// ```
    fn to_train_case_with_sep(&self, seps: &str) -> String;

    /// Converts a string to train case using characters other than the specified
    /// characters as separators.
    ///
    /// This method targets only the upper and lower cases of ASCII alphabets for
    /// capitalization, and the characters other than the specified characters as
    /// the second argument of this method are regarded as word separators and are
    /// replaced to hyphens.
    ///
    /// ```rust
    ///     use stringcase::Caser;
    ///
    ///     let train = "foo-bar100%baz".to_train_case_with_keep("%");
    ///     assert_eq!(train, "Foo-Bar100%-Baz");
    /// ```
    fn to_train_case_with_keep(&self, keeped: &str) -> String;
}

impl<T: AsRef<str>> Caser<T> for T {
    // camel case

    fn to_camel_case(&self) -> String {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "",
            keep: "",
        };
        camel_case_with_options(&self.as_ref(), &opts)
    }

    fn to_camel_case_with_options(&self, opts: &Options) -> String {
        camel_case_with_options(&self.as_ref(), opts)
    }

    fn to_camel_case_with_sep(&self, seps: &str) -> String {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: seps,
            keep: "",
        };
        camel_case_with_options(&self.as_ref(), &opts)
    }

    fn to_camel_case_with_keep(&self, kept: &str) -> String {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "",
            keep: kept,
        };
        camel_case_with_options(&self.as_ref(), &opts)
    }

    // cobol case

    fn to_cobol_case(&self) -> String {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "",
            keep: "",
        };
        cobol_case_with_options(&self.as_ref(), &opts)
    }

    fn to_cobol_case_with_options(&self, opts: &Options) -> String {
        cobol_case_with_options(&self.as_ref(), opts)
    }

    fn to_cobol_case_with_nums_as_word(&self) -> String {
        let opts = Options {
            separate_before_non_alphabets: true,
            separate_after_non_alphabets: true,
            separators: "",
            keep: "",
        };
        cobol_case_with_options(&self.as_ref(), &opts)
    }

    fn to_cobol_case_with_sep(&self, seps: &str) -> String {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: seps,
            keep: "",
        };
        cobol_case_with_options(&self.as_ref(), &opts)
    }

    fn to_cobol_case_with_keep(&self, kept: &str) -> String {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "",
            keep: kept,
        };
        cobol_case_with_options(&self.as_ref(), &opts)
    }

    // kebab case

    fn to_kebab_case(&self) -> String {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "",
            keep: "",
        };
        kebab_case_with_options(&self.as_ref(), &opts)
    }

    fn to_kebab_case_with_options(&self, opts: &Options) -> String {
        kebab_case_with_options(&self.as_ref(), opts)
    }

    fn to_kebab_case_with_nums_as_word(&self) -> String {
        let opts = Options {
            separate_before_non_alphabets: true,
            separate_after_non_alphabets: true,
            separators: "",
            keep: "",
        };
        kebab_case_with_options(&self.as_ref(), &opts)
    }

    fn to_kebab_case_with_sep(&self, seps: &str) -> String {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: seps,
            keep: "",
        };
        kebab_case_with_options(&self.as_ref(), &opts)
    }

    fn to_kebab_case_with_keep(&self, kept: &str) -> String {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "",
            keep: kept,
        };
        kebab_case_with_options(&self.as_ref(), &opts)
    }

    // macro case

    fn to_macro_case(&self) -> String {
        macro_case(&self.as_ref())
    }

    fn to_macro_case_with_nums_as_word(&self) -> String {
        macro_case_with_nums_as_word(&self.as_ref())
    }

    fn to_macro_case_with_sep(&self, seps: &str) -> String {
        macro_case_with_sep(&self.as_ref(), seps)
    }

    fn to_macro_case_with_keep(&self, keeped: &str) -> String {
        macro_case_with_keep(&self.as_ref(), keeped)
    }

    // pascal case

    fn to_pascal_case(&self) -> String {
        pascal_case(&self.as_ref())
    }

    fn to_pascal_case_with_sep(&self, seps: &str) -> String {
        pascal_case_with_sep(&self.as_ref(), seps)
    }

    fn to_pascal_case_with_keep(&self, keeped: &str) -> String {
        pascal_case_with_keep(&self.as_ref(), keeped)
    }

    // snake case

    fn to_snake_case(&self) -> String {
        snake_case(&self.as_ref())
    }

    fn to_snake_case_with_nums_as_word(&self) -> String {
        snake_case_with_nums_as_word(&self.as_ref())
    }

    fn to_snake_case_with_sep(&self, seps: &str) -> String {
        snake_case_with_sep(&self.as_ref(), seps)
    }

    fn to_snake_case_with_keep(&self, keeped: &str) -> String {
        snake_case_with_keep(&self.as_ref(), keeped)
    }

    // train case

    fn to_train_case(&self) -> String {
        train_case(&self.as_ref())
    }

    fn to_train_case_with_nums_as_word(&self) -> String {
        train_case_with_nums_as_word(&self.as_ref())
    }

    fn to_train_case_with_sep(&self, seps: &str) -> String {
        train_case_with_sep(&self.as_ref(), seps)
    }

    fn to_train_case_with_keep(&self, keeped: &str) -> String {
        train_case_with_keep(&self.as_ref(), keeped)
    }
}

#[cfg(test)]
mod tests_of_caser {
    use super::*;

    // camel case

    #[test]
    fn it_should_convert_to_camel_case() {
        let result = "foo_bar100%BAZQux".to_camel_case();
        assert_eq!(result, "fooBar100BazQux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_camel_case();
        assert_eq!(result, "fooBar100BazQux");
    }

    #[test]
    fn it_should_convert_to_camel_case_with_sep() {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "_",
            keep: "",
        };

        let result = "foo_bar100%BAZQux".to_camel_case_with_options(&opts);
        assert_eq!(result, "fooBar100%BazQux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_camel_case_with_options(&opts);
        assert_eq!(result, "fooBar100%BazQux");
    }

    #[test]
    fn it_should_convert_to_camel_case_with_keep() {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "",
            keep: "%",
        };

        let result = "foo_bar100%BAZQux".to_camel_case_with_options(&opts);
        assert_eq!(result, "fooBar100%BazQux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_camel_case_with_options(&opts);
        assert_eq!(result, "fooBar100%BazQux");
    }

    // cobol case

    #[test]
    fn it_should_convert_to_cobol_case() {
        let result = "foo_bar100%BAZQux".to_cobol_case();
        assert_eq!(result, "FOO-BAR100-BAZ-QUX");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_cobol_case();
        assert_eq!(result, "FOO-BAR100-BAZ-QUX");
    }

    #[test]
    fn it_should_convert_to_cobol_case_with_nums_as_word() {
        let opts = Options {
            separate_before_non_alphabets: true,
            separate_after_non_alphabets: true,
            separators: "",
            keep: "",
        };

        let result = "foo_bar100%BAZQux".to_cobol_case_with_options(&opts);
        assert_eq!(result, "FOO-BAR-100-BAZ-QUX");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_cobol_case_with_options(&opts);
        assert_eq!(result, "FOO-BAR-100-BAZ-QUX");
    }

    #[test]
    fn it_should_convert_to_cobol_case_with_sep() {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "_",
            keep: "",
        };

        let result = "foo_bar100%BAZQux".to_cobol_case_with_options(&opts);
        assert_eq!(result, "FOO-BAR100%-BAZ-QUX");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_cobol_case_with_options(&opts);
        assert_eq!(result, "FOO-BAR100%-BAZ-QUX");
    }

    #[test]
    fn it_should_convert_to_cobol_case_with_keep() {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "",
            keep: "%",
        };

        let result = "foo_bar100%BAZQux".to_cobol_case_with_options(&opts);
        assert_eq!(result, "FOO-BAR100%-BAZ-QUX");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_cobol_case_with_options(&opts);
        assert_eq!(result, "FOO-BAR100%-BAZ-QUX");
    }

    // kebab case

    #[test]
    fn it_should_convert_to_kebab_case() {
        let result = "foo_bar100%BAZQux".to_kebab_case();
        assert_eq!(result, "foo-bar100-baz-qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_kebab_case();
        assert_eq!(result, "foo-bar100-baz-qux");
    }

    #[test]
    fn it_should_convert_to_kebab_case_with_nums_as_word() {
        let opts = Options {
            separate_before_non_alphabets: true,
            separate_after_non_alphabets: true,
            separators: "",
            keep: "",
        };

        let result = "foo_bar100%BAZQux".to_kebab_case_with_options(&opts);
        assert_eq!(result, "foo-bar-100-baz-qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_kebab_case_with_options(&opts);
        assert_eq!(result, "foo-bar-100-baz-qux");
    }

    #[test]
    fn it_should_convert_to_kebab_case_with_sep() {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "_",
            keep: "",
        };

        let result = "foo_bar100%BAZQux".to_kebab_case_with_options(&opts);
        assert_eq!(result, "foo-bar100%-baz-qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_kebab_case_with_options(&opts);
        assert_eq!(result, "foo-bar100%-baz-qux");
    }

    #[test]
    fn it_should_convert_to_kebab_case_with_keep() {
        let opts = Options {
            separate_before_non_alphabets: false,
            separate_after_non_alphabets: true,
            separators: "",
            keep: "%",
        };

        let result = "foo_bar100%BAZQux".to_kebab_case_with_options(&opts);
        assert_eq!(result, "foo-bar100%-baz-qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_kebab_case_with_options(&opts);
        assert_eq!(result, "foo-bar100%-baz-qux");
    }

    // macro case

    #[test]
    fn it_should_convert_to_macro_case() {
        let result = "foo_bar100%BAZQux".to_macro_case();
        assert_eq!(result, "FOO_BAR100_BAZ_QUX");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_macro_case();
        assert_eq!(result, "FOO_BAR100_BAZ_QUX");
    }

    #[test]
    fn it_should_convert_to_macro_case_with_nums_as_word() {
        let result = "foo_bar100%BAZQux".to_macro_case_with_nums_as_word();
        assert_eq!(result, "FOO_BAR_100_BAZ_QUX");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_macro_case_with_nums_as_word();
        assert_eq!(result, "FOO_BAR_100_BAZ_QUX");
    }

    #[test]
    fn it_should_convert_to_macro_case_with_sep() {
        let result = "foo_bar100%BAZQux".to_macro_case_with_sep("_");
        assert_eq!(result, "FOO_BAR100%_BAZ_QUX");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_macro_case_with_sep("_");
        assert_eq!(result, "FOO_BAR100%_BAZ_QUX");
    }

    #[test]
    fn it_should_convert_to_macro_case_with_keep() {
        let result = "foo_bar100%BAZQux".to_macro_case_with_keep("%");
        assert_eq!(result, "FOO_BAR100%_BAZ_QUX");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_macro_case_with_keep("%");
        assert_eq!(result, "FOO_BAR100%_BAZ_QUX");
    }

    // pascal case

    #[test]
    fn it_should_convert_to_pascal_case() {
        let result = "foo_bar100%BAZQux".to_pascal_case();
        assert_eq!(result, "FooBar100BazQux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_pascal_case();
        assert_eq!(result, "FooBar100BazQux");
    }

    #[test]
    fn it_should_convert_to_pascal_case_with_sep() {
        let result = "foo_bar100%BAZQux".to_pascal_case_with_sep("_");
        assert_eq!(result, "FooBar100%BazQux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_pascal_case_with_sep("_");
        assert_eq!(result, "FooBar100%BazQux");
    }

    #[test]
    fn it_should_convert_to_pascal_case_with_keep() {
        let result = "foo_bar100%BAZQux".to_pascal_case_with_keep("%");
        assert_eq!(result, "FooBar100%BazQux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_pascal_case_with_keep("%");
        assert_eq!(result, "FooBar100%BazQux");
    }

    // snake case

    #[test]
    fn it_should_convert_to_snake_case() {
        let result = "foo_bar100%BAZQux".to_snake_case();
        assert_eq!(result, "foo_bar100_baz_qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_snake_case();
        assert_eq!(result, "foo_bar100_baz_qux");
    }

    #[test]
    fn it_should_convert_to_snake_case_with_nums_as_word() {
        let result = "foo_bar100%BAZQux".to_snake_case_with_nums_as_word();
        assert_eq!(result, "foo_bar_100_baz_qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_snake_case_with_nums_as_word();
        assert_eq!(result, "foo_bar_100_baz_qux");
    }

    #[test]
    fn it_should_convert_to_snake_case_with_sep() {
        let result = "foo_bar100%BAZQux".to_snake_case_with_sep("_");
        assert_eq!(result, "foo_bar100%_baz_qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_snake_case_with_sep("_");
        assert_eq!(result, "foo_bar100%_baz_qux");
    }

    #[test]
    fn it_should_convert_to_snake_case_with_keep() {
        let result = "foo_bar100%BAZQux".to_snake_case_with_keep("%");
        assert_eq!(result, "foo_bar100%_baz_qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_snake_case_with_keep("%");
        assert_eq!(result, "foo_bar100%_baz_qux");
    }

    // train case

    #[test]
    fn it_should_convert_to_train_case() {
        let result = "foo_bar100%BAZQux".to_train_case();
        assert_eq!(result, "Foo-Bar100-Baz-Qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_train_case();
        assert_eq!(result, "Foo-Bar100-Baz-Qux");
    }

    #[test]
    fn it_should_convert_to_train_case_with_nums_as_word() {
        let result = "foo_bar100%BAZQux".to_train_case_with_nums_as_word();
        assert_eq!(result, "Foo-Bar-100-Baz-Qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_train_case_with_nums_as_word();
        assert_eq!(result, "Foo-Bar-100-Baz-Qux");
    }

    #[test]
    fn it_should_convert_to_train_case_with_sep() {
        let result = "foo_bar100%BAZQux".to_train_case_with_sep("_");
        assert_eq!(result, "Foo-Bar100%-Baz-Qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_train_case_with_sep("_");
        assert_eq!(result, "Foo-Bar100%-Baz-Qux");
    }

    #[test]
    fn it_should_convert_to_train_case_with_keep() {
        let result = "foo_bar100%BAZQux".to_train_case_with_keep("%");
        assert_eq!(result, "Foo-Bar100%-Baz-Qux");

        let string = String::from("foo_bar100%BAZQux");
        let result = string.to_train_case_with_keep("%");
        assert_eq!(result, "Foo-Bar100%-Baz-Qux");
    }
}
