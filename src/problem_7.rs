//! > By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//! >
//! > What is the 10 001st prime number?
//!
//! Prime generation by running [`util::is_prime`](crate::util::is_prime) on
//! every number seems to be fast enough to fairly quickly iterate over primes
//! until I reach the 10001st one. I use
//! [`util::PrimeIter`](crate::util::PrimeIter) for iteration.
//!
//! ```
//! # rusteuler::framework::run_solution(7, || {
//! use rusteuler::util;
//!
//! let solution = util::PrimeIter::new().nth(10000).unwrap();
//! assert_eq!(solution, 104743);
//! # Ok(())
//! # }).unwrap();
//! ```
//!
//! ====TIME====
