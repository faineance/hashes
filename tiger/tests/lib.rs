#![no_std]
#[macro_use]
extern crate crypto_tests;
extern crate tiger;

use crypto_tests::hash::{Test, main_test};

#[test]
fn tiger_main() {
    // Examples from wikipedia
    let tests = new_tests!("test1", "test2", "test3");
    main_test::<tiger::Tiger>(&tests);
}
