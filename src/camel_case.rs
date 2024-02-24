// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

/// Converts a string to camel case.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is camel case.
///
/// This function targets only the upper and lower cases of ASCII alphabets for
/// capitalization, and all characters except ASCII alphabets and ASCII numbers
/// are eliminated as word separators.
///
/// ```rust
///     let camel = stringcase::camel_case("foo_bar_baz");
///     assert_eq!(camel, "fooBarBaz");
/// ```
pub fn camel_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    // .len returns byte count but ok in this case!

    enum ChIs {
      FirstOfStr,
      InFirstWord,
      NextOfUpper,
      NextOfMark,
      Others,
    }

    let mut flag = ChIs::FirstOfStr;

    for ch in input.chars() {
        if ch.is_ascii_uppercase() {
            match flag {
                ChIs::FirstOfStr | ChIs::InFirstWord => {
                    result.push(ch.to_ascii_lowercase());
                    flag = ChIs::InFirstWord;
                }
                ChIs::NextOfUpper => {
                    result.push(ch.to_ascii_lowercase());
                    //flag = ChIs::NextOfUpper;
                }
                _ => {
                    result.push(ch);
                    flag = ChIs::NextOfUpper
                }
            }
        } else if ch.is_ascii_lowercase() {
            match flag {
                ChIs::NextOfUpper => match result.pop() {
                    Some(prev) => {
                        result.push(prev.to_ascii_uppercase());
                        result.push(ch);
                        flag = ChIs::Others;
                    }
                    None => (), // impossible
                },
                ChIs::NextOfMark => {
                    result.push(ch.to_ascii_uppercase());
                    flag = ChIs::NextOfUpper;
                }
                _ => {
                    result.push(ch);
                    flag = ChIs::Others;
                }
            }
        } else if ch.is_ascii_digit() {
            result.push(ch);
            match flag {
                ChIs::NextOfMark => flag = ChIs::NextOfUpper,
                _ => flag = ChIs::Others,
            }
        } else {
            match flag {
                ChIs::FirstOfStr => (),
                _ => flag = ChIs::NextOfMark,
            }
        }
    }

    result
}

/// Converts a string to camel case using the specified characters as
/// separators.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is camel case.
///
/// This function targets only the upper and lower cases of ASCII alphabets for
/// capitalization, and the characters specified as the second argument of this
/// function are regarded as word separators and are eliminated.
///
/// ```rust
///     let camel = stringcase::camel_case_with_sep("foo-bar100%baz", "- ");
///     assert_eq!(camel, "fooBar100%Baz");
/// ```
pub fn camel_case_with_sep(input: &str, seps: &str) -> String {
    let mut result = String::with_capacity(input.len());
    // .len returns byte count but ok in this case!

    enum ChIs {
      FirstOfStr,
      InFirstWord,
      NextOfUpper,
      NextOfMark,
      Others,
    }
    let mut flag = ChIs::FirstOfStr;

    for ch in input.chars() {
        if seps.contains(ch) {
            match flag {
                ChIs::FirstOfStr => (),
                _ => flag = ChIs::NextOfMark,
            }
        } else if ch.is_ascii_uppercase() {
            match flag {
                ChIs::FirstOfStr | ChIs::InFirstWord => {
                    result.push(ch.to_ascii_lowercase());
                    flag = ChIs::InFirstWord;
                }
                ChIs::NextOfUpper => {
                    result.push(ch.to_ascii_lowercase());
                    //flag = ChIs::NextOfUpper;
                }
                _ => {
                    result.push(ch);
                    flag = ChIs::NextOfUpper;
                }
            }
        } else if ch.is_ascii_lowercase() {
            match flag {
                ChIs::NextOfUpper => match result.pop() {
                    Some(prev) => {
                        result.push(prev.to_ascii_uppercase());
                        result.push(ch);
                        flag = ChIs::Others;
                    }
                    None => (), // impossible
                },
                ChIs::NextOfMark => {
                    result.push(ch.to_ascii_uppercase());
                    flag = ChIs::NextOfUpper;
                }
                _ => {
                    result.push(ch);
                    flag = ChIs::Others;
                }
            }
        } else if ch.is_ascii_digit() {
            result.push(ch);
            match flag {
                ChIs::NextOfMark => flag = ChIs::NextOfUpper,
                _ => flag = ChIs::Others,
            }
        } else {
            result.push(ch);
            flag = ChIs::NextOfMark;
        }
    }

    result
}

/// Converts a string to camel case using characters other than the specified
/// characters as separators.
///
/// This function takes a string slice as its argument, then returns a `String`
/// of which the case style is camel case.
///
/// This function targets only the upper and lower cases of ASCII alphabets for
/// capitalization, and the characters other than the specified characters as
/// the second argument of this function are regarded as word separators and
/// are eliminated.
///
/// ```rust
///     let camel = stringcase::camel_case_with_keep("foo-bar100%baz", "%");
///     assert_eq!(camel, "fooBar100%Baz");
/// ```
pub fn camel_case_with_keep(input: &str, keeped: &str) -> String {
    let mut result = String::with_capacity(input.len());
    // .len returns byte count but ok in this case!

    enum ChIs {
        FirstOfStr,
        InFirstWord,
        NextOfUpper,
        NextOfMark,
        Others,
    }
    let mut flag = ChIs::FirstOfStr;

    for ch in input.chars() {
        if ch.is_ascii_uppercase() {
            match flag {
                ChIs::FirstOfStr | ChIs::InFirstWord => {
                    flag = ChIs::InFirstWord;
                    result.push(ch.to_ascii_lowercase());
                }
                ChIs::NextOfUpper => {
                    result.push(ch.to_ascii_lowercase());
                    //flag = ChIs::NextOfUpper;
                }
                _ => {
                    result.push(ch);
                    flag = ChIs::NextOfUpper;
                }
            }
        } else if ch.is_ascii_lowercase() {
            match flag {
                ChIs::NextOfUpper => match result.pop() {
                    Some(prev) => {
                        result.push(prev.to_ascii_uppercase());
                        result.push(ch);
                        flag = ChIs::Others;
                    }
                    None => (), // impossible
                },
                ChIs::NextOfMark => {
                    result.push(ch.to_ascii_uppercase());
                    flag = ChIs::NextOfUpper;
                }
                _ => {
                    result.push(ch);
                    flag = ChIs::Others;
                }
            }
        } else if ch.is_ascii_digit() {
            result.push(ch);
            match flag {
                ChIs::NextOfMark => flag = ChIs::NextOfUpper,
                _ => flag = ChIs::Others,
            }
        } else if keeped.contains(ch) {
            result.push(ch);
            flag = ChIs::NextOfMark;
        } else {
            match flag {
                ChIs::FirstOfStr => (),
                _ => flag = ChIs::NextOfMark,
            }
        }
    }

    result
}

#[cfg(test)]
mod tests_of_camel_case {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = camel_case("abcDefGHIjk");
        assert_eq!(result, "abcDefGhIjk");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = camel_case("AbcDefGHIjk");
        assert_eq!(result, "abcDefGhIjk");
    }

    #[test]
    fn it_should_convert_snake_case() {
        let result = camel_case("abc_def_ghi");
        assert_eq!(result, "abcDefGhi");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = camel_case("abc-def-ghi");
        assert_eq!(result, "abcDefGhi");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = camel_case("Abc-Def-Ghi");
        assert_eq!(result, "abcDefGhi");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = camel_case("ABC_DEF_GHI");
        assert_eq!(result, "abcDefGhi");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = camel_case("ABC-DEF-GHI");
        assert_eq!(result, "abcDefGhi");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = camel_case("abc123-456defG789HIJklMN12");
        assert_eq!(result, "abc123456defG789HiJklMn12");
    }

    #[test]
    fn is_should_treat_marks_as_separators() {
        let result = camel_case(":.abc~!@def#$ghi%&jk(lm)no/?");
        assert_eq!(result, "abcDefGhiJkLmNo");
    }

    #[test]
    fn it_should_convert_emtpy() {
        let result = camel_case("");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_camel_case_with_sep {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = camel_case_with_sep("abcDefGHIjk", "_-");
        assert_eq!(result, "abcDefGhIjk");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = camel_case_with_sep("AbcDefGHIjk", "_-");
        assert_eq!(result, "abcDefGhIjk");
    }

    #[test]
    fn it_should_convert_snake_case() {
        let result = camel_case_with_sep("abc_def_ghi", "_");
        assert_eq!(result, "abcDefGhi");

        let result = camel_case_with_sep("abc_def_ghi", "-");
        assert_eq!(result, "abc_Def_Ghi");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = camel_case_with_sep("abc-def-ghi", "-");
        assert_eq!(result, "abcDefGhi");

        let result = camel_case_with_sep("abc-def-ghi", "_");
        assert_eq!(result, "abc-Def-Ghi");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = camel_case_with_sep("Abc-Def-Ghi", "-");
        assert_eq!(result, "abcDefGhi");

        let result = camel_case_with_sep("Abc-Def-Ghi", "_");
        assert_eq!(result, "abc-Def-Ghi");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = camel_case_with_sep("ABC_DEF_GHI", "_");
        assert_eq!(result, "abcDefGhi");

        let result = camel_case_with_sep("ABC_DEF_GHI", "-");
        assert_eq!(result, "abc_Def_Ghi");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = camel_case_with_sep("ABC-DEF-GHI", "-");
        assert_eq!(result, "abcDefGhi");

        let result = camel_case_with_sep("ABC-DEF-GHI", "_");
        assert_eq!(result, "abc-Def-Ghi");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = camel_case_with_sep("abc123-456defG789HIJklMN12", "_");
        assert_eq!(result, "abc123-456defG789HiJklMn12");

        let result = camel_case_with_sep("abc123-456defG789HIJklMN12", "-");
        assert_eq!(result, "abc123456defG789HiJklMn12");
    }

    #[test]
    fn is_should_treat_marks_as_separators() {
        let result = camel_case_with_sep(":.abc~!@def#$ghi%&jk(lm)no/?", ":@$&()/");
        assert_eq!(result, ".Abc~!Def#Ghi%JkLmNo?");
    }

    #[test]
    fn it_should_convert_emtpy() {
        let result = camel_case_with_sep("", "-_");
        assert_eq!(result, "");
    }
}

#[cfg(test)]
mod tests_of_camel_case_with_keep {
    use super::*;

    #[test]
    fn it_should_convert_camel_case() {
        let result = camel_case_with_keep("abcDefGHIjk", "_-");
        assert_eq!(result, "abcDefGhIjk");
    }

    #[test]
    fn it_should_convert_pascal_case() {
        let result = camel_case_with_keep("AbcDefGHIjk", "_-");
        assert_eq!(result, "abcDefGhIjk");
    }

    #[test]
    fn it_should_convert_snake_case() {
        let result = camel_case_with_keep("abc_def_ghi", "-");
        assert_eq!(result, "abcDefGhi");

        let result = camel_case_with_keep("abc_def_ghi", "_");
        assert_eq!(result, "abc_Def_Ghi");
    }

    #[test]
    fn it_should_convert_kebab_case() {
        let result = camel_case_with_keep("abc-def-ghi", "_");
        assert_eq!(result, "abcDefGhi");

        let result = camel_case_with_keep("abc-def-ghi", "-");
        assert_eq!(result, "abc-Def-Ghi");
    }

    #[test]
    fn it_should_convert_train_case() {
        let result = camel_case_with_keep("Abc-Def-Ghi", "_");
        assert_eq!(result, "abcDefGhi");

        let result = camel_case_with_keep("Abc-Def-Ghi", "-");
        assert_eq!(result, "abc-Def-Ghi");
    }

    #[test]
    fn it_should_convert_macro_case() {
        let result = camel_case_with_keep("ABC_DEF_GHI", "-");
        assert_eq!(result, "abcDefGhi");

        let result = camel_case_with_keep("ABC_DEF_GHI", "_");
        assert_eq!(result, "abc_Def_Ghi");
    }

    #[test]
    fn it_should_convert_cobol_case() {
        let result = camel_case_with_keep("ABC-DEF-GHI", "_");
        assert_eq!(result, "abcDefGhi");

        let result = camel_case_with_keep("ABC-DEF-GHI", "-");
        assert_eq!(result, "abc-Def-Ghi");
    }

    #[test]
    fn it_should_keep_digits() {
        let result = camel_case_with_keep("abc123-456defG789HIJklMN12", "_");
        assert_eq!(result, "abc123456defG789HiJklMn12");

        let result = camel_case_with_keep("abc123-456defG789HIJklMN12", "-");
        assert_eq!(result, "abc123-456defG789HiJklMn12");
    }

    #[test]
    fn is_should_treat_marks_as_separators() {
        let result = camel_case_with_keep(":.abc~!@def#$ghi%&jk(lm)no/?", ".~!#%?");
        assert_eq!(result, ".Abc~!Def#Ghi%JkLmNo?");
    }

    #[test]
    fn it_should_convert_emtpy() {
        let result = camel_case_with_keep("", "-_");
        assert_eq!(result, "");
    }
}
