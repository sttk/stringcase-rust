// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

/// Converts a string to kebab case.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is kebab case.
///
/// This function targets the upper and lower cases of ASCII alphabets for
/// capitalization, and all characters except ASCII alphabets and ASCII numbers
/// are eliminated as word separators.
///
/// ```rust
///     let kebab = stringcase::kebab_case("fooBarBaz");
///     assert_eq!(kebab, "foo-bar-baz");
/// ```
pub fn kebab_case(input: &str) -> String {
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
                },
            }
            result.push(ch.to_ascii_lowercase());
        } else if ch.is_ascii_lowercase() {
            match flag {
                2 => match result.pop() {
                    Some(prev) => {
                        result.push('-');
                        result.push(prev);
                    },
                    None => (), // imbossible
                },
                3 => result.push('-'),
                _ => (),
            }
            flag = 4;
            result.push(ch);
        } else if ch.is_ascii_digit() {
            match flag {
                3 => result.push('-'),
                _ => (),
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

/// Converts a string to kebab case using the specified characters as
/// separators.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is kebab case.
///
/// This function targets only the upper and lower cases of ASCII alphabets for
/// capitalization, and the characters specified as the second argument of this
/// function are regarded as word separators and are replaced to hyphens.
///
/// ```rust
///     let kebab = stringcase::kebab_case_with_sep("foo-Bar100%Baz", "- ");
///     assert_eq!(kebab, "foo-bar100%-baz");
/// ```
pub fn kebab_case_with_sep(input: &str, seps: &str) -> String {
    let mut result = String::from("");

    let mut flag: u8 = 0;
    // 0: first char
    // 1: previous char is upper
    // 2: one an two chars before is upper
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
                },
            }
            result.push(ch.to_ascii_lowercase());
        } else if ch.is_ascii_lowercase() {
            match flag {
                2 => match result.pop() {
                    Some(prev) => {
                        result.push('-');
                        result.push(prev);
                    },
                    None => (), // impossible
                },
                3 | 4 => result.push('-'),
                _ => (),
            }
            flag = 5;
            result.push(ch);
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

/// Converts a string to kebab case using characters other than the specified
/// characters as separators.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is kebab case.
///
/// This function targets only the upper and lower cases of ASCII alphabets for
/// capitalization, and the characters other than the specified characters as
/// the second argument of this function are regarded as word separators and
/// are replaced to hyphens.
///
/// ```rust
///     let kebab = stringcase::kebab_case_with_keep("foo-Bar100%Baz", "%");
///     assert_eq!(kebab, "foo-bar100%-baz");
/// ```
pub fn kebab_case_with_keep(input: &str, keeped: &str) -> String {
    let mut result = String::from("");

    let mut flag: u8 = 0;
    // 0: first char
    // 1: previous char is upper
    // 2: one and two chars before is upper
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
                },
            }
            result.push(ch.to_ascii_lowercase());
        } else if ch.is_ascii_lowercase() {
            match flag {
                2 => match result.pop() {
                    Some(prev) => {
                        result.push('-');
                        result.push(prev);
                    },
                    None => (), // impossibble
                },
                3 | 4 => result.push('-'),
                _ => (),
            }
            flag = 5;
            result.push(ch);
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
mod tests_of_kebab_case {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = kebab_case("abcDefGhi");
        assert_eq!(result, "abc-def-ghi");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = kebab_case("AbcDefGhi");
        assert_eq!(result, "abc-def-ghi");
    }

    #[test]
    fn it_should_convert_snake_case() {
        let result = kebab_case("abc_def_ghi");
        assert_eq!(result, "abc-def-ghi");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = kebab_case("abc-def-ghi");
        assert_eq!(result, "abc-def-ghi");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = kebab_case("Abc-Def-Ghi");
        assert_eq!(result, "abc-def-ghi");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = kebab_case("ABC_DEF_GHI");
        assert_eq!(result, "abc-def-ghi");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = kebab_case("ABC-DEF-GHI");
        assert_eq!(result, "abc-def-ghi");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = kebab_case("abc123-456defG89HIJklMN12");
        assert_eq!(result, "abc123-456def-g89-hi-jkl-mn12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = kebab_case(":.abc~!@def#$ghi%&jk(lm)no/?");
        assert_eq!(result, "abc-def-ghi-jk-lm-no");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = kebab_case("");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_kebab_case_with_sep {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = kebab_case_with_sep("abcDefGhi", "-_");
        assert_eq!(result, "abc-def-ghi");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = kebab_case_with_sep("AbcDefGHIjk", "-_");
        assert_eq!(result, "abc-def-gh-ijk");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = kebab_case_with_sep("abc-def-ghi", "-");
        assert_eq!(result, "abc-def-ghi");

        let result = kebab_case_with_sep("abc-def-ghi", "_");
        assert_eq!(result, "abc--def--ghi");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = kebab_case_with_sep("Abc-Def-Ghi", "-");
        assert_eq!(result, "abc-def-ghi");

        let result = kebab_case_with_sep("Abc-Def-Ghi", "_");
        assert_eq!(result, "abc--def--ghi");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = kebab_case_with_sep("ABC_DEF_GHI", "_");
        assert_eq!(result, "abc-def-ghi");

        let result = kebab_case_with_sep("ABC_DEF_GHI", "-");
        assert_eq!(result, "abc_-def_-ghi");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = kebab_case_with_sep("ABC-DEF-GHI", "-");
        assert_eq!(result, "abc-def-ghi");

        let result = kebab_case_with_sep("ABC-DEF-GHI", "_");
        assert_eq!(result, "abc--def--ghi");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = kebab_case_with_sep("abc123-456defG89HIJklMN12", "-");
        assert_eq!(result, "abc123-456def-g89-hi-jkl-mn12");

        let result = kebab_case_with_sep("abc123-456defG89HIJklMN12", "_");
        assert_eq!(result, "abc123--456def-g89-hi-jkl-mn12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = kebab_case_with_sep(":.abc~!@def#$ghi%&jk(lm)no/?", ":@$&()/");
        assert_eq!(result, ".-abc~!-def#-ghi%-jk-lm-no-?");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = kebab_case_with_sep("", "_-");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_kebab_case_with_keep {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = kebab_case_with_keep("abcDefGhi", "-_");
        assert_eq!(result, "abc-def-ghi");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = kebab_case_with_keep("AbcDefGHIjk", "-_");
        assert_eq!(result, "abc-def-gh-ijk");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = kebab_case_with_keep("abc-def-ghi", "_");
        assert_eq!(result, "abc-def-ghi");

        let result = kebab_case_with_keep("abc-def-ghi", "-");
        assert_eq!(result, "abc--def--ghi");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = kebab_case_with_keep("Abc-Def-Ghi", "_");
        assert_eq!(result, "abc-def-ghi");

        let result = kebab_case_with_keep("Abc-Def-Ghi", "-");
        assert_eq!(result, "abc--def--ghi");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = kebab_case_with_keep("ABC_DEF_GHI", "-");
        assert_eq!(result, "abc-def-ghi");

        let result = kebab_case_with_keep("ABC_DEF_GHI", "_");
        assert_eq!(result, "abc_-def_-ghi");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = kebab_case_with_keep("ABC-DEF-GHI", "_");
        assert_eq!(result, "abc-def-ghi");

        let result = kebab_case_with_keep("ABC-DEF-GHI", "-");
        assert_eq!(result, "abc--def--ghi");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = kebab_case_with_keep("abc123-456defG89HIJklMN12", "_");
        assert_eq!(result, "abc123-456def-g89-hi-jkl-mn12");

        let result = kebab_case_with_keep("abc123-456defG89HIJklMN12", "-");
        assert_eq!(result, "abc123--456def-g89-hi-jkl-mn12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = kebab_case_with_keep(":.abc~!@def#$ghi%&jk(lm)no/?", ".~!#%?");
        assert_eq!(result, ".-abc~!-def#-ghi%-jk-lm-no-?");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = kebab_case_with_sep("", "_-");
        assert_eq!(result, "");
    }
}
