// Copyright (C) 2026 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::capitalize::capitalize;
use crate::options::Options;

/// Converts the input string to Ada case with the specified options.
///
/// ```rust
///     let opts = stringcase::Options{
///       separate_before_non_alphabets: true,
///       separate_after_non_alphabets: true,
///       separators: "",
///       keep: "",
///     };
///     let ada = stringcase::ada_case_with_options("fooBar123Baz", &opts);
///     assert_eq!(ada, "Foo_Bar_123_Baz");
/// ```
#[inline(always)]
pub fn ada_case_with_options(input: &str, opts: &Options) -> String {
    capitalize::<'_'>(input, opts)
}

/// Converts the input string to Ada case.
///
/// It treats the end of a sequence of non-alphabetical characters as a word boundary, but not
/// the beginning.
///
/// ```rust
///     let ada = stringcase::ada_case("fooBar123Baz");
///     assert_eq!(ada, "Foo_Bar123_Baz");
/// ```
#[inline(always)]
pub fn ada_case(input: &str) -> String {
    let opts = Options {
        separate_before_non_alphabets: false,
        separate_after_non_alphabets: true,
        separators: "",
        keep: "",
    };
    capitalize::<'_'>(input, &opts)
}

#[cfg(test)]
mod tests_of_ada_case {
    use super::*;

    #[test]
    fn convert_camel_case() {
        let result = ada_case("abcDefGHIjk");
        assert_eq!(result, "Abc_Def_Gh_Ijk");
    }

    #[test]
    fn convert_pascal_case() {
        let result = ada_case("AbcDefGHIjk");
        assert_eq!(result, "Abc_Def_Gh_Ijk");
    }

    #[test]
    fn convert_snake_case() {
        let result = ada_case("abc_def_ghi");
        assert_eq!(result, "Abc_Def_Ghi");
    }

    #[test]
    fn convert_kebab_case() {
        let result = ada_case("abc-def-ghi");
        assert_eq!(result, "Abc_Def_Ghi");
    }

    #[test]
    fn convert_train_case() {
        let result = ada_case("Abc-Def-Ghi");
        assert_eq!(result, "Abc_Def_Ghi");
    }

    #[test]
    fn convert_title_case() {
        let result = ada_case("Abc Def Ghi");
        assert_eq!(result, "Abc_Def_Ghi");
    }

    #[test]
    fn convert_ada_case() {
        let result = ada_case("Abc_Def_Ghi");
        assert_eq!(result, "Abc_Def_Ghi");
    }

    #[test]
    fn convert_macro_case() {
        let result = ada_case("ABC_DEF_GHI");
        assert_eq!(result, "Abc_Def_Ghi");
    }

    #[test]
    fn convert_cobol_case() {
        let result = ada_case("ABC-DEF-GHI");
        assert_eq!(result, "Abc_Def_Ghi");
    }

    #[test]
    fn convert_with_keeping_digits() {
        let result = ada_case("abc123-456defG89HIJklMN12");
        assert_eq!(result, "Abc123_456_Def_G89_Hi_Jkl_Mn12");
    }

    #[test]
    fn convert_with_symbols_as_separators() {
        let result = ada_case(":.abc~!@def#$ghi%&jk(lm)no/?");
        assert_eq!(result, "Abc_Def_Ghi_Jk_Lm_No");
    }

    #[test]
    fn convert_when_starting_with_digit() {
        let result = ada_case("123abc456def");
        assert_eq!(result, "123_Abc456_Def");

        let result = ada_case("123ABC456DEF");
        assert_eq!(result, "123_Abc456_Def");

        let result = ada_case("123Abc456Def");
        assert_eq!(result, "123_Abc456_Def");
    }

    #[test]
    fn convert_an_empty_string() {
        let result = ada_case("");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_ada_case_with_options {
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_train_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_title_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_ada_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc_123_456def_G_89hi_Jkl_Mn_12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc_Def_Ghi_Jk_Lm_No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc_456def");

            let result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc_456def");

            let result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc_456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("", &opts);
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_train_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_title_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_ada_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123_456_Def_G89_Hi_Jkl_Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc_Def_Ghi_Jk_Lm_No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123_Abc456_Def");

            let result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123_Abc456_Def");

            let result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("", &opts);
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_train_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_title_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_ada_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc_123_456_Def_G_89_Hi_Jkl_Mn_12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc_Def_Ghi_Jk_Lm_No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123_Abc_456_Def");

            let result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123_Abc_456_Def");

            let result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc_456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("", &opts);
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_train_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_title_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_ada_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123_456def_G89hi_Jkl_Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc_Def_Ghi_Jk_Lm_No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");

            let result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = ada_case_with_options("", &opts);
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "-";
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc__def__ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_-def_-ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_-_Def_-_Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "-";
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc__def__ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_-def_-ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc_123_456def_G_89hi_Jkl_Mn_12");

            opts.separators = "_";
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc_123-456def_G_89hi_Jkl_Mn_12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc_~!_Def_#_Ghi_%_Jk_Lm_No_?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc_456def");

            let result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc_456def");

            let result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc_456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = ada_case_with_options("", &opts);
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
            let result = ada_case_with_options("abc123def", &opts);
            assert_eq!(result, "Abc_123def");
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "-";
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc__Def__Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-_Def-_Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-_Def-_Ghi");
        }

        #[test]
        fn convert_title_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: " ",
                keep: "",
            };
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc _Def _Ghi");
        }

        #[test]
        fn convert_ada_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "-";
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc__Def__Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "-";
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc__Def__Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-_Def-_Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123_456_Def_G89_Hi_Jkl_Mn12");

            opts.separators = "_";
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456_Def_G89_Hi_Jkl_Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "._Abc~!_Def#_Ghi%_Jk_Lm_No_?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123_Abc456_Def");

            let result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123_Abc456_Def");

            let result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = ada_case_with_options("", &opts);
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
            let result = ada_case_with_options("abc123def", &opts);
            assert_eq!(result, "Abc123_Def");
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "-";
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc___Def___Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_-_Def_-_Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_-_Def_-_Ghi");
        }

        #[test]
        fn convert_title_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: " ",
                keep: "",
            };
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_ _Def_ _Ghi");
        }

        #[test]
        fn convert_ada_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "-";
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc___Def___Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "-";
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc___Def___Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_-_Def_-_Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc_123_456_Def_G_89_Hi_Jkl_Mn_12");

            opts.separators = "_";
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc_123-456_Def_G_89_Hi_Jkl_Mn_12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "._Abc_~!_Def_#_Ghi_%_Jk_Lm_No_?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123_Abc_456_Def");

            let result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123_Abc_456_Def");

            let result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc_456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = ada_case_with_options("", &opts);
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
            let result = ada_case_with_options("abc123def", &opts);
            assert_eq!(result, "Abc_123_Def");
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "-";
            let result = ada_case_with_options("abc_def_ghi", &opts);
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
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("abc-def-ghi", &opts);
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
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-_Def-_Ghi");
        }

        #[test]
        fn convert_title_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: " ",
                keep: "",
            };
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "_";
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc _Def _Ghi");
        }

        #[test]
        fn convert_ada_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "-";
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc__Def__Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.separators = "-";
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
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
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123_456def_G89hi_Jkl_Mn12");

            opts.separators = "_";
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456def_G89hi_Jkl_Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc~!_Def#_Ghi%_Jk_Lm_No_?");
        }

        #[test]
        fn convert_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");

            let result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = ada_case_with_options("", &opts);
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
            let result = ada_case_with_options("abc123def", &opts);
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let mut result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "_";
            result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc__def__ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_-def_-ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_-_Def_-_Ghi");
        }

        #[test]
        fn convert_title_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let mut result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = " ";
            result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_ _Def_ _Ghi");
        }

        #[test]
        fn convert_ada_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let mut result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "_";
            result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc___Def___Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let mut result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "_";
            result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc__def__ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_-def_-ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc_123_456def_G_89hi_Jkl_Mn_12");

            opts.keep = "-";
            result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc_123-456def_G_89hi_Jkl_Mn_12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: ".~!#%?",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc_~!_Def_#_Ghi_%_Jk_Lm_No_?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let mut result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc_456def");

            result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc_456def");

            result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc_456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = ada_case_with_options("", &opts);
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
            let result = ada_case_with_options("abc123def", &opts);
            assert_eq!(result, "Abc_123def");
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "_";
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc__Def__Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-_Def-_Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-_Def-_Ghi");
        }

        #[test]
        fn convert_title_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = " ";
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc _Def _Ghi");
        }

        #[test]
        fn convert_ada_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "_";
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc__Def__Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "_";
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc__Def__Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-_Def-_Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123_456_Def_G89_Hi_Jkl_Mn12");

            opts.keep = "-";
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456_Def_G89_Hi_Jkl_Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: ".~!#%?",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "._Abc~!_Def#_Ghi%_Jk_Lm_No_?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123_Abc456_Def");

            let result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123_Abc456_Def");

            let result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = ada_case_with_options("", &opts);
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "_";
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc___Def___Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_-_Def_-_Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_-_Def_-_Ghi");
        }

        #[test]
        fn convert_title_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = " ";
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_ _Def_ _Ghi");
        }

        #[test]
        fn convert_ada_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "_";
            let result = ada_case_with_options("Abc_Def_Ghi", &opts);
            assert_eq!(result, "Abc___Def___Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "_";
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc___Def___Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_-_Def_-_Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc_123_456_Def_G_89_Hi_Jkl_Mn_12");

            opts.keep = "-";
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc_123-456_Def_G_89_Hi_Jkl_Mn_12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: ".~!#%?",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "._Abc_~!_Def_#_Ghi_%_Jk_Lm_No_?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123_Abc_456_Def");

            let result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123_Abc_456_Def");

            let result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc_456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = ada_case_with_options("", &opts);
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
            let result = ada_case_with_options("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = ada_case_with_options("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc_Def_Gh_Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let result = ada_case_with_options("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "_";
            let result = ada_case_with_options("abc_def_ghi", &opts);
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
            let result = ada_case_with_options("abc-def-ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            let result = ada_case_with_options("abc-def-ghi", &opts);
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
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-_Def-_Ghi");
        }

        #[test]
        fn convert_title_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = " ";
            let result = ada_case_with_options("Abc Def Ghi", &opts);
            assert_eq!(result, "Abc _Def _Ghi");
        }

        #[test]
        fn convert_ada_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            let result = ada_case_with_options("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-_Def-_Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "_";
            let result = ada_case_with_options("ABC_DEF_GHI", &opts);
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
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc_Def_Ghi");

            opts.keep = "-";
            let result = ada_case_with_options("ABC-DEF-GHI", &opts);
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
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123_456def_G89hi_Jkl_Mn12");

            opts.keep = "-";
            let result = ada_case_with_options("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456def_G89hi_Jkl_Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: ".~!#%?",
            };
            let result = ada_case_with_options(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc~!_Def#_Ghi%_Jk_Lm_No_?");
        }

        #[test]
        fn convert_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = ada_case_with_options("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = ada_case_with_options("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");

            let result = ada_case_with_options("123Abc456Def", &opts);
            assert_eq!(result, "123_Abc456_Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = ada_case_with_options("", &opts);
            assert_eq!(result, "");
        }
    }
}
