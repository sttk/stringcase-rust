// Copyright (C) 2026 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::options::Options;

/// A generic function that converts string cases into a lowercased format joined by a specified
/// joiner character.
///
/// It processes the input string `input`, identifies word boundaries based on character casing and
/// non-alphabetic character rules defined in `opts`, converts alphabetic characters to lowercase,
/// and joins the words using the const generic character `JOINER`.
///
/// It operates by iterating character by character through `input` using an internal state
/// machine. It handles ASCII uppercase, ASCII lowercase, and non-alphabetic characters (digits and
/// symbols) according to options such as `opts.separators`, `opts.keep`,
/// `opts.separate_before_non_alphabets`, and `opts.separate_after_non_alphabets` to determine
/// word boundaries, lowerizing characters, and insert the `JOINER` character.
/// If a character is specified in both `opts.separators` and `opts.keep`, the character in
/// `opts.separators` takes precedence and the character in `opts.keep` is ignored.
///
/// # Parameters
///
/// - `JOINER`: A const generic `char` used as the delimiter between lowercased words.
/// - `input`: The target string slice (`&str`) to be lowercased.
/// - `opts`: A reference to [`Options`] defining separator rules, retained characters, and boundary
///   behaviors.
///
/// # Returns
///
/// - Returns a [`String`] with all words lowercased and joined by `JOINER`.
///   Returns an empty [`String`] if `input` is empty.
///
/// # Examples
///
/// ```rust
/// use stringcase::{lowerize, Options};
///
/// let opts = Options {
///     separate_before_non_alphabets: true,
///     separate_after_non_alphabets: true,
///     separators: "",
///     keep: "",
/// };
/// let result = lowerize::<'.' >("foo_bar_100_baz", &opts);
/// assert_eq!(result, "foo.bar.100.baz");
/// ```
pub fn lowerize<const JOINER: char>(input: &str, opts: &Options) -> String {
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
                result.push(ch.to_ascii_lowercase());
                flag = ChIs::NextOfUpper;
            } else if flag == ChIs::NextOfUpper
                || flag == ChIs::NextOfContdUpper
                || (!opts.separate_after_non_alphabets && flag == ChIs::NextOfKeptMark)
            {
                result.push(ch.to_ascii_lowercase());
                flag = ChIs::NextOfContdUpper;
            } else {
                result.push(JOINER);
                result.push(ch.to_ascii_lowercase());
                flag = ChIs::NextOfUpper;
            }
        } else if ch.is_ascii_lowercase() {
            if flag == ChIs::NextOfContdUpper {
                if let Some(prev) = result.pop() {
                    result.push(JOINER);
                    result.push(prev);
                    result.push(ch);
                }
            } else if flag == ChIs::NextOfSepMark
                || (opts.separate_after_non_alphabets && flag == ChIs::NextOfKeptMark)
            {
                result.push(JOINER);
                result.push(ch);
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
mod tests_of_lowerize {
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc.123.456def.g.89hi.jkl.mn.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "abc.def.ghi.jk.lm.no");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc.456def");

            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc.456def");

            let result = lowerize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.abc.456.def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("", &opts);
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc123.456.def.g89.hi.jkl.mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "abc.def.ghi.jk.lm.no");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.abc456.def");

            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.abc456.def");

            let result = lowerize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.abc456.def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("", &opts);
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc.123.456.def.g.89.hi.jkl.mn.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "abc.def.ghi.jk.lm.no");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.abc.456.def");

            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.abc.456.def");

            let result = lowerize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.abc.456.def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("", &opts);
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_train_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc123.456def.g89hi.jkl.mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "abc.def.ghi.jk.lm.no");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");

            let result = lowerize::<'.'>("123Abc456Def", &opts);
            assert_eq!(result, "123.abc456.def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "",
            };

            let result = lowerize::<'.'>("", &opts);
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "-";
            result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc._def._ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.-def.-ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.-.def.-.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };

            let mut result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "-";
            result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc._def._ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.-def.-ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc.123.456def.g.89hi.jkl.mn.12");

            opts.separators = "_";
            result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc.123-456def.g.89hi.jkl.mn.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc.~!.def.#.ghi.%.jk.lm.no.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc.456def");

            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc.456def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };

            let result = lowerize::<'.'>("", &opts);
            assert_eq!(result, "");
        }

        #[test]
        fn alphabets_and_numbers_in_separators_are_no_effect() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "-b2",
                keep: "",
            };

            let result = lowerize::<'.'>("abc123def", &opts);
            assert_eq!(result, "abc.123def");
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "-";
            result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc_.def_.ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc-.def-.ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc-.def-.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };

            let mut result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "-";
            result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc_.def_.ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc-.def-.ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc123.456.def.g89.hi.jkl.mn12");

            opts.separators = "_";
            result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc123-456.def.g89.hi.jkl.mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..abc~!.def#.ghi%.jk.lm.no.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.abc456.def");

            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.abc456.def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };

            let result = lowerize::<'.'>("", &opts);
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

            let result = lowerize::<'.'>("abc123def", &opts);
            assert_eq!(result, "abc123.def");
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "-";
            result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc._.def._.ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.-.def.-.ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.-.def.-.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "_",
                keep: "",
            };

            let mut result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "-";
            result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc._.def._.ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.-.def.-.ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc.123.456.def.g.89.hi.jkl.mn.12");

            opts.separators = "_";
            result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc.123-456.def.g.89.hi.jkl.mn.12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: ":@$&()/",
                keep: "",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..abc.~!.def.#.ghi.%.jk.lm.no.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-",
                keep: "",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.abc.456.def");

            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.abc.456.def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-_",
                keep: "",
            };

            let result = lowerize::<'.'>("", &opts);
            assert_eq!(result, "");
        }

        #[test]
        fn alphabets_and_numbers_in_separators_are_no_effect() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "-b2",
                keep: "",
            };

            let result = lowerize::<'.'>("abc123def", &opts);
            assert_eq!(result, "abc.123.def");
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "-";
            result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc_def_ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc-def-ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc-.def-.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "_",
                keep: "",
            };

            let mut result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "-";
            result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc_def_ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.separators = "_";
            result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc-def-ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };

            let mut result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc123.456def.g89hi.jkl.mn12");

            opts.separators = "_";
            result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc123-456def.g89hi.jkl.mn12");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: ":@$&()/",
                keep: "",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc~!.def#.ghi%.jk.lm.no.?");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-",
                keep: "",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-_",
                keep: "",
            };

            let result = lowerize::<'.'>("", &opts);
            assert_eq!(result, "");
        }

        #[test]
        fn alphabets_and_numbers_in_separators_are_no_effect() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "-b2",
                keep: "",
            };

            let result = lowerize::<'.'>("abc123def", &opts);
            assert_eq!(result, "abc123def");
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };

            let mut result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "_";
            result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc._def._ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.-def.-ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.-.def.-.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };

            let mut result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "_";
            result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc._def._ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.-def.-ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc.123.456def.g.89hi.jkl.mn.12");

            opts.keep = "-";
            result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc.123-456def.g.89hi.jkl.mn.12");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc.456def");

            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc.456def");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: ".~!#%?",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc.~!.def.#.ghi.%.jk.lm.no.?");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };

            let result = lowerize::<'.'>("", &opts);
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };

            let mut result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "_";
            result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc_.def_.ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc-.def-.ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc-.def-.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };

            let mut result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "_";
            result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc_.def_.ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc-.def-.ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc123.456.def.g89.hi.jkl.mn12");

            opts.keep = "-";
            result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc123-456.def.g89.hi.jkl.mn12");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.abc456.def");

            opts.keep = "_";
            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.abc456.def");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: ".~!#%?",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..abc~!.def#.ghi%.jk.lm.no.?");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };

            let result = lowerize::<'.'>("", &opts);
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };

            let mut result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "_";
            result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc._.def._.ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.-.def.-.ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.-.def.-.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };

            let mut result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "_";
            result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc._.def._.ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.-.def.-.ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc.123.456.def.g.89.hi.jkl.mn.12");

            opts.keep = "-";
            result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc.123-456.def.g.89.hi.jkl.mn.12");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123.abc.456.def");

            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123.abc.456.def");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: ".~!#%?",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, "..abc.~!.def.#.ghi.%.jk.lm.no.?");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: true,
                separate_after_non_alphabets: true,
                separators: "",
                keep: "-_",
            };

            let result = lowerize::<'.'>("", &opts);
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

            let result = lowerize::<'.'>("abcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_pascal_case() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };

            let result = lowerize::<'.'>("AbcDefGHIjk", &opts);
            assert_eq!(result, "abc.def.gh.ijk");
        }

        #[test]
        fn convert_snake_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };

            let mut result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "_";
            result = lowerize::<'.'>("abc_def_ghi", &opts);
            assert_eq!(result, "abc_def_ghi");
        }

        #[test]
        fn convert_kebab_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("abc-def-ghi", &opts);
            assert_eq!(result, "abc-def-ghi");
        }

        #[test]
        fn convert_train_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("Abc-Def-Ghi", &opts);
            assert_eq!(result, "abc-.def-.ghi");
        }

        #[test]
        fn convert_macro_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };

            let mut result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "_";
            result = lowerize::<'.'>("ABC_DEF_GHI", &opts);
            assert_eq!(result, "abc_def_ghi");
        }

        #[test]
        fn convert_cobol_case() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc.def.ghi");

            opts.keep = "-";
            result = lowerize::<'.'>("ABC-DEF-GHI", &opts);
            assert_eq!(result, "abc-def-ghi");
        }

        #[test]
        fn convert_with_keeping_digits() {
            let mut opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "_",
            };

            let mut result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc123.456def.g89hi.jkl.mn12");

            opts.keep = "-";
            result = lowerize::<'.'>("abc123-456defG89HIJklMN12", &opts);
            assert_eq!(result, "abc123-456def.g89hi.jkl.mn12");
        }

        #[test]
        fn convert_when_starting_with_digit() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-",
            };

            let result = lowerize::<'.'>("123abc456def", &opts);
            assert_eq!(result, "123abc456def");

            let result = lowerize::<'.'>("123ABC456DEF", &opts);
            assert_eq!(result, "123abc456def");
        }

        #[test]
        fn convert_with_symbols_as_separators() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: ".~!#%?",
            };

            let result = lowerize::<'.'>(":.abc~!@def#$ghi%&jk(lm)no/?", &opts);
            assert_eq!(result, ".abc~!.def#.ghi%.jk.lm.no.?");
        }

        #[test]
        fn convert_empty_string() {
            let opts = Options {
                separate_before_non_alphabets: false,
                separate_after_non_alphabets: false,
                separators: "",
                keep: "-_",
            };

            let result = lowerize::<'.'>("", &opts);
            assert_eq!(result, "");
        }
    }
}
