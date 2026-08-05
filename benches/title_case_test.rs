#![feature(test)]

extern crate test;

use test::Bencher;

use stringcase::{title_case, title_case_with_options, Options};

#[bench]
fn bench_title_case(b: &mut Bencher) {
    b.iter(|| title_case("foo_bar100%BAZQux"));
}

#[bench]
fn bench_title_case_with_sep(b: &mut Bencher) {
    let opts = Options {
        separators: "_",
        ..Default::default()
    };
    b.iter(|| title_case_with_options("foo_bar100%BAZQux", &opts));
}

#[bench]
fn bench_title_case_with_keep(b: &mut Bencher) {
    let opts = Options {
        keep: "%",
        ..Default::default()
    };
    b.iter(|| title_case_with_options("foo_bar100%BAZQux", &opts));
}

#[bench]
fn bench_title_case_with_nums_as_word(b: &mut Bencher) {
    let opts = Options {
        separate_before_non_alphabets: true,
        ..Default::default()
    };
    b.iter(|| title_case_with_options("foo_bar100%BAZQux", &opts));
}

#[bench]
fn bench_title_case_with_options(b: &mut Bencher) {
    let opts = Options::new(true, true, "", "%");
    b.iter(|| title_case_with_options("foo_bar100%BAZQux", &opts));
}
