//! > 2520 is the smallest number that can be divided by each of the numbers
//! from 1 to 10 without any remainder.
//! >
//! > What is the smallest positive number that is evenly divisible by all of
//! the numbers from 1 to 20?
//!
//! First off, we don't need to check all the numbers from 1 to 20: we just
//! need to check the primes. This is because if the number is divisible by,
//! say 3 and 2, we know prime factorization means it must be divisible by 6.
//! We use [`util::PrimeIterTrial`](crate::util::PrimeIterTrial) to iterate
//! over the primes.
//!
//! Honestly, I don't totally remember the details of this one. I'll add those
//! when I remember.
//!
//! ```
//! # rusteuler::framework::run_solution(5, || {
//! use rusteuler::util;
//!
//! const K: u64 = 20;
//! let limit = (K as f64).sqrt();
//! let solution = util::PrimeIterTrial::new()
//!     .take_while(|p| p <= &K)
//!     .map(|p| {
//!         let a = if p as f64 <= limit {
//!             (K as f64).log(p as f64).floor() as u64
//!         } else {
//!             1
//!         };
//!         p.pow(a as u32)
//!     })
//!     .product::<u64>();
//! assert_eq!(solution, 232792560);
//! # Ok(())
//! # }).unwrap();
//! ```
//!
//! ====TIME====
