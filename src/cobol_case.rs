// Copyright (C) 2024-2026 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::options::Options;
use crate::upperize::upperize;

/// Converts the input string to cobol case with the specified options.
///
/// ```rust
///     let opts = stringcase::Options{
///       separate_before_non_alphabets: true,
///       separate_after_non_alphabets: true,
///       separators: "",
///       keep: "",
///     };
///     let cobol = stringcase::cobol_case_with_options("fooBar123Baz", &opts);
///     assert_eq!(cobol, "FOO-BAR-123-BAZ");
/// ```
#[inline(always)]
pub fn cobol_case_with_options(input: &str, opts: &Options) -> String {
    upperize::<'-'>(input, opts)
}

/// Converts the input string to cobol case.
///
/// It treats the end of a sequence of non-alphabetic characters as a word boundary, but not
/// the beginning.
///
/// ```rust
///     let cobol = stringcase::cobol_case("fooBar123Baz");
///     assert_eq!(cobol, "FOO-BAR123-BAZ");
/// ```
#[inline(always)]
pub fn cobol_case(input: &str) -> String {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "",
    };
    upperize::<'-'>(input, &opts)
}

/// Converts the input string to cobol case with the specified separator characters.
#[doc(hidden)]
#[deprecated(since = "0.4.0", note = "Should use cobol_case_with_options instead")]
#[inline(always)]
pub fn cobol_case_with_sep(input: &str, seps: &str) -> String {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: seps,
        keep: "",
    };
    upperize::<'-'>(input, &opts)
}

/// Converts the input string to cobol case with the specified characters to be kept.
#[doc(hidden)]
#[deprecated(since = "0.4.0", note = "Should use cobol_case_with_options instead")]
#[inline(always)]
pub fn cobol_case_with_keep(input: &str, kept: &str) -> String {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "",
        keep: kept,
    };
    upperize::<'-'>(input, &opts)
}

/// Converts the input string to cobol case.
///
/// It treats the beginning and the end of a sequence of non-alphabetic characters as a word
/// boundary.
#[doc(hidden)]
#[deprecated(since = "0.4.0", note = "Should use cobol_case_with_options instead")]
#[inline(always)]
pub fn cobol_case_with_nums_as_word(input: &str) -> String {
    let opts = Options {
        separate_before_non_alphabets: true,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "",
    };
    upperize::<'-'>(input, &opts)
}

#[cfg(test)]
mod tests_of_cobol_case {
    use super::*;

    #[test]
    fn convert_camel_case() {
        assert_eq!(cobol_case("abcDefGHIjk"), "ABC-DEF-GH-IJK");
    }

    #[test]
    fn convert_pascal_case() {
        assert_eq!(cobol_case("AbcDefGHIjk"), "ABC-DEF-GH-IJK");
    }

    #[test]
    fn convert_snake_case() {
        assert_eq!(cobol_case("abc_def_ghi"), "ABC-DEF-GHI");
    }

    #[test]
    fn convert_kebab_case() {
        assert_eq!(cobol_case("abc-def-ghi"), "ABC-DEF-GHI");
    }

    #[test]
    fn convert_train_case() {
        assert_eq!(cobol_case("Abc-Def-Ghi"), "ABC-DEF-GHI");
    }

    #[test]
    fn convert_macro_case() {
        assert_eq!(cobol_case("ABC_DEF_GHI"), "ABC-DEF-GHI");
    }

    #[test]
    fn convert_cobol_case() {
        assert_eq!(cobol_case("ABC-DEF-GHI"), "ABC-DEF-GHI");
    }

    #[test]
    fn convert_with_digits() {
        assert_eq!(
            cobol_case("abc123-456defG89HIJklMN12"),
            "ABC123-456-DEF-G89-HI-JKL-MN12",
        );
    }

    #[test]
    fn convert_with_symbols_as_separators() {
        assert_eq!(
            cobol_case(":.abc~!@def#$ghi%&jk(lm)no/?"),
            "ABC-DEF-GHI-JK-LM-NO",
        );
    }

    #[test]
    fn convert_starting_with_digit() {
        assert_eq!(cobol_case("123abc456def"), "123-ABC456-DEF");
        assert_eq!(cobol_case("123ABC456DEF"), "123-ABC456-DEF");
        assert_eq!(cobol_case("123Abc456Def"), "123-ABC456-DEF");
    }

    #[test]
    fn convert_empty_string() {
        assert_eq!(cobol_case(""), "");
    }
}

#[cfg(test)]
mod tests_of_cobol_case_with_options {
    use super::*;

    mod non_alphabets_as_head_of_a_word {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_macro_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC-123-456DEF-G-89HI-JKL-MN-12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "ABC-DEF-GHI-JK-LM-NO");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123ABC-456DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC-456DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC-456-DEF");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }

    mod non_alphabets_as_tail_of_a_word {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_macro_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456-DEF-G89-HI-JKL-MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "ABC-DEF-GHI-JK-LM-NO");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-ABC456-DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-ABC456-DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC456-DEF");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }

    mod non_alphabets_as_a_word {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_macro_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC-123-456-DEF-G-89-HI-JKL-MN-12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "ABC-DEF-GHI-JK-LM-NO");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-ABC-456-DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-ABC-456-DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC-456-DEF");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }

    mod non_alphabets_as_part_of_a_word {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_macro_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456DEF-G89HI-JKL-MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "ABC-DEF-GHI-JK-LM-NO");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC456-DEF");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = cobol_case_with_options("", &opts);
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
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "-";
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-_DEF-_GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC---DEF---GHI");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "-";
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-_DEF-_GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC-123-456DEF-G-89HI-JKL-MN-12");

            opts.separators = "_";
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC-123-456DEF-G-89HI-JKL-MN-12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".ABC-~!-DEF-#-GHI-%-JK-LM-NO-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123ABC-456DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC-456DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC-456-DEF");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("", &opts);
            assert_eq!(result, "");
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
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "-";
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC_-DEF_-GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "-";
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC_-DEF_-GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456-DEF-G89-HI-JKL-MN12");

            opts.separators = "_";
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456-DEF-G89-HI-JKL-MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".-ABC~!-DEF#-GHI%-JK-LM-NO-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-ABC456-DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-ABC456-DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC456-DEF");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("", &opts);
            assert_eq!(result, "");
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
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "-";
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-_-DEF-_-GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC---DEF---GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC---DEF---GHI");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "-";
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-_-DEF-_-GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC---DEF---GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC-123-456-DEF-G-89-HI-JKL-MN-12");

            opts.separators = "_";
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC-123-456-DEF-G-89-HI-JKL-MN-12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".-ABC-~!-DEF-#-GHI-%-JK-LM-NO-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-ABC-456-DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-ABC-456-DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC-456-DEF");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("", &opts);
            assert_eq!(result, "");
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
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "-";
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC_DEF_GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "-";
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC_DEF_GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.separators = "_";
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456DEF-G89HI-JKL-MN12");

            opts.separators = "_";
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456DEF-G89HI-JKL-MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".ABC~!-DEF#-GHI%-JK-LM-NO-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC456-DEF");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = cobol_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }

    mod non_alphabets_as_head_of_a_word_with_kept_characters {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-_DEF-_GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC---DEF---GHI");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                keep: "-",
                separators: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "_";
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-_DEF-_GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC-123-456DEF-G-89HI-JKL-MN-12");

            opts.keep = "-";
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC-123-456DEF-G-89HI-JKL-MN-12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                keep: ".~!#%?",
                separators: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".ABC-~!-DEF-#-GHI-%-JK-LM-NO-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123ABC-456DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC-456DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC-456-DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }

    mod non_alphabets_as_tail_of_a_word_with_kept_characters {
        use super::*;

        #[test]
        fn convert_camel_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                keep: "-",
                separators: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "_";
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC_-DEF_-GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                keep: "-",
                separators: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "_";
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC_-DEF_-GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456-DEF-G89-HI-JKL-MN12");

            opts.keep = "-";
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456-DEF-G89-HI-JKL-MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                keep: ".~!#%?",
                separators: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".-ABC~!-DEF#-GHI%-JK-LM-NO-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-ABC456-DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-ABC456-DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC456-DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("", &opts);
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
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                keep: "-",
                separators: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "_";
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-_-DEF-_-GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC---DEF---GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC---DEF---GHI");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                keep: "-",
                separators: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "_";
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-_-DEF-_-GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC---DEF---GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC-123-456-DEF-G-89-HI-JKL-MN-12");

            opts.keep = "-";
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC-123-456-DEF-G-89-HI-JKL-MN-12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                keep: ".~!#%?",
                separators: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".-ABC-~!-DEF-#-GHI-%-JK-LM-NO-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123-ABC-456-DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123-ABC-456-DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC-456-DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("", &opts);
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
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC-DEF-GH-IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                keep: "-",
                separators: "",
            };
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "_";
            let result = cobol_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "ABC_DEF_GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC--DEF--GHI");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                keep: "-",
                separators: "",
            };
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "_";
            let result = cobol_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC_DEF_GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");

            opts.keep = "-";
            let result = cobol_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                keep: "_",
                separators: "",
            };
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456DEF-G89HI-JKL-MN12");

            opts.keep = "-";
            let result = cobol_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456DEF-G89HI-JKL-MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                keep: ".~!#%?",
                separators: "",
            };
            let result = cobol_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".ABC~!-DEF#-GHI%-JK-LM-NO-?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = cobol_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = cobol_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123-ABC456-DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                keep: "-_",
                separators: "",
            };
            let result = cobol_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }
}
