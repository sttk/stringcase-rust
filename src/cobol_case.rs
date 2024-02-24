// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

/// Converts a string to cobol case.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is cobol case.
///
/// This function targets the upper and lower cases of ASCII alphabets for
/// capitalization, and all characters except ASCII alphabets and ASCII numbers
/// are replaced to hyphens as word separators.
///
/// ```rust
///     let cobol = stringcase::cobol_case("foo_bar_baz");
///     assert_eq!(cobol, "FOO-BAR-BAZ");
/// ```
pub fn cobol_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len() + input.len() / 2);
    // .len returns byte count but ok in this case!

    enum ChIs {
      FirstOfStr,
      NextOfUpper,
      NextOfContdUpper,
      NextOfMark,
      Others,
    }
    let mut flag = ChIs::FirstOfStr;

    for ch in input.chars() {
        if ch.is_ascii_uppercase() {
            match flag {
                ChIs::FirstOfStr => {
                    result.push(ch);
                    flag = ChIs::NextOfUpper;
                },
                ChIs::NextOfUpper | ChIs::NextOfContdUpper => {
                    result.push(ch);
                    flag = ChIs::NextOfContdUpper;
                },
                _ => {
                    result.push('-');
                    result.push(ch);
                    flag = ChIs::NextOfUpper;
                }
            }
        } else if ch.is_ascii_lowercase() {
            match flag {
                ChIs::NextOfContdUpper => match result.pop() {
                    Some(prev) => {
                        result.push('-');
                        result.push(prev);
                    }
                    None => (), // impossible
                },
                ChIs::NextOfMark => result.push('-'),
                _ => (),
            }
            result.push(ch.to_ascii_uppercase());
            flag = ChIs::Others;
        } else if ch.is_ascii_digit() {
            match flag {
              ChIs::NextOfMark => result.push('-'),
              _ => (),
            }
            result.push(ch);
            flag = ChIs::Others;
        } else {
            match flag {
              ChIs::FirstOfStr => (),
              _ => flag = ChIs::NextOfMark,
            }
        }
    }

    result
}

/// Converts a string to cobol case using the specified characters as
/// separators.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is cobol case.
///
/// This function targets only the upper and lower cases of ASCII alphabets for
/// capitalization, and the characters specified as the second argument of this
/// function are regarded as word separators and are replaced to hyphens.
///
/// ```rust
///     let cobol = stringcase::cobol_case_with_sep("foo-bar100%baz", "- ");
///     assert_eq!(cobol, "FOO-BAR100%-BAZ");
/// ```
pub fn cobol_case_with_sep(input: &str, seps: &str) -> String {
    let mut result = String::with_capacity(input.len() + input.len() / 2);
    // .len returns byte count but ok in this case!

    enum ChIs {
      FirstOfStr,
      NextOfUpper,
      NextOfContdUpper,
      NextOfSepMark,
      NextOfKeepedMark,
      Others,
    }
    let mut flag = ChIs::FirstOfStr;

    for ch in input.chars() {
        if seps.contains(ch) {
            match flag {
                ChIs::FirstOfStr => (),
                _ => flag = ChIs::NextOfSepMark,
            }
        } else if ch.is_ascii_uppercase() {
            match flag {
                ChIs::FirstOfStr => {
                    result.push(ch);
                    flag = ChIs::NextOfUpper;
                },
                ChIs::NextOfUpper | ChIs::NextOfContdUpper => {
                    result.push(ch);
                    flag = ChIs::NextOfContdUpper;
                },
                _ => {
                    result.push('-');
                    result.push(ch);
                    flag = ChIs::NextOfUpper;
                }
            }
        } else if ch.is_ascii_lowercase() {
            match flag {
                ChIs::NextOfContdUpper => match result.pop() {
                    Some(prev) => {
                        result.push('-');
                        result.push(prev);
                    }
                    None => (), // impossible
                },
                ChIs::NextOfSepMark | ChIs::NextOfKeepedMark => {
                    result.push('-');
                },
                _ => (),
            }
            result.push(ch.to_ascii_uppercase());
            flag = ChIs::Others;
        } else if ch.is_ascii_digit() {
            match flag {
                ChIs::NextOfSepMark | ChIs::NextOfKeepedMark => {
                    result.push('-');
                },
                _ => (),
            }
            result.push(ch);
            flag = ChIs::Others;
        } else {
            match flag {
                ChIs::NextOfSepMark => result.push('-'),
                _ => (),
            }
            result.push(ch);
            flag = ChIs::NextOfKeepedMark;
        }
    }

    result
}

/// Converts a string to cobol case using characters other than the specified
/// characters as separators.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is cobol case.
///
/// This function targets only the upper and lower cases of ASCII alphabets for
/// capitalization, and the characters other than the specified characters as
/// the second argument of this function are regarded as word separators and
/// are replaced to hyphens.
///
/// ```rust
///     let cobol = stringcase::cobol_case_with_keep("foo-bar100%baz", "%");
///     assert_eq!(cobol, "FOO-BAR100%-BAZ");
/// ```
pub fn cobol_case_with_keep(input: &str, keeped: &str) -> String {
    let mut result = String::with_capacity(input.len() + input.len() / 2);
    // .len returns byte count but ok in this case!

    enum ChIs {
      FirstOfStr,
      NextOfUpper,
      NextOfContdUpper,
      NextOfSepMark,
      NextOfKeepedMark,
      Others,
    }
    let mut flag = ChIs::FirstOfStr;

    for ch in input.chars() {
        if ch.is_ascii_uppercase() {
            match flag {
                ChIs::FirstOfStr => {
                    result.push(ch);
                    flag = ChIs::NextOfUpper;
                },
                ChIs::NextOfUpper | ChIs::NextOfContdUpper => {
                    result.push(ch);
                    flag = ChIs::NextOfContdUpper;
                },
                _ => {
                    result.push('-');
                    result.push(ch);
                    flag = ChIs::NextOfUpper;
                },
            }
        } else if ch.is_ascii_lowercase() {
            match flag {
                ChIs::NextOfContdUpper => match result.pop() {
                    Some(prev) => {
                        result.push('-');
                        result.push(prev);
                    }
                    None => (), // impossible
                },
                ChIs::NextOfSepMark | ChIs::NextOfKeepedMark  => {
                    result.push('-');
                },
                _ => (),
            }
            result.push(ch.to_ascii_uppercase());
            flag = ChIs::Others;
        } else if ch.is_ascii_digit() {
            match flag {
                ChIs::NextOfSepMark | ChIs::NextOfKeepedMark => {
                    result.push('-');
                },
                _ => (),
            }
            result.push(ch);
            flag = ChIs::Others;
        } else if keeped.contains(ch) {
            match flag {
                ChIs::NextOfSepMark => result.push('-'),
                _ => (),
            }
            result.push(ch);
            flag = ChIs::NextOfKeepedMark;
        } else {
            match flag {
                ChIs::FirstOfStr => (),
                _ => flag = ChIs::NextOfSepMark,
            }
        }
    }

    result
}

#[cfg(test)]
mod tests_of_cobol_case {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = cobol_case("abcDefGHIjk");
        assert_eq!(result, "ABC-DEF-GH-IJK");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = cobol_case("AbcDefGHIjk");
        assert_eq!(result, "ABC-DEF-GH-IJK");
    }

    #[test]
    fn it_should_convert_snake_case() {
        let result = cobol_case("abc_def_ghi");
        assert_eq!(result, "ABC-DEF-GHI");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = cobol_case("abc-def-ghi");
        assert_eq!(result, "ABC-DEF-GHI");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = cobol_case("Abc-Def-Ghi");
        assert_eq!(result, "ABC-DEF-GHI");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = cobol_case("ABC_DEF_GHI");
        assert_eq!(result, "ABC-DEF-GHI");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = cobol_case("ABC-DEF-GHI");
        assert_eq!(result, "ABC-DEF-GHI");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = cobol_case("abc123-456defG789HIJklMN12");
        assert_eq!(result, "ABC123-456DEF-G789-HI-JKL-MN12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = cobol_case(":.abc~!@def#$ghi%&jk(lm)no/?");
        assert_eq!(result, "ABC-DEF-GHI-JK-LM-NO");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = cobol_case("");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_cobol_case_with_sep {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = cobol_case_with_sep("abcDefGHIjk", "-_");
        assert_eq!(result, "ABC-DEF-GH-IJK");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = cobol_case_with_sep("AbcDefGHIjk", "-_");
        assert_eq!(result, "ABC-DEF-GH-IJK");
    }

    #[test]
    fn it_should_convert_snake_case() {
        let result = cobol_case_with_sep("abc_def_ghi", "_");
        assert_eq!(result, "ABC-DEF-GHI");

        let result = cobol_case_with_sep("abc_def_ghi", "-");
        assert_eq!(result, "ABC_-DEF_-GHI");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = cobol_case_with_sep("abc-def-ghi", "-");
        assert_eq!(result, "ABC-DEF-GHI");

        let result = cobol_case_with_sep("abc-def-ghi", "_");
        assert_eq!(result, "ABC--DEF--GHI");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = cobol_case_with_sep("Abc-Def-Ghi", "-");
        assert_eq!(result, "ABC-DEF-GHI");

        let result = cobol_case_with_sep("Abc-Def-Ghi", "_");
        assert_eq!(result, "ABC--DEF--GHI");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = cobol_case_with_sep("ABC_DEF_GHI", "_");
        assert_eq!(result, "ABC-DEF-GHI");

        let result = cobol_case_with_sep("ABC_DEF_GHI", "-");
        assert_eq!(result, "ABC_-DEF_-GHI");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = cobol_case_with_sep("ABC-DEF-GHI", "-");
        assert_eq!(result, "ABC-DEF-GHI");

        let result = cobol_case_with_sep("ABC-DEF-GHI", "_");
        assert_eq!(result, "ABC--DEF--GHI");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = cobol_case_with_sep("abc123-456defG789HIJklMN12", "-");
        assert_eq!(result, "ABC123-456DEF-G789-HI-JKL-MN12");

        let result = cobol_case_with_sep("abc123-456defG789HIJklMN12", "_");
        assert_eq!(result, "ABC123--456DEF-G789-HI-JKL-MN12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = cobol_case_with_sep(":.abc~!@def#$ghi%&jk(lm)no/?", ":@$&()/");
        assert_eq!(result, ".-ABC~!-DEF#-GHI%-JK-LM-NO-?");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = cobol_case_with_sep("", "_-");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_cobol_case_with_keep {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = cobol_case_with_keep("abcDefGHIjk", "-_");
        assert_eq!(result, "ABC-DEF-GH-IJK");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = cobol_case_with_keep("AbcDefGHIjk", "-_");
        assert_eq!(result, "ABC-DEF-GH-IJK");
    }

    #[test]
    fn it_should_convert_snake_case() {
        let result = cobol_case_with_keep("abc_def_ghi", "-");
        assert_eq!(result, "ABC-DEF-GHI");

        let result = cobol_case_with_keep("abc_def_ghi", "_");
        assert_eq!(result, "ABC_-DEF_-GHI");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = cobol_case_with_keep("abc-def-ghi", "_");
        assert_eq!(result, "ABC-DEF-GHI");

        let result = cobol_case_with_keep("abc-def-ghi", "-");
        assert_eq!(result, "ABC--DEF--GHI");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = cobol_case_with_keep("Abc-Def-Ghi", "_");
        assert_eq!(result, "ABC-DEF-GHI");

        let result = cobol_case_with_keep("Abc-Def-Ghi", "-");
        assert_eq!(result, "ABC--DEF--GHI");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = cobol_case_with_keep("ABC_DEF_GHI", "-");
        assert_eq!(result, "ABC-DEF-GHI");

        let result = cobol_case_with_keep("ABC_DEF_GHI", "_");
        assert_eq!(result, "ABC_-DEF_-GHI");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = cobol_case_with_keep("ABC-DEF-GHI", "_");
        assert_eq!(result, "ABC-DEF-GHI");

        let result = cobol_case_with_keep("ABC-DEF-GHI", "-");
        assert_eq!(result, "ABC--DEF--GHI");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = cobol_case_with_keep("abc123-456defG789HIJklMN12", "_");
        assert_eq!(result, "ABC123-456DEF-G789-HI-JKL-MN12");

        let result = cobol_case_with_keep("abc123-456defG789HIJklMN12", "-");
        assert_eq!(result, "ABC123--456DEF-G789-HI-JKL-MN12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = cobol_case_with_keep(":.abc~!@def#$ghi%&jk(lm)no/?", ".~!#%?");
        assert_eq!(result, ".-ABC~!-DEF#-GHI%-JK-LM-NO-?");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = cobol_case_with_sep("", "_-");
        assert_eq!(result, "");
    }
}
