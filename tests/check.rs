extern crate bytecount;
#[cfg(not(feature = "cargo-miri"))]
#[macro_use]
extern crate quickcheck;
extern crate rand;

use std::iter;
use bytecount::{
    count, naive_count,
    num_chars, naive_num_chars,
};
use rand::Rng;
#[cfg(feature = "cargo-miri")]
use rand::SeedableRng;

fn random_bytes(len: usize) -> Vec<u8> {
    #[cfg(not(feature = "cargo-miri"))]
    let mut rng = rand::thread_rng();
    #[cfg(feature = "cargo-miri")]
    let mut rng = rand::StdRng::from_seed(&[0xdeadcafe]);
    rng.gen_iter::<u8>().take(len).collect::<Vec<_>>()
}

#[cfg(not(feature = "cargo-miri"))]
quickcheck! {
    fn check_count_correct(x: (Vec<u8>, u8)) -> bool {
        let (haystack, needle) = x;
        count(&haystack, needle) == naive_count(&haystack, needle)
    }
}

#[test]
fn check_count_large() {
    #[cfg(not(feature = "cargo-miri"))]
    let haystack = vec![0u8; 10_000_000];
    #[cfg(feature = "cargo-miri")]
    let haystack = vec![0u8; 1_000];
    assert_eq!(naive_count(&haystack, 0), count(&haystack, 0));
    assert_eq!(naive_count(&haystack, 1), count(&haystack, 1));
}

#[test]
fn check_count_large_rand() {
    #[cfg(not(feature = "cargo-miri"))]
    let haystack = random_bytes(100_000);
    #[cfg(feature = "cargo-miri")]
    let haystack = random_bytes(1_000);
    for i in (0..255).chain(iter::once(255)) {
        assert_eq!(naive_count(&haystack, i), count(&haystack, i));
    }
}

#[test]
fn check_count_some() {
    let haystack = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68];
    let needle = 68;
    assert_eq!(count(&haystack, needle), naive_count(&haystack, needle));
}

#[test]
fn check_count_overflow() {
    let haystack = vec![0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let needle = 2;
    assert_eq!(count(&haystack, needle), naive_count(&haystack, needle));
}

#[cfg(not(feature = "cargo-miri"))]
quickcheck! {
    fn check_num_chars_correct(haystack: Vec<u8>) -> bool {
        num_chars(&haystack) == naive_num_chars(&haystack)
    }
}

#[test]
fn check_num_chars_large() {
    #[cfg(not(feature = "cargo-miri"))]
    let haystack = vec![0u8; 10_000_000];
    #[cfg(feature = "cargo-miri")]
    let haystack = vec![0u8; 1_000];
    assert_eq!(naive_num_chars(&haystack), num_chars(&haystack));
    assert_eq!(naive_num_chars(&haystack), num_chars(&haystack));
}

#[test]
fn check_num_chars_some() {
    let haystack = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68];
    assert_eq!(num_chars(&haystack), naive_num_chars(&haystack));
}

#[test]
fn check_num_chars_overflow() {
    let haystack = vec![0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(num_chars(&haystack), naive_num_chars(&haystack));
}
