// Copyright (C) 2026 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::options::Options;

/// A generic function that converts string cases into a capitalized format joined by a specified
/// concatenator character.
///
/// It processes the input string `input`, identifies word boundaries based on character casing and
/// non-alphabetic character rules defined in `opts`, capitalizes the head character of each word,
/// lowercases subsequent letters, and joins the words using the const generic character
/// `CONCATENATOR`.
///
/// It operates by iterating character by character through `input` using an internal state
/// machine. It handles ASCII uppercase, ASCII lowercase, and non-alphabetic characters (digits and
/// symbols) according to options such as `opts.separators`, `opts.keep`,
/// `opts.separate_before_non_alphabets`, and `opts.separate_after_non_alphabets` to determine
/// word boundaries, capitalize initial letters, and insert the `CONCATENATOR` character.
/// If a character is specified in both `opts.separators` and `opts.keep`, the character in
/// `opts.separators` takes precedence and the character in `opts.keep` is ignored.
///
/// # Parameters
///
/// - `CONCATENATOR`: A const generic `char` used as the delimiter between capitalized words.
/// - `input`: The target string slice (`&str`) to be capitalized.
/// - `opts`: A reference to [`Options`] defining separator rules, retained characters, and boundary
///   behaviors.
///
/// # Returns
///
/// - Returns a [`String`] with all words capitalized and joined by `CONCATENATOR`.
///   Returns an empty [`String`] if `input` is empty.
///
/// # Examples
///
/// ```rust
/// use stringcase::{capitalize, Options};
///
/// let opts = Options {
///     separate_before_non_alphabets: true,
///     separate_after_non_alphabets: true,
///     separators: "",
///     keep: "",
/// };
/// let result = capitalize::<'.'>("foo_bar_100_baz", &opts);
/// assert_eq!(result, "Foo.Bar.100.Baz");
/// ```
pub fn capitalize<const CONCATENATOR: char>(input: &str, opts: &Options) -> String {
    let mut result = String::with_capacity(input.len() + input.len() / 2);
    // .len returns byte count but ok in this case!

    #[derive(PartialEq)]
    enum ChIs {
        FirstOfStr,
        NextOfUpper,
        NextOfContdUpper,
        NextOfSepMark,
        NextOfKeptMark,
        Other,
    }

    let mut flag = ChIs::FirstOfStr;

    for ch in input.chars() {
        if ch.is_ascii_uppercase() {
            if flag == ChIs::FirstOfStr {
                result.push(ch);
                flag = ChIs::NextOfUpper;
            } else if flag == ChIs::NextOfUpper
                || flag == ChIs::NextOfContdUpper
                || (!opts.separate_after_non_alphabets && flag == ChIs::NextOfKeptMark)
            {
                result.push(ch.to_ascii_lowercase());
                flag = ChIs::NextOfContdUpper;
            } else {
                result.push(CONCATENATOR);
                result.push(ch);
                flag = ChIs::NextOfUpper;
            }
        } else if ch.is_ascii_lowercase() {
            if flag == ChIs::FirstOfStr {
                result.push(ch.to_ascii_uppercase());
            } else if flag == ChIs::NextOfContdUpper {
                if let Some(prev) = result.pop() {
                    result.push(CONCATENATOR);
                    result.push(prev.to_ascii_uppercase());
                    result.push(ch);
                }
            } else if flag == ChIs::NextOfSepMark
                || (opts.separate_after_non_alphabets && flag == ChIs::NextOfKeptMark)
            {
                result.push(CONCATENATOR);
                result.push(ch.to_ascii_uppercase());
            } else {
                result.push(ch);
            }
            flag = ChIs::Other;
        } else {
            let mut is_kept_char = false;
            if ch.is_ascii_digit() {
                is_kept_char = true;
            } else if !opts.separators.is_empty() {
                if !opts.separators.contains(ch) {
                    is_kept_char = true;
                }
            } else if !opts.keep.is_empty() {
                #[allow(clippy::collapsible_if)]
                if opts.keep.contains(ch) {
                    is_kept_char = true;
                }
            }

            if is_kept_char {
                if opts.separate_before_non_alphabets {
                    if flag == ChIs::FirstOfStr || flag == ChIs::NextOfKeptMark {
                        result.push(ch);
                    } else {
                        result.push(CONCATENATOR);
                        result.push(ch);
                    }
                } else {
                    if flag != ChIs::NextOfSepMark {
                        result.push(ch);
                    } else {
                        result.push(CONCATENATOR);
                        result.push(ch);
                    }
                }
                flag = ChIs::NextOfKeptMark;
            } else {
                if flag != ChIs::FirstOfStr {
                    flag = ChIs::NextOfSepMark;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests_of_capitalize {
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_capitalize() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc.123.456def.G.89hi.Jkl.Mn.12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc.Def.Ghi.Jk.Lm.No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc.456def");

            let result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc.456def");

            let result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc.456.Def");
        }

        #[test]
        fn convert_empty_string_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("", &opts);
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_capitalize() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123.456.Def.G89.Hi.Jkl.Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc.Def.Ghi.Jk.Lm.No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.Abc456.Def");

            let result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.Abc456.Def");

            let result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc456.Def");
        }

        #[test]
        fn convert_empty_string_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("", &opts);
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_capitalize() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc.123.456.Def.G.89.Hi.Jkl.Mn.12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc.Def.Ghi.Jk.Lm.No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.Abc.456.Def");

            let result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.Abc.456.Def");

            let result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc.456.Def");
        }

        #[test]
        fn convert_empty_string_with_options() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("", &opts);
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_kebab_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_capitalize() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_macro_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_cobol_case_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");
        }

        #[test]
        fn convert_with_keeping_digits_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123.456def.G89hi.Jkl.Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "Abc.Def.Ghi.Jk.Lm.No");
        }

        #[test]
        fn convert_when_starting_with_digit_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");

            let result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc456.Def");
        }

        #[test]
        fn convert_empty_string_with_options() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = capitalize::<'.'>("", &opts);
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "-";
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc._def._ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "_";
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.-def.-ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "_";
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.-.Def.-.Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "-";
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc._def._ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "_";
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.-def.-ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc.123.456def.G.89hi.Jkl.Mn.12");

            opts.separators = "_";
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc.123-456def.G.89hi.Jkl.Mn.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc.~!.Def.#.Ghi.%.Jk.Lm.No.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc.456def");

            let result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc.456def");

            let result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc.456.Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = capitalize::<'.'>("", &opts);
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
            let result = capitalize::<'.'>("abc123def", &opts);
            assert_eq!(result, "Abc.123def");
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "-";
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_.Def_.Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "_";
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-.Def-.Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "_";
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-.Def-.Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "-";
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_.Def_.Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "_";
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-.Def-.Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123.456.Def.G89.Hi.Jkl.Mn12");

            opts.separators = "_";
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456.Def.G89.Hi.Jkl.Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..Abc~!.Def#.Ghi%.Jk.Lm.No.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.Abc456.Def");

            let result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.Abc456.Def");

            let result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc456.Def");
        }

        #[test]
        fn convert_an_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = capitalize::<'.'>("", &opts);
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
            let result = capitalize::<'.'>("abc123def", &opts);
            assert_eq!(result, "Abc123.Def");
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "-";
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc._.Def._.Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "_";
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.-.Def.-.Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "_";
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.-.Def.-.Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "-";
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc._.Def._.Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "_";
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.-.Def.-.Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc.123.456.Def.G.89.Hi.Jkl.Mn.12");

            opts.separators = "_";
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc.123-456.Def.G.89.Hi.Jkl.Mn.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..Abc.~!.Def.#.Ghi.%.Jk.Lm.No.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.Abc.456.Def");

            let result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.Abc.456.Def");

            let result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc.456.Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = capitalize::<'.'>("", &opts);
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
            let result = capitalize::<'.'>("abc123def", &opts);
            assert_eq!(result, "Abc.123.Def");
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "-";
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
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
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "_";
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
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
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "_";
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-.Def-.Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.separators = "-";
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
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
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123.456def.G89hi.Jkl.Mn12");

            opts.separators = "_";
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456def.G89hi.Jkl.Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc~!.Def#.Ghi%.Jk.Lm.No.?");
        }

        #[test]
        fn convert_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");

            let result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc456.Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = capitalize::<'.'>("", &opts);
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
            let result = capitalize::<'.'>("abc123def", &opts);
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let mut result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "_";
            result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc._def._ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.-def.-ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.-.Def.-.Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let mut result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "_";
            result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc._def._ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.-def.-ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let mut result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc.123.456def.G.89hi.Jkl.Mn.12");

            opts.keep = "-";
            result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc.123-456def.G.89hi.Jkl.Mn.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: ".~!#%?",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc.~!.Def.#.Ghi.%.Jk.Lm.No.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let mut result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc.456def");

            result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc.456def");

            result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc.456.Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = capitalize::<'.'>("", &opts);
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
            let result = capitalize::<'.'>("abc123def", &opts);
            assert_eq!(result, "Abc.123def");
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "_";
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc_.Def_.Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc-.Def-.Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-.Def-.Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "_";
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc_.Def_.Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc-.Def-.Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123.456.Def.G89.Hi.Jkl.Mn12");

            opts.keep = "-";
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456.Def.G89.Hi.Jkl.Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: ".~!#%?",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..Abc~!.Def#.Ghi%.Jk.Lm.No.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.Abc456.Def");

            let result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.Abc456.Def");

            let result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc456.Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = capitalize::<'.'>("", &opts);
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "_";
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc._.Def._.Ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.-.Def.-.Ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.-.Def.-.Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "_";
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc._.Def._.Ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.-.Def.-.Ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc.123.456.Def.G.89.Hi.Jkl.Mn.12");

            opts.keep = "-";
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc.123-456.Def.G.89.Hi.Jkl.Mn.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: ".~!#%?",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..Abc.~!.Def.#.Ghi.%.Jk.Lm.No.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.Abc.456.Def");

            let result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.Abc.456.Def");

            let result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc.456.Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = capitalize::<'.'>("", &opts);
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
            let result = capitalize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = capitalize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "Abc.Def.Gh.Ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "_";
            let result = capitalize::<'.'>("abc_def_ghi", &opts);
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
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            let result = capitalize::<'.'>("abc-def-ghi", &opts);
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
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            let result = capitalize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "Abc-.Def-.Ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "_";
            let result = capitalize::<'.'>("ABC_DEF_GHI", &opts);
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
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "Abc.Def.Ghi");

            opts.keep = "-";
            let result = capitalize::<'.'>("ABC-DEF-GHI", &opts);
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
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123.456def.G89hi.Jkl.Mn12");

            opts.keep = "-";
            let result = capitalize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "Abc123-456def.G89hi.Jkl.Mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: ".~!#%?",
            };
            let result = capitalize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc~!.Def#.Ghi%.Jk.Lm.No.?");
        }

        #[test]
        fn convert_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = capitalize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = capitalize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");

            let result = capitalize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.Abc456.Def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = capitalize::<'.'>("", &opts);
            assert_eq!(result, "");
        }
    }
}
