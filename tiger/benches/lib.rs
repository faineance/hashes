#![no_std]
#![feature(test)]
#[macro_use]
extern crate crypto_tests;
extern crate tiger;

bench_digest!(tiger::Tiger);
