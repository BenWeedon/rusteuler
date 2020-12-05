//! > The prime factors of 13195 are 5, 7, 13 and 29.
//! >
//! > What is the largest prime factor of the number 600851475143 ?
//!
//! This solution is currently very inefficient. It iterates through every
//! factor of 600851475143 and checks if each one is prime.
//! [`util::is_prime`](crate::util::is_prime) is also iterative, making this
//! algorithm extremely slow.
//!
//! ```
//! # rusteuler::framework::run_solution(|| {
//! use rusteuler::util;
//!
//! const N: u64 = 600851475143;
//! let mut solution = 0;
//! for n in 1..N / 2 {
//!     if N % n == 0 {
//!         let factor = N / n;
//!         if util::is_prime(factor) {
//!             solution = factor;
//!             break;
//!         }
//!     }
//! }
//! assert_eq!(solution, 6857);
//! # Ok(3)
//! # }).unwrap();
//! ```
//!
//! ====TIME====
