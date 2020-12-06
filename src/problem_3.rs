//! > The prime factors of 13195 are 5, 7, 13 and 29.
//! >
//! > What is the largest prime factor of the number 600851475143 ?
//!
//! For this solution, we simply keep dividing 600851475143 down until the
//! remainder is prime. At that point all attempted divisors have been less
//! than the square root of the remainder, meaning the remainder must be the
//! largest prime factor.
//!
//! This is based off one of the solutions at
//! https://www.mathblog.dk/project-euler-problem-3/.
//!
//! ```
//! # rusteuler::framework::run_solution(3, || {
//! const N: u64 = 600851475143;
//!
//! let mut remainder = N;
//! let mut factor = 2;
//! while factor * factor <= remainder {
//!     if remainder % factor == 0 {
//!         remainder = remainder / factor;
//!     } else {
//!         factor += 1;
//!     }
//! }
//!
//! assert_eq!(remainder, 6857);
//! # Ok(())
//! # }).unwrap();
//! ```
//!
//! ====TIME====
