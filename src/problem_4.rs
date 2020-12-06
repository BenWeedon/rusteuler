//! > A palindromic number reads the same both ways. The largest palindrome
//! made from the product of two 2-digit numbers is 9009 = 91 × 99.
//! >
//! > Find the largest palindrome made from the product of two 3-digit numbers.
//!
//! This solution iterates over all pairs of three digit numbers. For each
//! pair, it multiplies them and checks if the product is a palindrome using
//! [`util::is_palindromic`](crate::util::is_palindromic). We return the
//! largest one.
//!
//! ```
//! # rusteuler::framework::run_solution(4, || {
//! use rusteuler::util;
//!
//! let mut largest = 0;
//! for x in 100..1000 {
//!     for y in 100..1000 {
//!         let product = x * y;
//!         if product > largest && util::is_palindromic(product) {
//!             largest = product;
//!         }
//!     }
//! }
//! assert_eq!(largest, 906609);
//! # Ok(())
//! # }).unwrap();
//! ```
//!
//! ====TIME====
