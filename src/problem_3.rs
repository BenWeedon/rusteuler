//! > The prime factors of 13195 are 5, 7, 13 and 29.
//! >
//! > What is the largest prime factor of the number 600851475143 ?
//!
//! For this solution, we find all the factors of 600851475143. We iterate up
//! until its square root, and for each number test both it and its other
//! associated factor. We test the factors by checking if they're prime using
//! [`util::is_prime`](crate::util::is_prime) and then checking if they're
//! larger than our current largest factor.
//!
//! This is based off one of the solutions at
//! https://www.mathblog.dk/project-euler-problem-3/.
//!
//! ```
//! # rusteuler::framework::run_solution(3, || {
//! use rusteuler::util;
//!
//! const N: u64 = 600851475143;
//! let n_sqrt = (N as f64).sqrt() as u64;
//! let mut solution = 0;
//!
//! for i in 2..n_sqrt {
//!     if N % i == 0 {
//!         let lower_divisor = i;
//!         let higher_divisor = N / i;
//!
//!         if util::is_prime(higher_divisor) && higher_divisor > solution {
//!             solution = higher_divisor;
//!         } else if util::is_prime(lower_divisor) && lower_divisor > solution {
//!             solution = lower_divisor;
//!         }
//!     }
//! }
//!
//! assert_eq!(solution, 6857);
//! # Ok(())
//! # }).unwrap();
//! ```
//!
//! ====TIME====
