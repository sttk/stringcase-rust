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
    let mut result = String::from("");

    let mut flag: u8 = 0;
    // 0: first char
    // 1: previous char is upper
    // 2: one and two chars before are upper
    // 3: previous char is mark
    // 4: other

    for ch in input.chars() {
        if ch.is_ascii_uppercase() {
            match flag {
                0 => flag = 1,
                1 | 2 => flag = 2,
                _ => {
                    flag = 1;
                    result.push('-');
                }
            }
            result.push(ch);
        } else if ch.is_ascii_lowercase() {
            match flag {
                2 => match result.pop() {
                    Some(prev) => {
                        result.push('-');
                        result.push(prev);
                    }
                    None => (), // impossible
                },
                3 => result.push('-'),
                _ => (),
            }
            flag = 4;
            result.push(ch.to_ascii_uppercase());
        } else if ch.is_ascii_digit() {
            if flag == 3 {
                result.push('-');
            }
            flag = 4;
            result.push(ch);
        } else {
            if flag != 0 {
                flag = 3;
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
    let mut result = String::from("");

    let mut flag: u8 = 0;
    // 0: first char
    // 1: previous char is upper
    // 2: one and two chars before are upper
    // 3: previous char is mark (separator)
    // 4: previous char is mark (keeped)
    // 5: other

    for ch in input.chars() {
        if seps.contains(ch) {
            if flag != 0 {
                flag = 3;
            }
        } else if ch.is_ascii_uppercase() {
            match flag {
                0 => flag = 1,
                1 | 2 => flag = 2,
                _ => {
                    flag = 1;
                    result.push('-');
                }
            }
            result.push(ch);
        } else if ch.is_ascii_lowercase() {
            match flag {
                2 => match result.pop() {
                    Some(prev) => {
                        result.push('-');
                        result.push(prev);
                    }
                    None => (), // impossible
                },
                3 | 4 => result.push('-'),
                _ => (),
            }
            flag = 5;
            result.push(ch.to_ascii_uppercase());
        } else if ch.is_ascii_digit() {
            match flag {
                3 | 4 => result.push('-'),
                _ => (),
            }
            flag = 5;
            result.push(ch);
        } else {
            if flag == 3 {
                result.push('-');
            }
            flag = 4;
            result.push(ch);
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
    let mut result = String::from("");

    let mut flag: u8 = 0;
    // 0: first char
    // 1: previous char is upper
    // 2: one and two chars before are upper
    // 3: previous char is mark (separator)
    // 4: previous char is mark (keeped)
    // 5: other

    for ch in input.chars() {
        if ch.is_ascii_uppercase() {
            match flag {
                0 => flag = 1,
                1 | 2 => flag = 2,
                _ => {
                    flag = 1;
                    result.push('-');
                }
            }
            result.push(ch);
        } else if ch.is_ascii_lowercase() {
            match flag {
                2 => match result.pop() {
                    Some(prev) => {
                        result.push('-');
                        result.push(prev);
                    }
                    None => (), // impossible
                },
                3 | 4 => result.push('-'),
                _ => (),
            }
            flag = 5;
            result.push(ch.to_ascii_uppercase());
        } else if ch.is_ascii_digit() {
            match flag {
                3 | 4 => result.push('-'),
                _ => (),
            }
            flag = 5;
            result.push(ch);
        } else if keeped.contains(ch) {
            if flag == 3 {
                result.push('-');
            }
            flag = 4;
            result.push(ch);
        } else {
            if flag != 0 {
                flag = 3;
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
