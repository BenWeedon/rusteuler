//! > A palindromic number reads the same both ways. The largest palindrome
//! made from the product of two 2-digit numbers is 9009 = 91 × 99.
//! >
//! > Find the largest palindrome made from the product of two 3-digit numbers.
//!
//! This solution iterates over all pairs of three digit numbers. For each
//! pair, it multiplies them and checks if the product is a palindrome using
//! [`util::is_palindromic`]. We return the largest one.
//!
//! ```
//! # let solution = rusteuler::problem_4::run().unwrap();
//! assert_eq!(solution, 906609);
//! ```

use crate::util;

#[doc(hidden)]
pub fn run() -> Result<u64, String> {
    let mut largest = 0;
    for x in 100..1000 {
        for y in 100..1000 {
            let product = x * y;
            if product > largest && util::is_palindromic(product) {
                largest = product;
            }
        }
    }
    Ok(largest)
}
