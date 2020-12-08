//! > The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//! >
//! > Find the sum of all the primes below two million.
//!
//! This solution uses a fairly naive approach. We simply iterate through all
//! primes less than two million and take their sum. It uses
//! [`util::PrimeIter`](crate::util::PrimeIter), which is a trial based prime
//! iterator: it iterates through numbers, running
//! [`util::is_prime`](crate::util::is_prime) on each one.
//!
//! A more efficient solution could use an iterator based off of the Sieve of
//! Eratosthenes with a max value of 1_999_999.
//!
//! ```
//! # rusteuler::framework::run_solution(10, || {
//! use rusteuler::util;
//!
//! let solution = util::PrimeIter::new()
//!     .take_while(|p| p < &2_000_000)
//!     .sum::<u64>();
//! assert_eq!(solution, 142913828922);
//! # Ok(())
//! # }).unwrap();
//! ```
//!
//! ====TIME====
