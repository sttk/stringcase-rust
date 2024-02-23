// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

/// Converts a string to macro case.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is macro case.
///
/// This function targets the upper and lower cases of ASCII alphabets for
/// capitalization, and all characters except ASCII alphabets and ASCII numbers
/// are eliminated as word separators.
///
/// ```rust
///     let m = stringcase::macro_case("fooBarBaz");
///     assert_eq!(m, "FOO_BAR_BAZ");
/// ```
pub fn macro_case(input: &str) -> String {
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
                    result.push('_');
                }
            }
            result.push(ch);
        } else if ch.is_ascii_lowercase() {
            match flag {
                2 => match result.pop() {
                    Some(prev) => {
                        result.push('_');
                        result.push(prev);
                    }
                    None => (), // impossible
                },
                3 => result.push('_'),
                _ => (),
            }
            flag = 4;
            result.push(ch.to_ascii_uppercase());
        } else if ch.is_ascii_digit() {
            if flag == 3 {
                result.push('_');
            }
            flag = 4;
            result.push(ch)
        } else {
            if flag != 0 {
                flag = 3;
            }
        }
    }

    result
}

/// Converts a string to macro case using the specified characters as
/// separators.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is macro case.
///
/// This function targets only the upper and lower cases of ASCII alphabets for
/// capitalization, and the characters specified as the second argument of this
/// function are regarded as word separators and are replaced to underscores.
///
/// ```rust
///     let m = stringcase::macro_case_with_sep("foo-Bar100%Baz", "- ");
///     assert_eq!(m, "FOO_BAR100%_BAZ");
/// ```
pub fn macro_case_with_sep(input: &str, seps: &str) -> String {
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
                    result.push('_');
                }
            }
            result.push(ch);
        } else if ch.is_ascii_lowercase() {
            match flag {
                2 => match result.pop() {
                    Some(prev) => {
                        result.push('_');
                        result.push(prev);
                    }
                    None => (), // impossible
                },
                3 | 4 => result.push('_'),
                _ => (),
            }
            flag = 5;
            result.push(ch.to_ascii_uppercase());
        } else if ch.is_ascii_digit() {
            match flag {
                3 | 4 => result.push('_'),
                _ => (),
            }
            flag = 5;
            result.push(ch);
        } else {
            if flag == 3 {
                result.push('_');
            }
            flag = 4;
            result.push(ch);
        }
    }

    result
}

/// Converts a string to macro case using characters other than the specified
/// characters as separators.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is macro case.
///
/// This function targets only the upper and lower cases of ASCII alphabets for
/// capitalization, and the characters other than the specified characters as
/// the second argument of this function are regarded as word separators and
/// are replaced to underscores.
///
/// ```rust
///     let m = stringcase::macro_case_with_keep("foo-bar100%baz", "%");
///     assert_eq!(m, "FOO_BAR100%_BAZ");
/// ```
pub fn macro_case_with_keep(input: &str, keeped: &str) -> String {
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
                    result.push('_');
                }
            }
            result.push(ch);
        } else if ch.is_ascii_lowercase() {
            match flag {
                2 => match result.pop() {
                    Some(prev) => {
                        result.push('_');
                        result.push(prev);
                    }
                    None => (), // impossible
                },
                3 | 4 => result.push('_'),
                _ => (),
            }
            flag = 5;
            result.push(ch.to_ascii_uppercase());
        } else if ch.is_ascii_digit() {
            match flag {
                3 | 4 => result.push('_'),
                _ => (),
            }
            flag = 5;
            result.push(ch);
        } else if keeped.contains(ch) {
            if flag == 3 {
                result.push('_');
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
mod tests_of_macro_case {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = macro_case("abcDefGHIjk");
        assert_eq!(result, "ABC_DEF_GH_IJK");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = macro_case("AbcDefGHIjk");
        assert_eq!(result, "ABC_DEF_GH_IJK");
    }

    #[test]
    fn it_should_convert_snake_case() {
        let result = macro_case("abc_def_ghi");
        assert_eq!(result, "ABC_DEF_GHI");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = macro_case("abc-def-ghi");
        assert_eq!(result, "ABC_DEF_GHI");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = macro_case("Abc-Def-Ghi");
        assert_eq!(result, "ABC_DEF_GHI");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = macro_case("ABC_DEF_GHI");
        assert_eq!(result, "ABC_DEF_GHI");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = macro_case("ABC-DEF-GHI");
        assert_eq!(result, "ABC_DEF_GHI");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = macro_case("abc123-456defG89HIJklMN12");
        assert_eq!(result, "ABC123_456DEF_G89_HI_JKL_MN12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = macro_case(":.abc~!@def#$ghi%&jk(lm)no/?");
        assert_eq!(result, "ABC_DEF_GHI_JK_LM_NO");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = macro_case("");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_macro_case_with_sep {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = macro_case_with_sep("abcDefGHIjk", "-_");
        assert_eq!(result, "ABC_DEF_GH_IJK");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = macro_case_with_sep("AbcDefGHIjk", "-_");
        assert_eq!(result, "ABC_DEF_GH_IJK");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = macro_case_with_sep("abc-def-ghi", "-");
        assert_eq!(result, "ABC_DEF_GHI");

        let result = macro_case_with_sep("abc-def-ghi", "_");
        assert_eq!(result, "ABC-_DEF-_GHI");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = macro_case_with_sep("Abc-Def-Ghi", "-");
        assert_eq!(result, "ABC_DEF_GHI");

        let result = macro_case_with_sep("Abc-Def-Ghi", "_");
        assert_eq!(result, "ABC-_DEF-_GHI");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = macro_case_with_sep("ABC_DEF_GHI", "_");
        assert_eq!(result, "ABC_DEF_GHI");

        let result = macro_case_with_sep("ABC_DEF_GHI", "-");
        assert_eq!(result, "ABC__DEF__GHI");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = macro_case_with_sep("ABC-DEF-GHI", "-");
        assert_eq!(result, "ABC_DEF_GHI");

        let result = macro_case_with_sep("ABC-DEF-GHI", "_");
        assert_eq!(result, "ABC-_DEF-_GHI");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = macro_case_with_sep("abc123-456defG89HIJklMN12", "-");
        assert_eq!(result, "ABC123_456DEF_G89_HI_JKL_MN12");

        let result = macro_case_with_sep("abc123-456defG89HIJklMN12", "_");
        assert_eq!(result, "ABC123-_456DEF_G89_HI_JKL_MN12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = macro_case_with_sep(":.abc~!@def#$ghi%&jk(lm)no/?", ":@$&()/");
        assert_eq!(result, "._ABC~!_DEF#_GHI%_JK_LM_NO_?");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = macro_case_with_sep("", "_-");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_macro_case_with_keep {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = macro_case_with_keep("abcDefGHIjk", "-_");
        assert_eq!(result, "ABC_DEF_GH_IJK");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = macro_case_with_keep("AbcDefGHIjk", "-_");
        assert_eq!(result, "ABC_DEF_GH_IJK");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = macro_case_with_keep("abc-def-ghi", "_");
        assert_eq!(result, "ABC_DEF_GHI");

        let result = macro_case_with_keep("abc-def-ghi", "-");
        assert_eq!(result, "ABC-_DEF-_GHI");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = macro_case_with_keep("Abc-Def-Ghi", "_");
        assert_eq!(result, "ABC_DEF_GHI");

        let result = macro_case_with_keep("Abc-Def-Ghi", "-");
        assert_eq!(result, "ABC-_DEF-_GHI");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = macro_case_with_keep("ABC_DEF_GHI", "-");
        assert_eq!(result, "ABC_DEF_GHI");

        let result = macro_case_with_keep("ABC_DEF_GHI", "_");
        assert_eq!(result, "ABC__DEF__GHI");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = macro_case_with_keep("ABC-DEF-GHI", "_");
        assert_eq!(result, "ABC_DEF_GHI");

        let result = macro_case_with_keep("ABC-DEF-GHI", "-");
        assert_eq!(result, "ABC-_DEF-_GHI");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = macro_case_with_keep("abc123-456defG89HIJklMN12", "_");
        assert_eq!(result, "ABC123_456DEF_G89_HI_JKL_MN12");

        let result = macro_case_with_keep("abc123-456defG89HIJklMN12", "-");
        assert_eq!(result, "ABC123-_456DEF_G89_HI_JKL_MN12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = macro_case_with_keep(":.abc~!@def#$ghi%&jk(lm)no/?", ".~!#%?");
        assert_eq!(result, "._ABC~!_DEF#_GHI%_JK_LM_NO_?");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = macro_case_with_sep("", "_-");
        assert_eq!(result, "");
    }
}
