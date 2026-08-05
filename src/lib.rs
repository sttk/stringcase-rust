// Copyright (C) 2024-2026 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

//! This library provides some functions that convert string cases between Ada_Case, camelCase,
//! COBOL-CASE, kebab-case, MACRO_CASE, PascalCase, snake_case, Title Case, and Train-Case.
//! In addition, generic functions `capitalize`, `lowerize`, and `upperize` are provided to convert
//! string cases with a custom joiner character.
//! And this library also provides a trait `Caser` which enables strings to convert themselves
//! to their cases by their own methods.
//!
//! Basically, these functions only target ASCII uppercase and lowercase letters for
//! capitalization. All characters other than ASCII uppercase and lowercase letters and ASCII
//! numbers are removed as word separators.
//!
//! If you want to use some symbols as separators, specify those symbols in the `separators` field
//! of `Options` struct and use the `〜case_with_options` function for the desired case.
//! If you want to retain certain symbols and use everything else as separators, specify those
//! symbols in `keep` field of `Options` struct and use the `〜case_with_options` function for the
//! desired case.
//!
//! Additionally, you can specify whether to place word boundaries before and/or after
//! non-alphabetic characters with conversion options.
//! This can be set using the `separate_before_non_alphabets` and `separate_after_non_alphabets`
//! fields in the `Options` struct.
//!
//! The `〜_case` functions that do not take `Options` as an argument only place word boundaries
//! after non-alphabetic characters.
//! In other words, they behave as if `separate_before_non_alphabets = false` and
//! `separate_after_non_alphabets = true`.
//!
//! ## Install
//!
//! In `Cargo.toml`, write this crate as a dependency.
//!
//! ```toml
//! [dependencies]
//! stringcase = "0.4.0"
//! ```
//!
//! ## Usage
//!
//! The functions in this crate can be used as follows:
//!
//! ```rust
//! use stringcase::snake_case;
//!
//! fn main() {
//!     let input = "fooBar123Baz";
//!     let snake = snake_case(input);
//!     assert_eq!(snake, "foo_bar123_baz");
//! }
//! ```
//!
//! If you want the conversion to behave differently, use `〜_case_with_options`.
//!
//! ```rust
//! use stringcase::{snake_case_with_options, Options};
//!
//! fn main() {
//!     let opts = Options{separate_before_non_alphabets: true, ..Default::default()};
//!     let input = "fooBar123Baz";
//!     let snake = snake_case_with_options(input, &opts);
//!     assert_eq!(snake, "foo_bar_123_baz");
//! }
//! ```
//!
//! You can also use the generic functions `capitalize`, `lowerize`, and `upperize` to convert
//! strings into capitalized, lowercased, or uppercased words joined by a custom joiner
//! character:
//!
//! ```rust
//! use stringcase::{capitalize, lowerize, upperize, Options};
//!
//! fn main() {
//!     let opts = Options {
//!         separate_before_non_alphabets: true,
//!         separate_after_non_alphabets: true,
//!         ..Default::default()
//!     };
//!     let input = "fooBar123Baz";
//!     assert_eq!(capitalize::<'.'>(input, &opts), "Foo.Bar.123.Baz");
//!     assert_eq!(lowerize::<'.'>(input, &opts), "foo.bar.123.baz");
//!     assert_eq!(upperize::<'.'>(input, &opts), "FOO.BAR.123.BAZ");
//! }
//! ```
//!
//! And by bringing `Caser` with `use` declaration, it will be able to execute
//! methods of strings, `String` or `&str`, to convert to their cases.
//!
//! ```rust
//! use stringcase::{Caser, Options};
//!
//! fn main() {
//!     let input = "fooBar123Baz";
//!     let snake = input.to_snake_case();
//!     assert_eq!(snake, "foo_bar123_baz");
//!
//!     let opts = Options{separate_before_non_alphabets: true, ..Default::default()};
//!     let snake = input.to_snake_case_with_options(&opts);
//!     assert_eq!(snake, "foo_bar_123_baz");
//! }
//! ```

mod options;
pub use options::Options;

mod upperize;
pub use upperize::upperize;

mod cobol_case;
mod macro_case;
pub use cobol_case::*;
pub use macro_case::*;

mod lowerize;
pub use lowerize::lowerize;

mod kebab_case;
mod snake_case;
pub use kebab_case::*;
pub use snake_case::*;

mod capitalize;
pub use capitalize::capitalize;

mod ada_case;
mod pascal_case;
mod title_case;
mod train_case;
pub use ada_case::*;
pub use pascal_case::*;
pub use title_case::*;
pub use train_case::*;

mod camel_case;
pub use camel_case::*;

mod caser;
pub use caser::*;
