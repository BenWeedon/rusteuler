//! > The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//! >
//! > Find the sum of all the primes below two million.
//!
//! For this solution, we iterate through all primes less than two million and
//! take their sum. It uses
//! [`util::PrimeIterEratosthenes`](crate::util::PrimeIterEratosthenes), which
//! is an iterator based on the Sieve of Eratosthenes. This kind of iterator is
//! much faster than trial iteration, although it requires you to specify a
//! maximum bound.
//!
//! ```
//! # rusteuler::framework::run_solution(10, || {
//! use rusteuler::util;
//!
//! let solution = util::PrimeIterEratosthenes::new(1_999_999).sum::<u64>();
//! assert_eq!(solution, 142913828922);
//! # Ok(())
//! # }).unwrap();
//! ```
//!
//! ====TIME====
