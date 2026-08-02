// Copyright (C) 2026 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::options::Options;

/// A generic function that converts string cases into an uppercased format joined by a specified
/// joiner character.
///
/// It processes the input string `input`, identifies word boundaries based on character casing and
/// non-alphabetic character rules defined in `opts`, converts alphabetic characters to uppercase,
/// and joins the words using the const generic character `JOINER`.
///
/// It operates by iterating character by character through `input` using an internal state
/// machine. It handles ASCII uppercase, ASCII lowercase, and non-alphabetic characters (digits and
/// symbols) according to options such as `opts.separators`, `opts.keep`,
/// `opts.separate_before_non_alphabets`, and `opts.separate_after_non_alphabets` to determine
/// word boundaries, upperizing characters, and insert the `JOINER` character.
/// If a character is specified in both `opts.separators` and `opts.keep`, the character in
/// `opts.separators` takes precedence and the character in `opts.keep` is ignored.
///
/// # Parameters
///
/// - `JOINER`: A const generic `char` used as the delimiter between uppercased words.
/// - `input`: The target string slice (`&str`) to be uppercased.
/// - `opts`: A reference to [`Options`] defining separator rules, retained characters, and boundary
///   behaviors.
///
/// # Returns
///
/// - Returns a [`String`] with all words uppercased and joined by `JOINER`.
///   Returns an empty [`String`] if `input` is empty.
///
/// # Examples
///
/// ```rust
/// use stringcase::{upperize, Options};
///
/// let opts = Options {
///     separate_before_non_alphabets: true,
///     separate_after_non_alphabets: true,
///     separators: "",
///     keep: "",
/// };
/// let result = upperize::<'.' >("foo_bar_100_baz", &opts);
/// assert_eq!(result, "FOO.BAR.100.BAZ");
/// ```
pub fn upperize<const JOINER: char>(input: &str, opts: &Options) -> String {
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
                result.push(ch);
                flag = ChIs::NextOfContdUpper;
            } else {
                result.push(JOINER);
                result.push(ch);
                flag = ChIs::NextOfUpper;
            }
        } else if ch.is_ascii_lowercase() {
            if flag == ChIs::NextOfContdUpper {
                if let Some(prev) = result.pop() {
                    result.push(JOINER);
                    result.push(prev);
                    result.push(ch.to_ascii_uppercase());
                }
            } else if flag == ChIs::NextOfSepMark
                || (opts.separate_after_non_alphabets && flag == ChIs::NextOfKeptMark)
            {
                result.push(JOINER);
                result.push(ch.to_ascii_uppercase());
            } else {
                result.push(ch.to_ascii_uppercase());
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
                        result.push(JOINER);
                        result.push(ch);
                    }
                } else {
                    if flag != ChIs::NextOfSepMark {
                        result.push(ch);
                    } else {
                        result.push(JOINER);
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
mod tests_of_upperize {
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
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_upperize() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_with_digits() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC.123.456DEF.G.89HI.JKL.MN.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "ABC.DEF.GHI.JK.LM.NO");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123ABC.456DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC.456DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC.456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("", &opts);
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
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_upperize() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_with_digits() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123.456.DEF.G89.HI.JKL.MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "ABC.DEF.GHI.JK.LM.NO");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.ABC456.DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.ABC456.DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("", &opts);
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
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_upperize() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_with_digits() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC.123.456.DEF.G.89.HI.JKL.MN.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "ABC.DEF.GHI.JK.LM.NO");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.ABC.456.DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.ABC.456.DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC.456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("", &opts);
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
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_upperize() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_with_digits() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123.456DEF.G89HI.JKL.MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "ABC.DEF.GHI.JK.LM.NO");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };
            let result = upperize::<'.'>("", &opts);
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
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "-";
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC._DEF._GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.-DEF.-GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.-.DEF.-.GHI");
        }

        #[test]
        fn convert_upperize() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "-";
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC._DEF._GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.-DEF.-GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC.123.456DEF.G.89HI.JKL.MN.12");

            opts.separators = "_";
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC.123-456DEF.G.89HI.JKL.MN.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".ABC.~!.DEF.#.GHI.%.JK.LM.NO.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123ABC.456DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC.456DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC.456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("", &opts);
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
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "-";
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC_.DEF_.GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-.DEF-.GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-.DEF-.GHI");
        }

        #[test]
        fn convert_upperize() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "-";
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC_.DEF_.GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-.DEF-.GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123.456.DEF.G89.HI.JKL.MN12");

            opts.separators = "_";
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456.DEF.G89.HI.JKL.MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..ABC~!.DEF#.GHI%.JK.LM.NO.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.ABC456.DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.ABC456.DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("", &opts);
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
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "-";
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC._.DEF._.GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.-.DEF.-.GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.-.DEF.-.GHI");
        }

        #[test]
        fn convert_upperize() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "-";
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC._.DEF._.GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.-.DEF.-.GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC.123.456.DEF.G.89.HI.JKL.MN.12");

            opts.separators = "_";
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC.123-456.DEF.G.89.HI.JKL.MN.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..ABC.~!.DEF.#.GHI.%.JK.LM.NO.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.ABC.456.DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.ABC.456.DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC.456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("", &opts);
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
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "-";
            let result = upperize::<'.'>("abc_def_ghi", &opts);
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
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("abc-def-ghi", &opts);
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
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-.DEF-.GHI");
        }

        #[test]
        fn convert_upperize() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "-";
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
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
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.separators = "_";
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
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
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123.456DEF.G89HI.JKL.MN12");

            opts.separators = "_";
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456DEF.G89HI.JKL.MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".ABC~!.DEF#.GHI%.JK.LM.NO.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };
            let result = upperize::<'.'>("", &opts);
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
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC._DEF._GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.-DEF.-GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.-.DEF.-.GHI");
        }

        #[test]
        fn convert_upperize() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "_";
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC._DEF._GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.-DEF.-GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC.123.456DEF.G.89HI.JKL.MN.12");

            opts.keep = "-";
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC.123-456DEF.G.89HI.JKL.MN.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: ".~!#%?",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".ABC.~!.DEF.#.GHI.%.JK.LM.NO.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123ABC.456DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC.456DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC.456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("", &opts);
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
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "_";
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC_.DEF_.GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-.DEF-.GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-.DEF-.GHI");
        }

        #[test]
        fn convert_upperize() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "_";
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC_.DEF_.GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-.DEF-.GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123.456.DEF.G89.HI.JKL.MN12");

            opts.keep = "-";
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456.DEF.G89.HI.JKL.MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: ".~!#%?",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..ABC~!.DEF#.GHI%.JK.LM.NO.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.ABC456.DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.ABC456.DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("", &opts);
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
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "_";
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC._.DEF._.GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.-.DEF.-.GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.-.DEF.-.GHI");
        }

        #[test]
        fn convert_upperize() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "_";
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC._.DEF._.GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.-.DEF.-.GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC.123.456.DEF.G.89.HI.JKL.MN.12");

            opts.keep = "-";
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC.123-456.DEF.G.89.HI.JKL.MN.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: ".~!#%?",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..ABC.~!.DEF.#.GHI.%.JK.LM.NO.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.ABC.456.DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.ABC.456.DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC.456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("", &opts);
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
            let result = upperize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "ABC.DEF.GH.IJK");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "_";
            let result = upperize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "ABC_DEF_GHI");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "ABC-.DEF-.GHI");
        }

        #[test]
        fn convert_upperize() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "_";
            let result = upperize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "ABC_DEF_GHI");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC.DEF.GHI");

            opts.keep = "-";
            let result = upperize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "ABC-DEF-GHI");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123.456DEF.G89HI.JKL.MN12");

            opts.keep = "-";
            let result = upperize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "ABC123-456DEF.G89HI.JKL.MN12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: ".~!#%?",
            };
            let result = upperize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".ABC~!.DEF#.GHI%.JK.LM.NO.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = upperize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123ABC456DEF");

            let result = upperize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.ABC456.DEF");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };
            let result = upperize::<'.'>("", &opts);
            assert_eq!(result, "");
        }
    }
}
