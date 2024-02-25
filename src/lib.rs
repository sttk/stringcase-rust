// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

//! This library provides some functions that convert string cases between
//! camelCase, COBOL-CASE, kebab-case, MACRO_CASE, PascalCase, snake_case and
//! Train-Case.
//! And this library also provides a trait `Caser` which has some methods to
//! convert string cases.
//!
//! Basically, these functions targets the upper and lower cases of only ASCII
//! alphabets for capitalization, and all characters except ASCII alphabets and
//! ASCII numbers are eliminated as word separators.
//!
//! To limit characters using as separators, the functions named like
//! `*_with_sep` are provided, and to keep specified characters, the functions
//! named like `*_with_keep` are provided.
//!
//! ## Install
//!
//! In `Cargo.toml`, write this crate as a dependency.
//!
//! ```toml
//! [dependencies]
//! stringcase = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! The function contained in this crate are executed as follows:
//!
//! ```rust
//! use stringcase::camel_case;
//!
//! fn main() {
//!     let input = "foo-bar-baz";
//!     let camel = camel_case(input);
//!     assert_eq!(camel, "fooBarBaz");
//! }
//! ```
//!
//! And by bringing `Caser` with `use` declaration, it will be able to execute
//! methods of strings, `String` or `&str`, to convert their cases.
//!
//! ```rust
//! use stringcase::Caser;
//!
//! fn main() {
//!     let input = "foo-bar-baz";
//!     let camel = input.to_camel_case();
//!     assert_eq!(camel, "fooBarBaz");
//! }
//! ```

mod camel_case;
mod caser;
mod cobol_case;
mod kebab_case;
mod macro_case;
mod pascal_case;
mod snake_case;
mod train_case;

pub use camel_case::*;
pub use caser::*;
pub use cobol_case::*;
pub use kebab_case::*;
pub use macro_case::*;
pub use pascal_case::*;
pub use snake_case::*;
pub use train_case::*;
