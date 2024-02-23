// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

/// Converts a string to train case.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is train case.
///
/// This function targets the upper and lower cases of ASCII alphabets for
/// capitalization, and all characters except ASCII alphabets and ASCII numbers
/// are eliminated as word separators.
///
/// ```rust
///     let train = stringcase::train_case("fooBarBaz");
///     assert_eq!(train, "Foo-Bar-Baz");
/// ```
pub fn train_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 2);
    // .len returns byte count but ok in this case!

    let mut flag: u8 = 0;
    // 0: first char
    // 1: previous char is upper
    // 2: one and two chars before are upper
    // 3: previous char is mark
    // 4: other

    for ch in input.chars() {
        if ch.is_ascii_uppercase() {
            match flag {
                1 | 2 => {
                    flag = 2;
                    result.push(ch.to_ascii_lowercase());
                }
                0 => {
                    flag = 1;
                    result.push(ch);
                }
                _ => {
                    flag = 1;
                    result.push('-');
                    result.push(ch);
                }
            }
        } else if ch.is_ascii_lowercase() {
            match flag {
                0 => {
                    flag = 1;
                    result.push(ch.to_ascii_uppercase());
                }
                1 => match result.pop() {
                    Some(prev) => {
                        flag = 4;
                        result.push(prev.to_ascii_uppercase());
                        result.push(ch);
                    }
                    None => (), // impossible
                },
                2 => match result.pop() {
                    Some(prev) => {
                        flag = 4;
                        result.push('-');
                        result.push(prev.to_ascii_uppercase());
                        result.push(ch);
                    }
                    None => (), // impossible
                },
                3 => {
                    flag = 1;
                    result.push('-');
                    result.push(ch.to_ascii_uppercase());
                }
                _ => {
                    flag = 4;
                    result.push(ch);
                }
            }
        } else if ch.is_ascii_digit() {
            match flag {
                0 => {
                    flag = 1;
                    result.push(ch);
                }
                3 => {
                    flag = 1;
                    result.push('-');
                    result.push(ch);
                }
                _ => {
                    flag = 4;
                    result.push(ch);
                }
            }
        } else {
            if flag != 0 {
                flag = 3;
            }
        }
    }

    result
}

/// Converts a string to train case using the specified characters as
/// separators.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is train case.
///
/// This function targets only the upper and lower cases of ASCII alphabets for
/// capitalization, and the characters specified as the second argument of this
/// function are regarded as word separators and are replaced to hyphens.
///
/// ```rust
///     let train = stringcase::train_case_with_sep("foo-Bar100%Baz", "- ");
///     assert_eq!(train, "Foo-Bar100%-Baz");
/// ```
pub fn train_case_with_sep(input: &str, seps: &str) -> String {
    let mut result = String::with_capacity(input.len() * 2);
    // .len returns byte count but ok in this case!

    let mut flag: u8 = 0;
    // 0: first char
    // 1: previous char is upper
    // 2: one and two chars before are upper
    // 3: previous char is mark
    // 4: other

    for ch in input.chars() {
        if seps.contains(ch) {
            if flag != 0 {
                flag = 3;
            }
        } else if ch.is_ascii_uppercase() {
            match flag {
                1 | 2 => {
                    flag = 2;
                    result.push(ch.to_ascii_lowercase());
                }
                0 => {
                    flag = 1;
                    result.push(ch);
                }
                _ => {
                    flag = 1;
                    result.push('-');
                    result.push(ch);
                }
            }
        } else if ch.is_ascii_lowercase() {
            match flag {
                0 => {
                    flag = 1;
                    result.push(ch.to_ascii_uppercase());
                }
                1 => match result.pop() {
                    Some(prev) => {
                        flag = 4;
                        result.push(prev.to_ascii_uppercase());
                        result.push(ch);
                    }
                    None => (), // impossible
                },
                2 => match result.pop() {
                    Some(prev) => {
                        flag = 4;
                        result.push('-');
                        result.push(prev.to_ascii_uppercase());
                        result.push(ch);
                    }
                    None => (), // impossible
                },
                3 => {
                    flag = 1;
                    result.push('-');
                    result.push(ch.to_ascii_uppercase());
                }
                _ => {
                    flag = 4;
                    result.push(ch);
                }
            }
        } else if ch.is_ascii_digit() {
            match flag {
                0 => {
                    flag = 1;
                    result.push(ch);
                }
                3 => {
                    flag = 1;
                    result.push('-');
                    result.push(ch);
                }
                _ => {
                    flag = 4;
                    result.push(ch);
                }
            }
        } else {
            flag = 3;
            result.push(ch);
        }
    }

    result
}

/// Converts a string to train case using characters other than the specified
/// characters as separators.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is train case.
///
/// This function targets only the upper and lower cases of ASCII alphabets for
/// capitalization, and the characters other than the specified characters as
/// the second argument of this function are regarded as word separators and
/// are replaced to hyphens.
///
/// ```rust
///     let train = stringcase::train_case_with_keep("foo-bar100%baz", "%");
///     assert_eq!(train, "Foo-Bar100%-Baz");
/// ```
pub fn train_case_with_keep(input: &str, keeped: &str) -> String {
    let mut result = String::with_capacity(input.len() * 2);
    // .len returns byte count but ok in this case!

    let mut flag: u8 = 0;
    // 0: first char
    // 1: previous char is upper
    // 2: one and two chars before are upper
    // 3: previous char is mark
    // 4: other

    for ch in input.chars() {
        if ch.is_ascii_uppercase() {
            match flag {
                1 | 2 => {
                    flag = 2;
                    result.push(ch.to_ascii_lowercase());
                }
                0 => {
                    flag = 1;
                    result.push(ch);
                }
                _ => {
                    flag = 1;
                    result.push('-');
                    result.push(ch);
                }
            }
        } else if ch.is_ascii_lowercase() {
            match flag {
                0 => {
                    flag = 1;
                    result.push(ch.to_ascii_uppercase());
                }
                1 => match result.pop() {
                    Some(prev) => {
                        flag = 4;
                        result.push(prev.to_ascii_uppercase());
                        result.push(ch);
                    }
                    None => (), // impossible
                },
                2 => match result.pop() {
                    Some(prev) => {
                        flag = 4;
                        result.push('-');
                        result.push(prev.to_ascii_uppercase());
                        result.push(ch);
                    }
                    None => (), // impossible
                },
                3 => {
                    flag = 1;
                    result.push('-');
                    result.push(ch.to_ascii_uppercase());
                }
                _ => {
                    flag = 4;
                    result.push(ch);
                }
            }
        } else if ch.is_ascii_digit() {
            match flag {
                0 => {
                    flag = 1;
                    result.push(ch);
                }
                3 => {
                    flag = 1;
                    result.push('-');
                    result.push(ch);
                }
                _ => {
                    flag = 4;
                    result.push(ch);
                }
            }
        } else if keeped.contains(ch) {
            flag = 3;
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
mod tests_of_train_case {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = train_case("abcDefGHIjk");
        assert_eq!(result, "Abc-Def-Gh-Ijk");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = train_case("AbcDefGHIjk");
        assert_eq!(result, "Abc-Def-Gh-Ijk");
    }

    #[test]
    fn it_should_convert_snake_case() {
        let result = train_case("abc_def_ghi");
        assert_eq!(result, "Abc-Def-Ghi");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = train_case("abc-def-ghi");
        assert_eq!(result, "Abc-Def-Ghi");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = train_case("Abc-Def-Ghi");
        assert_eq!(result, "Abc-Def-Ghi");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = train_case("ABC_DEF_GHI");
        assert_eq!(result, "Abc-Def-Ghi");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = train_case("ABC-DEF-GHI");
        assert_eq!(result, "Abc-Def-Ghi");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = train_case("abc123-456defG789HIJklMN12");
        assert_eq!(result, "Abc123-456def-G789-Hi-Jkl-Mn12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = train_case(":.abc~!@def#$ghi%&jk(lm)no/?");
        assert_eq!(result, "Abc-Def-Ghi-Jk-Lm-No");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = train_case("");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_train_case_with_sep {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = train_case_with_sep("abcDefGHIjk", "-_");
        assert_eq!(result, "Abc-Def-Gh-Ijk");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = train_case_with_sep("AbcDefGHIjk", "-_");
        assert_eq!(result, "Abc-Def-Gh-Ijk");
    }

    #[test]
    fn it_should_convert_snake_case() {
        let result = train_case_with_sep("abc_def_ghi", "_");
        assert_eq!(result, "Abc-Def-Ghi");

        let result = train_case_with_sep("abc_def_ghi", "-");
        assert_eq!(result, "Abc_-Def_-Ghi");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = train_case_with_sep("abc-def-ghi", "-");
        assert_eq!(result, "Abc-Def-Ghi");

        let result = train_case_with_sep("abc-def-ghi", "_");
        assert_eq!(result, "Abc--Def--Ghi");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = train_case_with_sep("Abc-Def-Ghi", "-");
        assert_eq!(result, "Abc-Def-Ghi");

        let result = train_case_with_sep("Abc-Def-Ghi", "_");
        assert_eq!(result, "Abc--Def--Ghi");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = train_case_with_sep("ABC_DEF_GHI", "_");
        assert_eq!(result, "Abc-Def-Ghi");

        let result = train_case_with_sep("ABC_DEF_GHI", "-");
        assert_eq!(result, "Abc_-Def_-Ghi");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = train_case_with_sep("ABC_DEF_GHI", "_");
        assert_eq!(result, "Abc-Def-Ghi");

        let result = train_case_with_sep("ABC_DEF_GHI", "-");
        assert_eq!(result, "Abc_-Def_-Ghi");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = train_case_with_sep("abc123-456defG789HIJklMN12", "-");
        assert_eq!(result, "Abc123-456def-G789-Hi-Jkl-Mn12");

        let result = train_case_with_sep("abc123-456defG789HIJklMN12", "_");
        assert_eq!(result, "Abc123--456def-G789-Hi-Jkl-Mn12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = train_case_with_sep(":.abc~!@def#$ghi%&jk(lm)no/?", ":@$&()/");
        assert_eq!(result, ".-Abc~!-Def#-Ghi%-Jk-Lm-No?");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = train_case_with_sep("", "-_");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_train_case_with_keep {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = train_case_with_keep("abcDefGHIjk", "-_");
        assert_eq!(result, "Abc-Def-Gh-Ijk");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = train_case_with_keep("AbcDefGHIjk", "-_");
        assert_eq!(result, "Abc-Def-Gh-Ijk");
    }

    #[test]
    fn it_should_convert_snake_case() {
        let result = train_case_with_keep("abc_def_ghi", "-");
        assert_eq!(result, "Abc-Def-Ghi");

        let result = train_case_with_keep("abc_def_ghi", "_");
        assert_eq!(result, "Abc_-Def_-Ghi");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = train_case_with_keep("abc-def-ghi", "_");
        assert_eq!(result, "Abc-Def-Ghi");

        let result = train_case_with_keep("abc-def-ghi", "-");
        assert_eq!(result, "Abc--Def--Ghi");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = train_case_with_keep("Abc-Def-Ghi", "_");
        assert_eq!(result, "Abc-Def-Ghi");

        let result = train_case_with_keep("Abc-Def-Ghi", "-");
        assert_eq!(result, "Abc--Def--Ghi");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = train_case_with_keep("ABC_DEF_GHI", "-");
        assert_eq!(result, "Abc-Def-Ghi");

        let result = train_case_with_keep("ABC_DEF_GHI", "_");
        assert_eq!(result, "Abc_-Def_-Ghi");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = train_case_with_keep("ABC-DEF-GHI", "_");
        assert_eq!(result, "Abc-Def-Ghi");

        let result = train_case_with_keep("ABC-DEF-GHI", "-");
        assert_eq!(result, "Abc--Def--Ghi");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = train_case_with_keep("abc123-456defG789HIJklMN12", "_");
        assert_eq!(result, "Abc123-456def-G789-Hi-Jkl-Mn12");

        let result = train_case_with_keep("abc123-456defG789HIJklMN12", "-");
        assert_eq!(result, "Abc123--456def-G789-Hi-Jkl-Mn12");
    }

    #[test]
    fn it_should_treat_marks_as_separators() {
        let result = train_case_with_keep(":.abc~!@def#$ghi%&jk(lm)no/?", ".~!#%?");
        assert_eq!(result, ".-Abc~!-Def#-Ghi%-Jk-Lm-No?");
    }

    #[test]
    fn it_should_convert_empty() {
        let result = train_case_with_keep("", "-_");
        assert_eq!(result, "");
    }
}
