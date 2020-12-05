//! > The prime factors of 13195 are 5, 7, 13 and 29.
//! >
//! > What is the largest prime factor of the number 600851475143 ?
//!
//! This solution is currently very inefficient. It iterates through every
//! factor of 600851475143 and checks if each one is prime. [`util::is_prime`]
//! is also iterative, making this algorithm extremely slow.
//!
//! ```
//! # let solution = rusteuler::problem_3::run().unwrap();
//! assert_eq!(solution, 6857);
//! ```

use crate::util;

#[doc(hidden)]
pub fn run() -> Result<u64, String> {
    const N: u64 = 600851475143;
    for n in 1..N / 2 {
        if N % n == 0 {
            let factor = N / n;
            if util::is_prime(factor) {
                return Ok(factor);
            }
        }
    }
    Err(format!("Largest prime factor not found for number {}.", N))
}
