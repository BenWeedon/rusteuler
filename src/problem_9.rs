//! > A Pythagorean triplet is a set of three natural numbers, `a < b < c`, for
//! which,
//! >
//! > <code>a<sup>2</sup> + b<sup>2</sup> = c<sup>2</sup></code>
//! >
//! > For example, <code>3<sup>2</sup> + 4<sup>2</sup> = 9 + 16 = 25 = 5<sup>2</sup></code>.
//! >
//! > There exists exactly one Pythagorean triplet for which `a + b + c =
//! 1000`. Find the product `abc`.
//!
//! We iterate over all values of <var>a</var>, <var>b</var>, and <var>c</var>
//! less than 1000. Once we find a combination where both <code>a<sup>2</sup> + b<sup>2</sup> = c<sup>2</sup></code>
//! and <code>a + b + c = 1000</code>, we break out of all the loops with the
//! solution.
//!
//! This solution is pretty inefficient since it needs to iterate over so many
//! numbers.
//!
//! ```
//! # rusteuler::framework::run_solution(9, || {
//! const N: u64 = 1000;
//!
//! let mut solution = 0;
//! 'outer: for a in 0..N {
//!     for b in a + 1..N {
//!         let c_squared = a.pow(2) + b.pow(2);
//!         for c in b + 1..N {
//!             if c.pow(2) == c_squared && a + b + c == N {
//!                 solution = a * b * c;
//!                 break 'outer;
//!             }
//!         }
//!     }
//! }
//!
//! assert_eq!(solution, 31875000);
//! # Ok(())
//! # }).unwrap();
//! ```
//!
//! ====TIME====
