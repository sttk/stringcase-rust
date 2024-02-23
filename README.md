# [stringcase-rust][repo-url] [![crate.io][crateio-img]][crateio-url] [![doc.rs][docrs-img]][docrs-url] [![CI Status][ci-img]][ci-url] [![MIT License][mit-img]][mit-url]

This library provides some functions that convert string cases between camelCase, COBOL-CASE, kebab-case, MACRO_CASE, PascalCase, snake_case and Train-Case.

Basically, these functions targets the upper and lower cases of only ASCII alphabets for capitalization, and all characters except ASCII alphabets and ASCII numbers are eliminated as word separators.

To limit characters using as separators, the functions named like `*_with_sep` are provided, and to keep specified characters, the functions named like `*_with_keep` are provided.

## Install

In `Cargo.toml`, write this crate as a dependency.

```toml
[dependencies]
stringcase = "0.1.1"
```

## Usage

The function contained in this crate are executed as follows:

```rust
use stringcase::camel_case;

fn main() {
    let input = "foo-bar-baz";
    let camel = camel_case(input);
    assert_eq!(camel, "fooBarBaz");
}
```

## Supporting Rust versions

This library supports Rust 1.56.1 or later.

```
% cargo msrv
Fetching index
Determining the Minimum Supported Rust Version (MSRV) for toolchain x86_64-apple-darwin
Using check command cargo check
Check for toolchain '1.66.1-x86_64-apple-darwin' succeeded
Check for toolchain '1.61.0-x86_64-apple-darwin' succeeded
Check for toolchain '1.58.1-x86_64-apple-darwin' succeeded
Check for toolchain '1.57.0-x86_64-apple-darwin' succeeded
Check for toolchain '1.56.1-x86_64-apple-darwin' succeeded
   Finished The MSRV is: 1.56.1   ████████████████████████████████████████████████████████████████ 00:02:42
```

## License

Copyright (C) 2024 Takayuki Sato

This program is free software under MIT License.<br>
See the file LICENSE in this distribution for more details.


[repo-url]: https://github.com/sttk/stringcase-rust
[crateio-img]: https://img.shields.io/badge/crate.io-ver.0.1.1-fc8d62?logo=rust
[crateio-url]: https://crates.io/crates/stringcase
[docrs-img]: https://img.shields.io/badge/doc.rs-stringcase-66c2a5?logo=docs.rs
[docrs-url]: https://docs.rs/stringcase
[ci-img]: https://github.com/sttk/stringcase-rust/actions/workflows/rust.yml/badge.svg?branch=main
[ci-url]: https://github.com/sttk/stringcase-rust/actions
[mit-img]: https://img.shields.io/badge/license-MIT-green.svg
[mit-url]: https://opensource.org/licenses/MIT
