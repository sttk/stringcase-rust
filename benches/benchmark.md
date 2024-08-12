## 0.3.0

> - Date: 2024/08/12
> - rustc 1.80.1 (3f5fd8dd4 2024-08-06)

```
% ./build.sh bench

running 320 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 87/320
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 174/320
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 261/320
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii
test result: ok. 0 passed; 0 failed; 320 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test bench_camel_case           ... bench:         117.95 ns/iter (+/- 5.98)
test bench_camel_case_with_keep ... bench:         130.06 ns/iter (+/- 1.95)
test bench_camel_case_with_sep  ... bench:         219.25 ns/iter (+/- 3.74)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.61s


running 4 tests
test bench_cobol_case                   ... bench:         128.08 ns/iter (+/- 5.10)
test bench_cobol_case_with_keep         ... bench:         145.26 ns/iter (+/- 3.27)
test bench_cobol_case_with_nums_as_word ... bench:         125.37 ns/iter (+/- 3.79)
test bench_cobol_case_with_sep          ... bench:         224.37 ns/iter (+/- 5.25)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 11.18s


running 4 tests
test bench_kebab_case                   ... bench:         114.68 ns/iter (+/- 5.19)
test bench_kebab_case_with_keep         ... bench:         141.48 ns/iter (+/- 3.79)
test bench_kebab_case_with_nums_as_word ... bench:         127.85 ns/iter (+/- 6.08)
test bench_kebab_case_with_sep          ... bench:         209.53 ns/iter (+/- 10.48)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 1.85s


running 4 tests
test bench_macro_case                   ... bench:         128.51 ns/iter (+/- 5.29)
test bench_macro_case_with_keep         ... bench:         144.81 ns/iter (+/- 5.90)
test bench_macro_case_with_nums_as_word ... bench:         126.36 ns/iter (+/- 4.99)
test bench_macro_case_with_sep          ... bench:         222.56 ns/iter (+/- 6.66)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 3.71s


running 3 tests
test bench_pascal_case           ... bench:         127.49 ns/iter (+/- 6.61)
test bench_pascal_case_with_keep ... bench:         140.21 ns/iter (+/- 5.25)
test bench_pascal_case_with_sep  ... bench:         237.46 ns/iter (+/- 6.03)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 4.37s


running 4 tests
test bench_snake_case                   ... bench:         115.38 ns/iter (+/- 4.57)
test bench_snake_case_with_keep         ... bench:         142.41 ns/iter (+/- 5.35)
test bench_snake_case_with_nums_as_word ... bench:         126.88 ns/iter (+/- 4.01)
test bench_snake_case_with_sep          ... bench:         211.74 ns/iter (+/- 8.32)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 5.14s


running 4 tests
test bench_train_case                   ... bench:         118.75 ns/iter (+/- 2.79)
test bench_train_case_with_keep         ... bench:         136.30 ns/iter (+/- 8.72)
test bench_train_case_with_nums_as_word ... bench:         118.41 ns/iter (+/- 2.57)
test bench_train_case_with_sep          ... bench:         182.70 ns/iter (+/- 5.04)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 11.13s
```

## 0.2.1

> - Date: 2024/08/12
> - rustc 1.80.1 (3f5fd8dd4 2024-08-06)

```
% ./build.sh bench

running 248 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 87/248
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 174/248
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii
test result: ok. 0 passed; 0 failed; 248 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test bench_camel_case           ... bench:         119.61 ns/iter (+/- 5.54)
test bench_camel_case_with_keep ... bench:         133.05 ns/iter (+/- 5.09)
test bench_camel_case_with_sep  ... bench:         220.75 ns/iter (+/- 5.18)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 1.96s


running 3 tests
test bench_cobol_case           ... bench:         126.28 ns/iter (+/- 3.90)
test bench_cobol_case_with_keep ... bench:         144.03 ns/iter (+/- 4.27)
test bench_cobol_case_with_sep  ... bench:         220.31 ns/iter (+/- 5.63)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 3.77s


running 3 tests
test bench_kebab_case           ... bench:         125.43 ns/iter (+/- 6.52)
test bench_kebab_case_with_keep ... bench:         140.52 ns/iter (+/- 6.26)
test bench_kebab_case_with_sep  ... bench:         213.29 ns/iter (+/- 8.70)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.68s


running 3 tests
test bench_macro_case           ... bench:         124.57 ns/iter (+/- 4.52)
test bench_macro_case_with_keep ... bench:         145.26 ns/iter (+/- 4.05)
test bench_macro_case_with_sep  ... bench:         226.62 ns/iter (+/- 5.38)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 4.33s


running 3 tests
test bench_pascal_case           ... bench:         124.71 ns/iter (+/- 4.41)
test bench_pascal_case_with_keep ... bench:         137.41 ns/iter (+/- 3.87)
test bench_pascal_case_with_sep  ... bench:         190.97 ns/iter (+/- 7.21)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 6.78s


running 3 tests
test bench_snake_case           ... bench:         156.07 ns/iter (+/- 4.29)
test bench_snake_case_with_keep ... bench:         167.70 ns/iter (+/- 4.13)
test bench_snake_case_with_sep  ... bench:         242.15 ns/iter (+/- 5.27)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 1.13s


running 3 tests
test bench_train_case           ... bench:         121.08 ns/iter (+/- 1.64)
test bench_train_case_with_keep ... bench:         134.10 ns/iter (+/- 3.67)
test bench_train_case_with_sep  ... bench:         212.42 ns/iter (+/- 5.32)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 5.62s
```

## 0.2.1 

> - Date: 2024/02/26
> - Rust: 1.76.0 (c84b36747 2024-01-18)

```
% ./build.sh bench

running 248 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 88/248
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 176/248
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii
test result: ok. 0 passed; 0 failed; 248 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test bench_camel_case           ... bench:          82 ns/iter (+/- 7)
test bench_camel_case_with_keep ... bench:          88 ns/iter (+/- 6)
test bench_camel_case_with_sep  ... bench:         150 ns/iter (+/- 7)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 2.44s


running 3 tests
test bench_cobol_case           ... bench:          78 ns/iter (+/- 5)
test bench_cobol_case_with_keep ... bench:          91 ns/iter (+/- 6)
test bench_cobol_case_with_sep  ... bench:         146 ns/iter (+/- 12)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.94s


running 3 tests
test bench_kebab_case           ... bench:          80 ns/iter (+/- 6)
test bench_kebab_case_with_keep ... bench:          87 ns/iter (+/- 5)
test bench_kebab_case_with_sep  ... bench:         151 ns/iter (+/- 10)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.92s


running 3 tests
test bench_macro_case           ... bench:         104 ns/iter (+/- 6)
test bench_macro_case_with_keep ... bench:         105 ns/iter (+/- 33)
test bench_macro_case_with_sep  ... bench:         143 ns/iter (+/- 9)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 1.20s


running 3 tests
test bench_pascal_case           ... bench:          79 ns/iter (+/- 9)
test bench_pascal_case_with_keep ... bench:          95 ns/iter (+/- 2)
test bench_pascal_case_with_sep  ... bench:         146 ns/iter (+/- 36)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 7.45s


running 3 tests
test bench_snake_case           ... bench:          84 ns/iter (+/- 4)
test bench_snake_case_with_keep ... bench:          90 ns/iter (+/- 7)
test bench_snake_case_with_sep  ... bench:         158 ns/iter (+/- 214)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.98s


running 3 tests
test bench_train_case           ... bench:          82 ns/iter (+/- 5)
test bench_train_case_with_keep ... bench:          89 ns/iter (+/- 3)
test bench_train_case_with_sep  ... bench:         140 ns/iter (+/- 10)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 6.83s
```

## 0.2.0

```
% ./build.sh bench

running 248 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 88/248
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 176/248
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii
test result: ok. 0 passed; 0 failed; 248 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test bench_camel_case           ... bench:          78 ns/iter (+/- 3)
test bench_camel_case_with_keep ... bench:          88 ns/iter (+/- 6)
test bench_camel_case_with_sep  ... bench:         149 ns/iter (+/- 12)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.91s


running 3 tests
test bench_cobol_case           ... bench:         108 ns/iter (+/- 21)
test bench_cobol_case_with_keep ... bench:         103 ns/iter (+/- 8)
test bench_cobol_case_with_sep  ... bench:         146 ns/iter (+/- 12)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.94s


running 3 tests
test bench_kebab_case           ... bench:          82 ns/iter (+/- 24)
test bench_kebab_case_with_keep ... bench:          87 ns/iter (+/- 2)
test bench_kebab_case_with_sep  ... bench:         155 ns/iter (+/- 12)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 9.63s


running 3 tests
test bench_macro_case           ... bench:          78 ns/iter (+/- 4)
test bench_macro_case_with_keep ... bench:          90 ns/iter (+/- 2)
test bench_macro_case_with_sep  ... bench:         144 ns/iter (+/- 8)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 9.58s


running 3 tests
test bench_pascal_case           ... bench:          77 ns/iter (+/- 3)
test bench_pascal_case_with_keep ... bench:          93 ns/iter (+/- 12)
test bench_pascal_case_with_sep  ... bench:         145 ns/iter (+/- 9)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 4.26s


running 3 tests
test bench_snake_case           ... bench:          80 ns/iter (+/- 7)
test bench_snake_case_with_keep ... bench:          89 ns/iter (+/- 27)
test bench_snake_case_with_sep  ... bench:         151 ns/iter (+/- 11)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.78s


running 3 tests
test bench_train_case           ... bench:          82 ns/iter (+/- 6)
test bench_train_case_with_keep ... bench:          86 ns/iter (+/- 8)
test bench_train_case_with_sep  ... bench:         142 ns/iter (+/- 14)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 1.40s
```


## 0.1.1

```
% ./build.sh bench

running 206 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 88/206
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 176/206
iiiiiiiiiiiiiiiiiiiiiiiiiiiiii
test result: ok. 0 passed; 0 failed; 206 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test bench_camel_case           ... bench:          79 ns/iter (+/- 4)
test bench_camel_case_with_keep ... bench:          86 ns/iter (+/- 5)
test bench_camel_case_with_sep  ... bench:         129 ns/iter (+/- 10)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 3.99s


running 3 tests
test bench_cobol_case           ... bench:         104 ns/iter (+/- 10)
test bench_cobol_case_with_keep ... bench:         117 ns/iter (+/- 15)
test bench_cobol_case_with_sep  ... bench:         165 ns/iter (+/- 13)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 1.03s


running 3 tests
test bench_kebab_case           ... bench:          79 ns/iter (+/- 6)
test bench_kebab_case_with_keep ... bench:          91 ns/iter (+/- 7)
test bench_kebab_case_with_sep  ... bench:         135 ns/iter (+/- 13)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.85s


running 3 tests
test bench_macro_case           ... bench:          81 ns/iter (+/- 6)
test bench_macro_case_with_keep ... bench:          92 ns/iter (+/- 6)
test bench_macro_case_with_sep  ... bench:         142 ns/iter (+/- 17)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 10.21s


running 3 tests
test bench_pascal_case           ... bench:          86 ns/iter (+/- 7)
test bench_pascal_case_with_keep ... bench:          86 ns/iter (+/- 6)
test bench_pascal_case_with_sep  ... bench:         126 ns/iter (+/- 10)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 1.16s


running 3 tests
test bench_snake_case           ... bench:          79 ns/iter (+/- 7)
test bench_snake_case_with_keep ... bench:          94 ns/iter (+/- 6)
test bench_snake_case_with_sep  ... bench:         136 ns/iter (+/- 12)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.86s


running 3 tests
test bench_train_case           ... bench:          86 ns/iter (+/- 10)
test bench_train_case_with_keep ... bench:          86 ns/iter (+/- 11)
test bench_train_case_with_sep  ... bench:         134 ns/iter (+/- 8)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 10.44s
```

## 0.1.0

```
% ./build.sh bench

running 206 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 88/206
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii 176/206
iiiiiiiiiiiiiiiiiiiiiiiiiiiiii
test result: ok. 0 passed; 0 failed; 206 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 3 tests
test bench_camel_case           ... bench:         118 ns/iter (+/- 7)
test bench_camel_case_with_keep ... bench:         131 ns/iter (+/- 13)
test bench_camel_case_with_sep  ... bench:         174 ns/iter (+/- 8)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 5.84s


running 3 tests
test bench_cobol_case           ... bench:         216 ns/iter (+/- 29)
test bench_cobol_case_with_keep ... bench:         235 ns/iter (+/- 10)
test bench_cobol_case_with_sep  ... bench:         295 ns/iter (+/- 27)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 5.56s


running 3 tests
test bench_kebab_case           ... bench:         198 ns/iter (+/- 11)
test bench_kebab_case_with_keep ... bench:         214 ns/iter (+/- 11)
test bench_kebab_case_with_sep  ... bench:         257 ns/iter (+/- 19)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 4.17s


running 3 tests
test bench_macro_case           ... bench:         190 ns/iter (+/- 15)
test bench_macro_case_with_keep ... bench:         200 ns/iter (+/- 12)
test bench_macro_case_with_sep  ... bench:         235 ns/iter (+/- 26)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.55s


running 3 tests
test bench_pascal_case           ... bench:         131 ns/iter (+/- 12)
test bench_pascal_case_with_keep ... bench:         126 ns/iter (+/- 12)
test bench_pascal_case_with_sep  ... bench:         180 ns/iter (+/- 3)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 6.29s


running 3 tests
test bench_snake_case           ... bench:         198 ns/iter (+/- 15)
test bench_snake_case_with_keep ... bench:         208 ns/iter (+/- 14)
test bench_snake_case_with_sep  ... bench:         258 ns/iter (+/- 6)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 5.45s


running 3 tests
test bench_train_case           ... bench:         202 ns/iter (+/- 19)
test bench_train_case_with_keep ... bench:         219 ns/iter (+/- 61)
test bench_train_case_with_sep  ... bench:         243 ns/iter (+/- 19)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.99s
```
