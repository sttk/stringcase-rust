# [stringcase for Rust][repo-url] [![crates.io][cratesio-img]][cratesio-url] [![doc.rs][docrs-img]][docrs-url] [![CI Status][ci-img]][ci-url] [![MIT License][mit-img]][mit-url]

This library provides some functions that convert string cases between camelCase, COBOL-CASE, kebab-case, MACRO_CASE, PascalCase, snake_case and Train-Case.
And this library also provides a trait `Caser` which enables strings to convert themselves to their cases by their own methods.

Basically, these functions only target ASCII uppercase and lowercase letters for capitalization. All characters other than ASCII uppercase and lowercase letters and ASCII numbers are removed as word separators.

If you want to use some symbols as separators, specify those symbols in the `separators` field of `Options` struct and use the `〜case_with_options` function for the desired case.
If you want to retain certain symbols and use everything else as separators, specify those symbols in `keep` field of `Options` struct and use the `〜case_with_options` function for the desired case.

Additionally, you can specify whether to place word boundaries before and/or after non-alphabetic characters with conversion options.
This can be set using the `separate_before_non_alphabets` and `separate_after_non_alphabets` fields in the `Options` struct.

The `〜_case` functions that do not take `Options` as an argument only place word boundaries after non-alphabetic characters.
In other words, they behave as if `separate_before_non_alphabets = false` and `separate_after_non_alphabets = true`.

## Installation

In `Cargo.toml`, write this crate as a dependency.

```toml
[dependencies]
stringcase = "0.4.0"
```

## Usage

The functions in this crate can be executed as follows:

```rust
use stringcase::snake_case;

fn main() {
    let input = "fooBar123Baz";
    let snake = snake_case(input);
    assert_eq!(snake, "foo_bar123_baz");
}
```

If you want the conversion to behave differently, use `〜_case_with_options`.

```rust
use stringcase::{snake_case_with_options, Options};

fn main() {
    let opts = Options{separate_before_non_alphabets: true, ..Default::default()};
    let input = "fooBar123Baz";
    let snake = snake_case_with_options(input, &opts);
    assert_eq!(snake, "foo_bar_123_baz");
}
```

And by bringing `Caser` with `use` declaration, it will be able to execute methods of strings, `String` or `&str`, to convert themselves to their cases.

```rust
use stringcase::{Caser, Options};

fn main() {
    let input = "fooBar123Baz";
    let snake = input.to_snake_case();
    assert_eq!(snake, "foo_bar123_baz");

    let opts = Options{separate_before_non_alphabets: true, ..Default::default()};
    let snake = input.to_snake_case_with_options(&opts);
    assert_eq!(snake, "foo_bar_123_baz");
}
```

## Supporting Rust versions

This library supports Rust 1.56.1 or later.

```
% ./build.sh msrv
  [Meta]   cargo-msrv 0.18.4

Compatibility Check #1: Rust 1.73.0
  [OK]     Is compatible

Compatibility Check #2: Rust 1.64.0
  [OK]     Is compatible

Compatibility Check #3: Rust 1.60.0
  [OK]     Is compatible

Compatibility Check #4: Rust 1.58.1
  [OK]     Is compatible

Compatibility Check #5: Rust 1.57.0
  [OK]     Is compatible

Compatibility Check #6: Rust 1.56.1
  [OK]     Is compatible

Result:
   Considered (min … max):   Rust 1.56.1 … Rust 1.89.0
   Search method:            bisect
   MSRV:                     1.56.1
   Target:                   x86_64-apple-darwin
```

## License

Copyright (C) 2024-2025 Takayuki Sato

This program is free software under MIT License.<br>
See the file LICENSE in this distribution for more details.


[repo-url]: https://github.com/sttk/stringcase-rust
[cratesio-img]: https://img.shields.io/badge/crates.io-ver.0.4.0-fc8d62?logo=rust
[cratesio-url]: https://crates.io/crates/stringcase
[docrs-img]: https://img.shields.io/badge/doc.rs-stringcase-66c2a5?logo=docs.rs
[docrs-url]: https://docs.rs/stringcase
[ci-img]: https://github.com/sttk/stringcase-rust/actions/workflows/rust.yml/badge.svg?branch=main
[ci-url]: https://github.com/sttk/stringcase-rust/actions?query=branch%3Amain
[mit-img]: https://img.shields.io/badge/license-MIT-green.svg
[mit-url]: https://opensource.org/licenses/MIT
