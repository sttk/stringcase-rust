#![feature(test)]

extern crate test;

use stringcase::{camel_case, camel_case_with_sep, camel_case_with_keep};
use test::Bencher;

#[bench]
fn bench_camel_case(b: &mut Bencher) {
    b.iter(|| camel_case("foo_bar100%BAZQux"));
}

#[bench]
fn bench_camel_case_with_sep(b: &mut Bencher) {
    b.iter(|| camel_case_with_sep("foo_bar100%BAZQux", "_"));
}

#[bench]
fn bench_camel_case_with_keep(b: &mut Bencher) {
    b.iter(|| camel_case_with_keep("foo_bar100%BAZQux", "%"));
}
