// Copyright (C) 2024-2025 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::capitalize::capitalize;
use crate::options::Options;

/// Converts the input string to train case with the specified options.
///
/// ```rust
///     let opts = stringcase::Options{
///       separate_before_non_alphabets: true,
///       separate_after_non_alphabets: true,
///       separators: "",
///       keep: "",
///     };
///     let train = stringcase::train_case_with_options("fooBar123Baz", &opts);
///     assert_eq!(train, "Foo-Bar-123-Baz");
/// ```
pub fn train_case_with_options(input: &str, opts: &Options) -> String {
    capitalize::<'-'>(input, opts)
}

/// Converts the input string to train case.
///
/// It treats the end of a sequence of non-alphabetical characters as a word boundary, but not
/// the beginning.
///
/// ```rust
///     let train = stringcase::train_case("fooBar123Baz");
///     assert_eq!(train, "Foo-Bar123-Baz");
/// ```
pub fn train_case(input: &str) -> String {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "",
    };
    capitalize::<'-'>(input, &opts)
}

/// Converts the input string to train case with the specified separator characters.
#[deprecated(since = "0.4.0", note = "Should use train_case_with_options instead")]
pub fn train_case_with_sep(input: &str, seps: &str) -> String {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: seps,
        keep: "",
    };
    capitalize::<'-'>(input, &opts)
}

/// Converts the input string to train case with the specified characters to be kept.
#[deprecated(since = "0.4.0", note = "Should use train_case_with_options instead")]
pub fn train_case_with_keep(input: &str, kept: &str) -> String {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "",
        keep: kept,
    };
    capitalize::<'-'>(input, &opts)
}

/// Converts the input string to train case.
///
/// It treats the beginning and the end of a sequence of non-alphabetical characters as a word
/// boundary.
#[deprecated(since = "0.4.0", note = "Should use train_case_with_options instead")]
pub fn train_case_with_nums_as_word(input: &str) -> String {
    let opts = Options {
        separate_before_non_alphabets: true,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "",
    };
    capitalize::<'-'>(input, &opts)
}

#[cfg(test)]
mod tests_of_train_case {
    use super::*;

    #[test]
    fn convert_camel_case() {
        let result = train_case("abcDefGHIjk");
        assert_eq!(result, "Abc-Def-Gh-Ijk");
    }

    #[test]
    fn convert_pascal_case() {
        let result = train_case("AbcDefGHIjk");
        assert_eq!(result, "Abc-Def-Gh-Ijk");
    }

    #[test]
    fn convert_snake_case() {
        let result = train_case("abc_def_ghi");
        assert_eq!(result, "Abc-Def-Ghi");
    }

    #[test]
    fn convert_kebab_case() {
        let result = train_case("abc-def-ghi");
        assert_eq!(result, "Abc-Def-Ghi");
    }

    #[test]
    fn convert_train_case() {
        let result = train_case("Abc-Def-Ghi");
        assert_eq!(result, "Abc-Def-Ghi");
    }

    #[test]
    fn convert_macro_case() {
        let result = train_case("ABC_DEF_GHI");
        assert_eq!(result, "Abc-Def-Ghi");
    }

    #[test]
    fn convert_cobol_case() {
        let result = train_case("ABC-DEF-GHI");
        assert_eq!(result, "Abc-Def-Ghi");
    }

    #[test]
    fn convert_with_keeping_digits() {
        let result = train_case("abc123-456defG89HIJklMN12");
        assert_eq!(result, "Abc123-456-Def-G89-Hi-Jkl-Mn12");
    }

    #[test]
    fn convert_with_symbols_as_separators() {
        let result = train_case(":.abc~!@def#$ghi%&jk(lm)no/?");
        assert_eq!(result, "Abc-Def-Ghi-Jk-Lm-No");
    }

    #[test]
    fn convert_when_starting_with_digit() {
        let result = train_case("123abc456def");
        assert_eq!(result, "123-Abc456-Def");

        let result = train_case("123ABC456DEF");
        assert_eq!(result, "123-Abc456-Def");

        let result = train_case("123Abc456Def");
        assert_eq!(result, "123-Abc456-Def");
    }

    #[test]
    fn convert_empty_string() {
        let result = train_case("");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_cobol_case_with_options {
    use super::*;

    mod non_alphabets_as_head_of_a_word {
        use super::*;

        #[test]
        fn convert_camel_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_train_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc-123-456def-G-89hi-Jkl-Mn-12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc-Def-Ghi-Jk-Lm-No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc-456def");

            let result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc-456def");

            let result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc-456-Def");
        }

        #[test]
        fn convert_empty_string_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }

    mod non_alphabets_as_tail_of_a_word {
        use super::*;

        #[test]
        fn convert_camel_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_train_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456-Def-G89-Hi-Jkl-Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc-Def-Ghi-Jk-Lm-No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-Abc456-Def");

            let result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-Abc456-Def");

            let result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc456-Def");
        }

        #[test]
        fn convert_empty_string_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }

    mod non_alphabets_as_a_word {
        use super::*;

        #[test]
        fn convert_camel_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_train_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc-123-456-Def-G-89-Hi-Jkl-Mn-12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc-Def-Ghi-Jk-Lm-No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-Abc-456-Def");

            let result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-Abc-456-Def");

            let result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc-456-Def");
        }

        #[test]
        fn convert_empty_string_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }

    mod non_alphabets_as_part_of_a_word {
        use super::*;

        #[test]
        fn convert_camel_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_train_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456def-G89hi-Jkl-Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc-Def-Ghi-Jk-Lm-No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");

            let result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc456-Def");
        }

        #[test]
        fn convert_empty_string_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }

    mod non_alphabets_as_head_of_a_word_with_separators {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "-";
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-_def-_ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "_";
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc--def--ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "_";
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc---Def---Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "-";
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-_def-_ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "_";
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc--def--ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc-123-456def-G-89hi-Jkl-Mn-12");

            opts.separators = "_";
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc-123-456def-G-89hi-Jkl-Mn-12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc-~!-Def-#-Ghi-%-Jk-Lm-No-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc-456def");

            let result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc-456def");

            let result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc-456-Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }

        #[test]
        fn alphabets_and_numbers_in_separators_have_no_effect() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-b2",
                keep: "",
            };
            let result = train_case_with_options("abc123def", &opts);
            assert_eq!(result, "Abc-123def");
        }
    }

    mod non_alphabets_as_tail_of_a_word_with_separators {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "-";
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_-Def_-Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "_";
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc--Def--Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "_";
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc--Def--Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "-";
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_-Def_-Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "_";
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc--Def--Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456-Def-G89-Hi-Jkl-Mn12");

            opts.separators = "_";
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456-Def-G89-Hi-Jkl-Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".-Abc~!-Def#-Ghi%-Jk-Lm-No-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-Abc456-Def");

            let result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-Abc456-Def");

            let result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc456-Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }

        #[test]
        fn alphabets_and_numbers_in_separators_are_no_effect() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-b2",
                keep: "",
            };
            let result = train_case_with_options("abc123def", &opts);
            assert_eq!(result, "Abc123-Def");
        }
    }

    mod non_alphabets_as_a_word_with_separators {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "-";
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-_-Def-_-Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "_";
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc---Def---Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "_";
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc---Def---Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "-";
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-_-Def-_-Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "_";
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc---Def---Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc-123-456-Def-G-89-Hi-Jkl-Mn-12");

            opts.separators = "_";
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc-123-456-Def-G-89-Hi-Jkl-Mn-12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".-Abc-~!-Def-#-Ghi-%-Jk-Lm-No-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-Abc-456-Def");

            let result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-Abc-456-Def");

            let result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc-456-Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }

        #[test]
        fn alphabets_and_numbers_in_separators_no_effect() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-b2",
                keep: "",
            };
            let result = train_case_with_options("abc123def", &opts);
            assert_eq!(result, "Abc-123-Def");
        }
    }

    mod non_alphabets_as_part_of_a_word_with_separators {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "-";
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_def_ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "_";
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-def-ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "_";
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc--Def--Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.separators = "-";
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_def_ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456def-G89hi-Jkl-Mn12");

            opts.separators = "_";
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456def-G89hi-Jkl-Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc~!-Def#-Ghi%-Jk-Lm-No-?");
        }

        #[test]
        fn convert_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");

            let result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc456-Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }

        #[test]
        fn alphabets_and_numbers_in_separators_have_no_effect() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-b2",
                keep: "",
            };
            let result = train_case_with_options("abc123def", &opts);
            assert_eq!(result, "Abc123def");
        }
    }

    mod non_alphabets_as_head_of_a_word_with_kept_characters {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let mut result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "_";
            result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-_def-_ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc--def--ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc---Def---Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let mut result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "_";
            result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-_def-_ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc--def--ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc-123-456def-G-89hi-Jkl-Mn-12");

            opts.keep = "-";
            result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc-123-456def-G-89hi-Jkl-Mn-12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: ".~!#%?",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc-~!-Def-#-Ghi-%-Jk-Lm-No-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let mut result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc-456def");

            result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc-456def");

            result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc-456-Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }

        #[test]
        fn alphabets_and_numbers_in_separators_have_no_effect() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-b2",
            };
            let result = train_case_with_options("abc123def", &opts);
            assert_eq!(result, "Abc-123def");
        }
    }

    mod non_alphabets_as_tail_of_a_word_with_kept_characters {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "_";
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_-Def_-Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc--Def--Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc--Def--Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "_";
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_-Def_-Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc--Def--Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456-Def-G89-Hi-Jkl-Mn12");

            opts.keep = "-";
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456-Def-G89-Hi-Jkl-Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: ".~!#%?",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".-Abc~!-Def#-Ghi%-Jk-Lm-No-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-Abc456-Def");

            let result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-Abc456-Def");

            let result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc456-Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }

    mod non_alphabets_as_a_word_with_kept_characters {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "_";
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-_-Def-_-Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc---Def---Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc---Def---Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "_";
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-_-Def-_-Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc---Def---Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc-123-456-Def-G-89-Hi-Jkl-Mn-12");

            opts.keep = "-";
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc-123-456-Def-G-89-Hi-Jkl-Mn-12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: ".~!#%?",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".-Abc-~!-Def-#-Ghi-%-Jk-Lm-No-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-Abc-456-Def");

            let result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-Abc-456-Def");

            let result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc-456-Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }

    mod non_alphabets_as_part_of_a_word_with_kept_characters {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc-Def-Gh-Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "_";
            let result = train_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_def_ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            let result = train_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-def-ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            let result = train_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc--Def--Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "_";
            let result = train_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_def_ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-Def-Ghi");

            opts.keep = "-";
            let result = train_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-def-ghi");
        }

        #[test]
        fn convert_with_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456def-G89hi-Jkl-Mn12");

            opts.keep = "-";
            let result = train_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456def-G89hi-Jkl-Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: ".~!#%?",
            };
            let result = train_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc~!-Def#-Ghi%-Jk-Lm-No-?");
        }

        #[test]
        fn convert_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = train_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");

            let result = train_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-Abc456-Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = train_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }
}
