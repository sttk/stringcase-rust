#![feature(test)]

extern crate test;

use stringcase::{cobol_case, cobol_case_with_sep, cobol_case_with_keep};
use test::Bencher;

#[bench]
fn bench_cobol_case(b: &mut Bencher) {
    b.iter(|| cobol_case("foo_bar100%BAZQux"));
}

#[bench]
fn bench_cobol_case_with_sep(b: &mut Bencher) {
    b.iter(|| cobol_case_with_sep("foo_bar100%BAZQux", "_"));
}

#[bench]
fn bench_cobol_case_with_keep(b: &mut Bencher) {
    b.iter(|| cobol_case_with_keep("foo_bar100%BAZQux", "%"));
}
